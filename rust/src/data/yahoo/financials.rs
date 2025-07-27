use std::error::Error;
use serde_json::Value;
use polars::prelude::*;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use crate::data::yahoo::web::get_json_response;
use crate::utils::date_utils::{convert_to_quarter, convert_to_year};
use crate::data::yahoo::config::{Fundamentals, FundamentalsResponse, Object, StatementFrequency, StatementType};

pub async fn get_fundamentals(
    symbol: &str,
    statement_type: StatementType,
    frequency: StatementFrequency
) -> Result<DataFrame, Box<dyn Error>> {
    let period1 = (Utc::now() - Duration::days(365 * 5)).timestamp();
    let period2 = Utc::now().timestamp();
    let _type = match statement_type {
        StatementType::IncomeStatement => Fundamentals.get_income_statement_items(frequency),
        StatementType::BalanceSheet => Fundamentals.get_balance_sheet_items(frequency),
        StatementType::CashFlowStatement => Fundamentals.get_cash_flow_items(frequency),
        _ => unimplemented!("Statement Type Not Supported for get_fundamentals"),
    };
    let _type_clone = _type.clone();
    let url = format!("https://query2.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/\
    timeseries/{symbol}?symbol={symbol}&type={_type}&period1={period1}&period2={period2}");
    let result = get_json_response(url).await?;
    let data: FundamentalsResponse = serde_json::from_value(result)
        .map_err(|e| format!("Failed to deserialize into FundamentalsResponse: {e}"))?;
    let mut columns: Vec<Column> = vec![];
    let mut temp_items: HashMap<String, Value> = HashMap::new();
    let mut init = 0;
    for item in &data.timeseries.result{
        // convert to polars dataframe
        for (key, value) in item {
            if _type_clone.contains(key.as_str()){
                let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                    .map_err(|e| format!("Failed to deserialize into Object: {e}"))?;
                let date_vec = items.iter().map(|x| x.asOfDate.clone()).collect::<Vec<String>>();
                if date_vec.len() < 4 {
                    temp_items.insert(key.clone(), value.clone());
                    break;
                }
                if init == 0 {
                    let date_series = Column::new("asOfDate".into(), &date_vec);
                    columns.push(date_series);
                    init += 1;
                }

                if items.len() == columns[0].len(){
                    let vars_vec = items.iter().map(|x| x.reportedValue.raw).collect::<Vec<f64>>();
                    let vars_series = Column::new(key.as_str().replace(&frequency.to_string(), "").into(), &vars_vec);
                    columns.push(vars_series);
                }
                else {
                    let mut vars_vec: Vec<f64> = vec![];
                    for d in columns[0].as_series().unwrap().iter(){
                        let mut found = false;
                        for item in items.iter() {
                            if item.asOfDate == d.to_string(){
                                vars_vec.push(item.reportedValue.raw);
                                found = true;
                                break;
                            }
                        }
                        if !found{
                            vars_vec.push(0.0);
                        }
                    }
                    let vars_series = Column::new(key.as_str().replace(&frequency.to_string(), "").into(), &vars_vec);
                    columns.push(vars_series);
                }

            }
        }
    }

    if !temp_items.is_empty() {
        for (key, value) in temp_items {
            let items: Vec<Object> = serde_json::from_value(value.to_string().parse()?)
                .map_err(|e| format!("Failed to deserialize into Object: {e}"))?;
            let mut vars_vec: Vec<f64> = vec![];
            for d in columns[0].as_series().unwrap().iter(){
                let mut found = false;
                for item in items.iter() {
                    if format!("\"{}\"", item.asOfDate) == d.to_string(){
                        vars_vec.push(item.reportedValue.raw);
                        found = true;
                        break;
                    }
                }
                if !found{
                    vars_vec.push(0.0);
                }
            }
            let vars_series = Column::new(key.as_str().replace(&frequency.to_string(), "").into(), &vars_vec);
            columns.push(vars_series);
        }
    }
    let df = DataFrame::new(columns)?;
    Ok(df)
}

