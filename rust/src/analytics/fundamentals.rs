use std::collections::HashMap;
use std::error::Error;
use polars::prelude::*;
use crate::data::ticker::TickerData;
use crate::models::ticker::Ticker;
use crate::utils::date_utils::convert_to_quarter;

pub trait Financials {
    fn financial_ratios(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn income_statement(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn balance_sheet(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn cashflow_statement(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
}

impl Financials for Ticker {
    /// Computes financial ratios
    ///
    /// # Returns
    ///
    /// * `Financials` struct
    async fn financial_ratios(&self) -> Result<DataFrame, Box<dyn Error>>{
        let income_statement = self.get_fundamentals("income-statement", "quarterly").await?;
        let balance_sheet = self.get_fundamentals("balance-sheet", "quarterly").await?;
        let cash_flow = self.get_fundamentals("cash-flow", "quarterly").await?;
        let ratios = vec![
            Series::new("date", income_statement.column("asOfDate")?),
            Series::new("Gross Profit Margin", (income_statement.column("GrossProfit")? / income_statement.column("TotalRevenue")?)?),
            Series::new("Operating Profit Margin", (income_statement.column("EBIT")? / income_statement.column("TotalRevenue")?)?),
            Series::new("Net Profit Margin", (income_statement.column("NetIncome")? / income_statement.column("TotalRevenue")?)?),
            Series::new("Return on Assets", (income_statement.column("NetIncome")? / balance_sheet.column("TotalAssets")?)?),
            Series::new("Return on Equity", (income_statement.column("NetIncome")? / balance_sheet.column("TotalEquityGrossMinorityInterest")?)?),
            Series::new("Quick Ratio", (balance_sheet.column("CurrentAssets")? / balance_sheet.column("CurrentLiabilities")?)?),
            Series::new("Current Ratio", (balance_sheet.column("CurrentAssets")? / balance_sheet.column("CurrentLiabilities")?)?),
            Series::new("Debt to Equity", (balance_sheet.column("TotalLiabilitiesNetMinorityInterest")? / balance_sheet.column("TotalEquityGrossMinorityInterest")?)?),
            Series::new("Debt to Assets", (balance_sheet.column("TotalLiabilitiesNetMinorityInterest")? / balance_sheet.column("TotalAssets")?)?),
            Series::new("Interest Coverage", (income_statement.column("EBIT")? / income_statement.column("InterestExpense")?)?),
            Series::new("Asset Turnover", (income_statement.column("TotalRevenue")? / balance_sheet.column("TotalAssets")?)?),
            Series::new("Inventory Turnover", (income_statement.column("CostOfRevenue")? / balance_sheet.column("Inventory")?)?),
            Series::new("Days Receivable", (balance_sheet.column("AccountsReceivable")? / income_statement.column("TotalRevenue")?)? * 365.0),
            Series::new("Days Inventory", (balance_sheet.column("Inventory")? / income_statement.column("CostOfRevenue")?)? * 365.0),
            Series::new("Days Payable", (balance_sheet.column("AccountsPayable")? / income_statement.column("CostOfRevenue")?)? * 365.0),
            Series::new("Earnings per Share", income_statement.column("DilutedEPS")?),
            Series::new("Price to Earnings", (balance_sheet.column("TotalCapitalization")? / income_statement.column("NetIncome")?)?),
            Series::new("Price to Book", (balance_sheet.column("TotalCapitalization")? / balance_sheet.column("TotalEquityGrossMinorityInterest")?)?),
            Series::new("Price to Sales", (balance_sheet.column("TotalCapitalization")? / income_statement.column("TotalRevenue")?)?),
            Series::new("Price to Cashflow", (balance_sheet.column("TotalCapitalization")? / cash_flow.column("OperatingCashFlow")?)?),
            Series::new("Price to Free Cashflow", (balance_sheet.column("TotalCapitalization")? / cash_flow.column("FreeCashFlow")?)?),
        ];
        
        let df = DataFrame::new(ratios)?;

        // Transpose the DataFrame
        let dates = df.column("date")?.str()?.into_no_null_iter()
            .collect::<Vec<&str>>();
        let dates= convert_to_quarter(dates);
        let mut df = df.drop("date").unwrap();
        let items = Series::new("Items", df.get_column_names());
        let mut transposed_df = df.transpose(None, None)?;
        let _ =  transposed_df.set_column_names(&dates)?;
        let _ = transposed_df.insert_column(0, items)?;
        Ok(transposed_df)
    }

    /// Formats the income statement
    ///
    /// # Returns
    ///
    /// * `DataFrame` - Formatted income statement
    async fn income_statement(&self) -> Result<DataFrame, Box<dyn Error>> {
        let income_statement = self.get_fundamentals("income-statement", "quarterly").await?;
        
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

        // remove item from cols if it doesnt exist in the income statement dataframe
        cols.retain(|x| income_statement.column(*x).is_ok());

        let df = income_statement.clone().select(&cols)?;

        // Create a vector of expressions for renaming and selecting columns
        let expressions: Vec<Expr> = cols
            .iter()
            .map(|old_name| {
                let new_name = ifrs_mapping.get(*old_name).unwrap_or(old_name);
                col(*old_name).alias(new_name)
            })
            .collect();

        // Select and alias columns using the expressions
        let mut renamed_df = df.lazy().select(expressions).collect()?;

        // Transpose the DataFrame
        let dates = income_statement.column("asOfDate")?.str()?.into_no_null_iter()
            .collect::<Vec<&str>>();
        let dates= convert_to_quarter(dates);
        let items = Series::new("Items", renamed_df.get_column_names());
        let mut transposed_df = renamed_df.transpose(None, None)?;
        let _ =  transposed_df.set_column_names(&dates)?;
        let _ = transposed_df.insert_column(0, items)?;
        Ok(transposed_df)
    }

    /// Formats the balance sheet
    ///
    /// # Returns
    ///
    /// * `DataFrame` - Formatted balance sheet
    async fn balance_sheet(&self) -> Result<DataFrame, Box<dyn Error>> {
        let balance_sheet = self.get_fundamentals("balance-sheet", "quarterly").await?;
        
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
        cols.retain(|x| balance_sheet.column(*x).is_ok());


        let df = balance_sheet.clone().select(&cols)?;

        // Create a vector of expressions for renaming and selecting columns
        let expressions: Vec<Expr> = cols
            .iter()
            .map(|old_name| {
                let new_name = ifrs_mapping.get(*old_name).unwrap_or(old_name);
                col(*old_name).alias(new_name)
            })
            .collect();

        // Select and alias columns using the expressions
        let mut renamed_df = df.lazy().select(expressions).collect()?;

        // Transpose the DataFrame
        let dates = balance_sheet.column("asOfDate")?.str()?.into_no_null_iter()
            .collect::<Vec<&str>>();
        let dates= convert_to_quarter(dates);
        let items = Series::new("Items", renamed_df.get_column_names());
        let mut transposed_df = renamed_df.transpose(None, None)?;
        let _ =  transposed_df.set_column_names(&dates)?;
        let _ = transposed_df.insert_column(0, items)?;

        Ok(transposed_df)

    }

    /// Formats the cash flow statement
    ///
    /// # Returns
    ///
    /// * `DataFrame` - Formatted cash flow statement
    async fn cashflow_statement(&self) -> Result<DataFrame, Box<dyn Error>> {
        let cash_flow = self.get_fundamentals("cash-flow", "quarterly").await?;
        
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
        cols.retain(|x| cash_flow.column(*x).is_ok());


        let df = cash_flow.clone().select(&cols)?;

        // Create a vector of expressions for renaming and selecting columns
        let expressions: Vec<Expr> = cols
            .iter()
            .map(|old_name| {
                let new_name = ifrs_mapping.get(*old_name).unwrap_or(old_name);
                col(*old_name).alias(new_name)
            })
            .collect();

        // Select and alias columns using the expressions
        let mut renamed_df = df.lazy().select(expressions).collect()?;

        // Transpose the DataFrame
        let dates = cash_flow.column("asOfDate")?.str()?.into_no_null_iter()
            .collect::<Vec<&str>>();
        let dates= convert_to_quarter(dates);
        let items = Series::new("Items", renamed_df.get_column_names());
        let mut transposed_df = renamed_df.transpose(None, None)?;
        let _ =  transposed_df.set_column_names(&dates)?;
        let _ = transposed_df.insert_column(0, items)?;

        Ok(transposed_df)
    }
}