pub async fn financial_ratios(symbol: &str, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>>{
    let income_statement = get_fundamentals(symbol, StatementType::IncomeStatement, frequency).await?;
    let balance_sheet = get_fundamentals(symbol, StatementType::BalanceSheet, frequency).await?;
    let cash_flow = get_fundamentals(symbol, StatementType::CashFlowStatement, frequency).await?;
    let zero_series = Column::new("zero".into(), vec![0.0; income_statement.height()]);
    let ratios = vec![
        income_statement.column("asOfDate")?.clone().with_name("date".into()),
        (income_statement.column("GrossProfit")? / income_statement.column("TotalRevenue")?)?.with_name("Gross Profit Margin".into()),
        (income_statement.column("EBIT")? / income_statement.column("TotalRevenue")?)?.with_name("Operating Profit Margin".into()),
        (income_statement.column("NetIncome")? / income_statement.column("TotalRevenue")?)?.with_name("Net Profit Margin".into()),
        (income_statement.column("NetIncome")? / balance_sheet.column("TotalAssets")?)?.with_name("Return on Assets".into()),
        (income_statement.column("NetIncome")? / balance_sheet.column("TotalEquityGrossMinorityInterest")?)?.with_name("Return on Equity".into()),
        (balance_sheet.column("CurrentAssets")? / balance_sheet.column("CurrentLiabilities")?)?.with_name("Quick Ratio".into()),
        (balance_sheet.column("CurrentAssets")? / balance_sheet.column("CurrentLiabilities")?)?.with_name("Current Ratio".into()),
        (balance_sheet.column("TotalLiabilitiesNetMinorityInterest")? / balance_sheet.column("TotalEquityGrossMinorityInterest")?)?.with_name("Debt to Equity".into()),
        (balance_sheet.column("TotalLiabilitiesNetMinorityInterest")? / balance_sheet.column("TotalAssets")?)?.with_name("Debt to Assets".into()),
        (income_statement.column("EBIT")? / income_statement.column("InterestExpense").unwrap_or(&zero_series))?.with_name("Interest Coverage".into()),
        (income_statement.column("TotalRevenue")? / balance_sheet.column("TotalAssets")?)?.with_name("Asset Turnover".into()),
        (income_statement.column("CostOfRevenue")? / balance_sheet.column("Inventory")?)?.with_name("Inventory Turnover".into()),
        ((balance_sheet.column("AccountsReceivable")? / income_statement.column("TotalRevenue")?)? * 365.0).with_name("Days Receivable".into()),
        ((balance_sheet.column("Inventory")? / income_statement.column("CostOfRevenue")?)? * 365.0).with_name("Days Inventory".into()),
        ((balance_sheet.column("AccountsPayable")? / income_statement.column("CostOfRevenue")?)? * 365.0).with_name("Days Payable".into()),
        income_statement.column("DilutedEPS")?.clone().with_name("Earnings per Share".into()),
        (balance_sheet.column("TotalCapitalization")? / income_statement.column("NetIncome")?)?.with_name("Price to Earnings".into()),
        (balance_sheet.column("TotalCapitalization")? / balance_sheet.column("TotalEquityGrossMinorityInterest")?)?.with_name("Price to Book".into()),
        (balance_sheet.column("TotalCapitalization")? / income_statement.column("TotalRevenue")?)?.with_name("Price to Sales".into()),
        (balance_sheet.column("TotalCapitalization")? / cash_flow.column("OperatingCashFlow")?)?.with_name("Price to Cashflow".into()),
        (balance_sheet.column("TotalCapitalization")? / cash_flow.column("FreeCashFlow")?)?.with_name("Price to Free Cashflow".into()),
    ];

    let df = DataFrame::new(ratios)?;

    // Transpose the DataFrame
    let dates = df.column("date")?.str()?.into_no_null_iter()
        .collect::<Vec<&str>>();
    let dates= match frequency {
        StatementFrequency::Quarterly => convert_to_quarter(dates),
        StatementFrequency::Annual => convert_to_year(dates),
    };
    let mut df = df.drop("date").unwrap();
    let items = Series::new("Items".into(), df.get_column_names_str());
    let mut transposed_df = df.transpose(None, None)?;
    transposed_df.set_column_names(&dates)?;
    let _ = transposed_df.insert_column(0, items)?;
    Ok(transposed_df)
}

pub async fn income_statement(symbol: &str, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
    let income_statement = get_fundamentals(symbol, StatementType::IncomeStatement, frequency).await?;

    let mut ifrs_mapping = HashMap::new();
    ifrs_mapping.insert("TotalRevenue", "Revenue");
    ifrs_mapping.insert("GrossProfit", "Gross Profit");
    ifrs_mapping.insert("OperatingExpense", "Operating Expenses");
    ifrs_mapping.insert("NetIncome", "Net Income");
    ifrs_mapping.insert("TaxProvision", "Income Tax Expense");
    ifrs_mapping.insert("BasicEPS", "Earnings per Share - Basic");
    ifrs_mapping.insert("DilutedEPS", "Earnings per Share - Diluted");
    ifrs_mapping.insert("EBIT", "Operating Profit (EBIT)");
    ifrs_mapping.insert("EBITDA", "EBITDA");
    ifrs_mapping.insert("ReconciledCostOfRevenue", "Cost of Goods Sold");
    ifrs_mapping.insert("ReconciledDepreciation", "Depreciation and Amortization");
    ifrs_mapping.insert("InterestIncome", "Interest Income");
    ifrs_mapping.insert("InterestExpense", "Interest Expense");

    let mut cols = vec![
        "TotalRevenue", "ReconciledCostOfRevenue", "GrossProfit", "OperatingExpense",
        "EBITDA", "ReconciledDepreciation", "EBIT", "InterestIncome", "InterestExpense",
        "TaxProvision", "NetIncome", "BasicEPS", "DilutedEPS",
    ];

    // remove item from cols if it doesn't exist in the income statement dataframe
    cols.retain(|x| income_statement.column(x).is_ok());

    let df = income_statement.clone().select(cols.clone())?;

    // Create a vector of expressions for renaming and selecting columns
    let expressions: Vec<Expr> = cols
        .iter()
        .map(|old_name| {
            let new_name = ifrs_mapping.get(*old_name).unwrap_or(old_name);
            col(*old_name).alias(*new_name)
        })
        .collect();

    // Select and alias columns using the expressions
    let mut renamed_df = df.lazy().select(expressions).collect()?;

    // Transpose the DataFrame
    let dates = income_statement.column("asOfDate")?.str()?.into_no_null_iter()
        .collect::<Vec<&str>>();
    let dates= match frequency {
        StatementFrequency::Quarterly => convert_to_quarter(dates),
        StatementFrequency::Annual => convert_to_year(dates),
    };
    let items = Series::new("Items".into(), renamed_df.get_column_names_str());
    let mut transposed_df = renamed_df.transpose(None, None)?;
    transposed_df.set_column_names(&dates)?;
    let _ = transposed_df.insert_column(0, items)?;
    Ok(transposed_df)
}

/// Formats the balance sheet
///
/// # Returns
///
/// * `DataFrame` - Formatted balance sheet
pub async fn balance_sheet(symbol: &str, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
    let balance_sheet = get_fundamentals(symbol, StatementType::BalanceSheet, frequency).await?;

    let mut ifrs_mapping = HashMap::new();

    // Assets
    ifrs_mapping.insert("CashAndCashEquivalents", "Cash and Cash Equivalents");
    ifrs_mapping.insert("AccountsReceivable", "Accounts Receivable");
    ifrs_mapping.insert("Inventory", "Inventories");
    ifrs_mapping.insert("OtherCurrentAssets", "Other Current Assets");
    ifrs_mapping.insert("CurrentAssets", "Total Current Assets");
    ifrs_mapping.insert("NetPPE", "Property, Plant, and Equipment (Net)");
    ifrs_mapping.insert("GoodwillAndOtherIntangibleAssets", "Intangible Assets");
    ifrs_mapping.insert("OtherNonCurrentAssets", "Other Non-Current Assets");
    ifrs_mapping.insert("TotalNonCurrentAssets", "Total Non-Current Assets");
    ifrs_mapping.insert("TotalAssets", "Total Assets");

    // Liabilities
    ifrs_mapping.insert("AccountsPayable", "Accounts Payable");
    ifrs_mapping.insert("CurrentDebt", "Short-Term Debt");
    ifrs_mapping.insert("OtherCurrentLiabilities", "Other Current Liabilities");
    ifrs_mapping.insert("CurrentLiabilities", "Total Current Liabilities");
    ifrs_mapping.insert("LongTermDebt", "Long-Term Debt");
    ifrs_mapping.insert("OtherNonCurrentLiabilities", "Other Non-Current Liabilities");
    ifrs_mapping.insert("TotalNonCurrentLiabilitiesNetMinorityInterest", "Total Non-Current Liabilities");
    ifrs_mapping.insert("TotalLiabilitiesNetMinorityInterest", "Total Liabilities");

    // Equity
    ifrs_mapping.insert("CommonStock", "Common Stock");
    ifrs_mapping.insert("RetainedEarnings", "Retained Earnings");
    ifrs_mapping.insert("CommonStockEquity", "Total Equity");
    ifrs_mapping.insert("TotalEquityGrossMinorityInterest", "Total Liabilities and Equity");

    let mut cols = vec![
        "CashAndCashEquivalents", "AccountsReceivable", "Inventory", "OtherCurrentAssets",
        "CurrentAssets", "NetPPE", "GoodwillAndOtherIntangibleAssets",
        "OtherNonCurrentAssets", "TotalNonCurrentAssets", "TotalAssets",
        "AccountsPayable", "CurrentDebt", "OtherCurrentLiabilities", "CurrentLiabilities", "LongTermDebt",
        "OtherNonCurrentLiabilities", "TotalNonCurrentLiabilitiesNetMinorityInterest", "TotalLiabilitiesNetMinorityInterest",
        "CommonStock", "RetainedEarnings", "CommonStockEquity", "TotalEquityGrossMinorityInterest",
    ];

    // remove item from cols if it doesn't exist in the balance sheet dataframe
    cols.retain(|x| balance_sheet.column(x).is_ok());


    let df = balance_sheet.clone().select(cols.clone())?;

    // Create a vector of expressions for renaming and selecting columns
    let expressions: Vec<Expr> = cols
        .iter()
        .map(|old_name| {
            let new_name = ifrs_mapping.get(*old_name).unwrap_or(old_name);
            col(*old_name).alias(*new_name)
        })
        .collect();

    // Select and alias columns using the expressions
    let mut renamed_df = df.lazy().select(expressions).collect()?;

    // Transpose the DataFrame
    let dates = balance_sheet.column("asOfDate")?.str()?.into_no_null_iter()
        .collect::<Vec<&str>>();
    let dates= match frequency {
        StatementFrequency::Quarterly => convert_to_quarter(dates),
        StatementFrequency::Annual => convert_to_year(dates),
    };
    let items = Column::new("Items".into(), renamed_df.get_column_names_str());
    let mut transposed_df = renamed_df.transpose(None, None)?;
    transposed_df.set_column_names(&dates)?;
    let _ = transposed_df.insert_column(0, items)?;

    Ok(transposed_df)

}

/// Formats the cash flow statement
///
/// # Returns
///
/// * `DataFrame` - Formatted cash flow statement
pub async fn cashflow_statement(symbol: &str, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
    let cash_flow = get_fundamentals(symbol, StatementType::CashFlowStatement, frequency).await?;

    let mut ifrs_mapping = HashMap::new();

    // Operating Activities
    ifrs_mapping.insert("NetIncomeFromContinuingOperations", "Net Income from Continuing Operations");
    ifrs_mapping.insert("DepreciationAmortizationDepletion", "Depreciation, Amortization, and Depletion");
    ifrs_mapping.insert("StockBasedCompensation", "Stock-Based Compensation");
    ifrs_mapping.insert("DeferredIncomeTax", "Deferred Income Tax");
    ifrs_mapping.insert("ChangeInWorkingCapital", "Changes in Working Capital");
    ifrs_mapping.insert("CashFlowFromContinuingOperatingActivities", "Cash Flow from Continuing Operating Activities");

    // Investing Activities
    ifrs_mapping.insert("PurchaseOfPPE", "Purchase of Property, Plant, and Equipment");
    ifrs_mapping.insert("PurchaseOfBusiness", "Purchase of Business");
    ifrs_mapping.insert("NetInvestmentPurchaseAndSale", "Net Investment Purchase and Sale");
    ifrs_mapping.insert("SaleOfInvestment", "Sale of Investment");
    ifrs_mapping.insert("PurchaseOfInvestment", "Purchase of Investment");
    ifrs_mapping.insert("NetOtherInvestingChanges", "Net Other Investing Changes");
    ifrs_mapping.insert("CashFlowFromContinuingInvestingActivities", "Cash Flow from Continuing Investing Activities");

    // Financing Activities
    ifrs_mapping.insert("IssuanceOfCapitalStock", "Issuance of Capital Stock");
    ifrs_mapping.insert("CommonStockIssuance", "Common Stock Issuance");
    ifrs_mapping.insert("NetCommonStockIssuance", "Net Common Stock Issuance");
    ifrs_mapping.insert("CommonStockDividendPaid", "Common Stock Dividend Paid");
    ifrs_mapping.insert("CashDividendsPaid", "Cash Dividends Paid");
    ifrs_mapping.insert("RepurchaseOfCapitalStock", "Repurchase of Capital Stock");
    ifrs_mapping.insert("LongTermDebtPayments", "Long-Term Debt Payments");
    ifrs_mapping.insert("NetLongTermDebtIssuance", "Net Long-Term Debt Issuance");
    ifrs_mapping.insert("NetIssuancePaymentsOfDebt", "Net Issuance/Payments of Debt");
    ifrs_mapping.insert("NetOtherFinancingCharges", "Net Other Financing Charges");
    ifrs_mapping.insert("CashFlowFromContinuingFinancingActivities", "Cash Flow from Continuing Financing Activities");

    // Summary
    ifrs_mapping.insert("BeginningCashPosition", "Beginning Cash Position");
    ifrs_mapping.insert("OperatingCashFlow", "Operating Cash Flow");
    ifrs_mapping.insert("InvestingCashFlow", "Investing Cash Flow");
    ifrs_mapping.insert("FinancingCashFlow", "Financing Cash Flow");
    ifrs_mapping.insert("EndCashPosition", "Ending Cash Position");
    ifrs_mapping.insert("FreeCashFlow", "Free Cash Flow");


    let mut cols = vec![
        "NetIncomeFromContinuingOperations", "DepreciationAmortizationDepletion", "StockBasedCompensation",
        "DeferredIncomeTax", "ChangeInWorkingCapital", "CashFlowFromContinuingOperatingActivities",
        "PurchaseOfPPE", "PurchaseOfBusiness", "NetInvestmentPurchaseAndSale", "SaleOfInvestment", "PurchaseOfInvestment",
        "NetOtherInvestingChanges", "CashFlowFromContinuingInvestingActivities", "IssuanceOfCapitalStock",
        "CommonStockIssuance", "NetCommonStockIssuance", "CommonStockDividendPaid", "CashDividendsPaid",
        "RepurchaseOfCapitalStock", "LongTermDebtPayments", "NetLongTermDebtIssuance", "NetIssuancePaymentsOfDebt",
        "NetOtherFinancingCharges", "CashFlowFromContinuingFinancingActivities", "EffectOfExchangeRateChanges",
        "BeginningCashPosition", "OperatingCashFlow", "InvestingCashFlow", "FinancingCashFlow", "EndCashPosition", "FreeCashFlow",
    ];

    // remove item from cols if it doesn't exist in the cash flow dataframe
    cols.retain(|x| cash_flow.column(x).is_ok());


    let df = cash_flow.clone().select(cols.clone())?;

    // Create a vector of expressions for renaming and selecting columns
    let expressions: Vec<Expr> = cols
        .iter()
        .map(|old_name| {
            let new_name = ifrs_mapping.get(*old_name).unwrap_or(old_name);
            col(*old_name).alias(*new_name)
        })
        .collect();

    // Select and alias columns using the expressions
    let mut renamed_df = df.lazy().select(expressions).collect()?;

    // Transpose the DataFrame
    let dates = cash_flow.column("asOfDate")?.str()?.into_no_null_iter()
        .collect::<Vec<&str>>();
    let dates= match frequency {
        StatementFrequency::Quarterly => convert_to_quarter(dates),
        StatementFrequency::Annual => convert_to_year(dates),
    };
    let items = Column::new("Items".into(), renamed_df.get_column_names_str());
    let mut transposed_df = renamed_df.transpose(None, None)?;
    transposed_df.set_column_names(&dates)?;
    let _ = transposed_df.insert_column(0, items)?;

    Ok(transposed_df)
}
