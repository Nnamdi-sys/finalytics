use std::collections::HashMap;
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub symbol: String,
    #[serde(rename = "longName")]
    pub name: String,
    #[serde(rename = "quoteType")]
    pub asset_class: String,
    #[serde(rename = "fullExchangeName")]
    pub exchange: String,
    #[serde(default)]
    #[serde(rename = "financialCurrency")]
    pub currency: String,
    #[serde(rename = "regularMarketTime")]
    pub timestamp: i64,
    #[serde(rename = "regularMarketPrice")]
    pub price: f64,
    #[serde(rename = "regularMarketOpen")]
    pub open: f64,
    #[serde(rename = "regularMarketDayHigh")]
    pub high: f64,
    #[serde(rename = "regularMarketDayLow")]
    pub low: f64,
    #[serde(rename = "regularMarketPreviousClose")]
    pub close : f64,
    #[serde(rename = "regularMarketVolume")]
    pub volume: f64,
    pub bid : f64,
    pub ask : f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerSummaryStats {
    pub symbol: String,
    #[serde(default)]
    pub long_name: String,
    #[serde(default)]
    pub full_exchange_name: String,
    pub currency: String,
    pub regular_market_time : i64,
    pub regular_market_price: f64,
    pub regular_market_change_percent: f64,
    pub regular_market_volume: f64,
    pub regular_market_open: f64,
    pub regular_market_day_high: f64,
    pub regular_market_day_low: f64,
    pub regular_market_previous_close: f64,
    pub fifty_two_week_high: f64,
    pub fifty_two_week_low: f64,
    #[serde(default)]
    pub fifty_two_week_change_percent: f64,
    pub fifty_day_average: f64,
    pub two_hundred_day_average: f64,
    #[serde(default)]
    #[serde(rename = "epsTrailingTwelveMonths")]
    pub trailing_eps: f64,
    #[serde(default)]
    #[serde(rename = "epsCurrentYear")]
    pub current_eps: f64,
    #[serde(default)]
    pub eps_forward: f64,
    #[serde(default)]
    #[serde(rename = "trailingPE")]
    pub trailing_pe: f64,
    #[serde(default)]
    #[serde(rename = "priceEpsCurrentYear")]
    pub current_pe: f64,
    #[serde(default)]
    #[serde(rename = "forwardPE")]
    pub forward_pe: f64,
    #[serde(default)]
    pub dividend_rate: f64,
    #[serde(default)]
    pub dividend_yield: f64,
    #[serde(default)]
    pub book_value: f64,
    #[serde(default)]
    pub price_to_book: f64,
    #[serde(default)]
    pub market_cap: f64,
    #[serde(default)]
    pub shares_outstanding: f64,
    #[serde(default)]
    pub average_analyst_rating: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct OptionContract {
    pub contractSymbol: String,
    pub strike: f64,
    pub currency: String,
    pub lastPrice: f64,
    #[serde(default)]
    pub change: f64,
    #[serde(default)]
    pub percentChange: f64,
    #[serde(default)]
    pub openInterest: f64,
    #[serde(default)]
    pub bid: f64,
    #[serde(default)]
    pub ask: f64,
    pub contractSize: String,
    pub expiration: i64,
    pub lastTradeDate: i64,
    pub impliedVolatility: f64,
    pub inTheMoney: bool,
}

#[derive(Debug)]
pub struct Options {
    pub ticker_price: f64,
    pub expiration_dates: Vec<String>,
    pub ttms: Vec<f64>,
    pub strikes: Vec<f64>,
    pub chain: DataFrame
}

#[derive(Debug, Deserialize)]
pub struct FundamentalsResponse {
    pub timeseries: TimeSeries,
}

#[derive(Debug, Deserialize)]
pub struct TimeSeries {
    pub result: Vec<HashMap<String, Value>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Object {
    pub asOfDate: String,
    pub reportedValue: Figure,
}

#[derive(Debug, Deserialize)]
pub struct Figure {
    pub raw: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Interval {
    TwoMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    SixtyMinutes,
    NinetyMinutes,
    OneHour,
    OneDay,
    FiveDays,
    OneWeek,
    OneMonth,
    ThreeMonths,
}

impl Interval {
    pub fn to_string(&self) -> String {
        match self {
            Interval::TwoMinutes => "2m".to_string(),
            Interval::FiveMinutes => "5m".to_string(),
            Interval::FifteenMinutes => "15m".to_string(),
            Interval::ThirtyMinutes => "30m".to_string(),
            Interval::SixtyMinutes => "60m".to_string(),
            Interval::NinetyMinutes => "90m".to_string(),
            Interval::OneHour => "1h".to_string(),
            Interval::OneDay => "1d".to_string(),
            Interval::FiveDays => "5d".to_string(),
            Interval::OneWeek => "1wk".to_string(),
            Interval::OneMonth => "1mo".to_string(),
            Interval::ThreeMonths => "3mo".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Interval {
        match s {
            "2m" => Interval::TwoMinutes,
            "5m" => Interval::FiveMinutes,
            "15m" => Interval::FifteenMinutes,
            "30m" => Interval::ThirtyMinutes,
            "60m" => Interval::SixtyMinutes,
            "90m" => Interval::NinetyMinutes,
            "1h" => Interval::OneHour,
            "1d" => Interval::OneDay,
            "5d" => Interval::FiveDays,
            "1wk" => Interval::OneWeek,
            "1mo" => Interval::OneMonth,
            "3mo" => Interval::ThreeMonths,
            _ => Interval::OneDay,
        }
    }

    pub fn to_days(&self) -> f64 {
        match self {
            Interval::TwoMinutes => 2.0 / 24.0 * 60.0,
            Interval::FiveMinutes => 5.0 / 24.0 * 60.0,
            Interval::FifteenMinutes => 15.0 / 24.0 * 60.0,
            Interval::ThirtyMinutes => 30.0 / 24.0 * 60.0,
            Interval::SixtyMinutes => 60.0 / 24.0 * 60.0,
            Interval::OneHour => 60.0 / 24.0 * 60.0,
            Interval::NinetyMinutes => 90.0 / 24.0 * 60.0,
            Interval::OneDay => 1.0,
            Interval::FiveDays => 5.0,
            Interval::OneWeek => 5.0,
            Interval::OneMonth => 20.0,
            Interval::ThreeMonths => 60.0,
        }
    }
}

pub struct Fundamentals;

impl Fundamentals {
    pub fn get_income_statement_items(&self, frequency: &str) -> String {
        let income_vec =  vec![
            "TotalRevenue", "ExciseTaxes", "OperatingRevenue", "GrossProfit", "CostOfRevenue",
            "SalariesAndWages", "RentAndLandingFees", "InsuranceAndClaims", "OtherGandA",
            "GeneralAndAdministrativeExpense", "SellingAndMarketingExpense", "SellingGeneralAndAdministration",
            "ResearchAndDevelopment", "DepreciationIncomeStatement", "AmortizationOfIntangiblesIncomeStatement",
            "DepreciationAndAmortizationInIncomeStatement", "Amortization", "DepreciationAmortizationDepletionIncomeStatement",
            "DepletionIncomeStatement", "ProvisionForDoubtfulAccounts", "OtherTaxes", "OtherOperatingExpenses",
            "OperatingExpense", "InterestIncomeNonOperating", "InterestExpenseNonOperating", "InterestExpense",
            "NetNonOperatingInterestIncomeExpense", "GainOnSaleOfSecurity", "EarningsFromEquityInterest",
            "SecuritiesAmortization", "RestructuringAndMergernAcquisition", "ImpairmentOfCapitalAssets",
            "WriteOff", "OtherSpecialCharges", "GainOnSaleOfBusiness", "GainOnSaleOfPPE", "SpecialIncomeCharges",
            "OtherNonOperatingIncomeExpenses", "OtherIncomeExpense", "PretaxIncome", "TaxProvision",
            "EarningsFromEquityInterestNetOfTax", "NetIncomeContinuousOperations", "NetIncomeDiscontinuousOperations",
            "NetIncomeExtraordinary", "NetIncomeFromTaxLossCarryforward", "NetIncomeIncludingNoncontrollingInterests",
            "MinorityInterests", "NetIncome", "PreferredStockDividends", "OtherunderPreferredStockDividend",
            "NetIncomeCommonStockholders", "AverageDilutionEarnings", "DilutedNIAvailtoComStockholders",
            "BasicContinuousOperations", "BasicDiscontinuousOperations", "BasicExtraordinary", "BasicAccountingChange",
            "TaxLossCarryforwardBasicEPS", "BasicEPSOtherGainsLosses", "BasicEPS", "DilutedContinuousOperations",
            "DilutedDiscontinuousOperations", "DilutedExtraordinary", "DilutedAccountingChange",
            "TaxLossCarryforwardDilutedEPS", "DilutedEPSOtherGainsLosses", "DilutedEPS", "BasicAverageShares",
            "DilutedAverageShares", "DividendPerShare", "TotalOperatingIncomeAsReported", "ReportedNormalizedBasicEPS",
            "ReportedNormalizedDilutedEPS", "RentExpenseSupplemental", "TotalExpenses",
            "NetIncomeFromContinuingAndDiscontinuedOperation", "NormalizedIncome", "ContinuingAndDiscontinuedBasicEPS",
            "ContinuingAndDiscontinuedDilutedEPS", "InterestIncome", "InterestExpense", "NetInterestIncome", "EBIT", "EBITDA",
            "ReconciledCostOfRevenue", "ReconciledDepreciation", "NetIncomeFromContinuingOperationNetMinorityInterest",
            "TotalUnusualItemsExcludingGoodwill", "TotalUnusualItems", "NormalizedBasicEPS", "NormalizedDilutedEPS",
            "NormalizedEBITDA", "TaxRateForCalcs", "TaxEffectOfUnusualItems",
        ];
        let result = match frequency {
            "annual" => {
                let out_str = income_vec.iter().map(|x| format!("{}{}", "annual", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
            "quarterly" => {
                let out_str = income_vec.iter().map(|x| format!("{}{}", "quarterly", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
            _ => {
                let out_str = income_vec.iter().map(|x| format!("{}{}", "annual", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
        };
        result
    }

    pub fn get_balance_sheet_items(&self, frequency: &str) -> String {
        let balance_vec = vec![
            "TreasurySharesNumber", "PreferredSharesNumber", "OrdinarySharesNumber", "ShareIssued", "NetDebt",
            "TotalDebt", "TangibleBookValue", "InvestedCapital", "WorkingCapital", "NetTangibleAssets",
            "CapitalLeaseObligations", "CommonStockEquity", "PreferredStockEquity", "TotalCapitalization",
            "TotalEquityGrossMinorityInterest", "MinorityInterest", "StockholdersEquity",
            "OtherEquityInterest", "GainsLossesNotAffectingRetainedEarnings", "OtherEquityAdjustments",
            "FixedAssetsRevaluationReserve", "ForeignCurrencyTranslationAdjustments",
            "MinimumPensionLiabilities", "UnrealizedGainLoss", "TreasuryStock", "RetainedEarnings",
            "AdditionalPaidInCapital", "CapitalStock", "OtherCapitalStock", "CommonStock", "PreferredStock",
            "TotalPartnershipCapital", "GeneralPartnershipCapital", "LimitedPartnershipCapital",
            "TotalLiabilitiesNetMinorityInterest", "TotalNonCurrentLiabilitiesNetMinorityInterest",
            "OtherNonCurrentLiabilities", "LiabilitiesHeldforSaleNonCurrent", "RestrictedCommonStock",
            "PreferredSecuritiesOutsideStockEquity", "DerivativeProductLiabilities", "EmployeeBenefits",
            "NonCurrentPensionAndOtherPostretirementBenefitPlans", "NonCurrentAccruedExpenses",
            "DuetoRelatedPartiesNonCurrent", "TradeandOtherPayablesNonCurrent",
            "NonCurrentDeferredLiabilities", "NonCurrentDeferredRevenue",
            "NonCurrentDeferredTaxesLiabilities", "LongTermDebtAndCapitalLeaseObligation",
            "LongTermCapitalLeaseObligation", "LongTermDebt", "LongTermProvisions", "CurrentLiabilities",
            "OtherCurrentLiabilities", "CurrentDeferredLiabilities", "CurrentDeferredRevenue",
            "CurrentDeferredTaxesLiabilities", "CurrentDebtAndCapitalLeaseObligation",
            "CurrentCapitalLeaseObligation", "CurrentDebt", "OtherCurrentBorrowings", "LineOfCredit",
            "CommercialPaper", "CurrentNotesPayable", "PensionandOtherPostRetirementBenefitPlansCurrent",
            "CurrentProvisions", "PayablesAndAccruedExpenses", "CurrentAccruedExpenses", "InterestPayable",
            "Payables", "OtherPayable", "DuetoRelatedPartiesCurrent", "DividendsPayable", "TotalTaxPayable",
            "IncomeTaxPayable", "AccountsPayable", "TotalAssets", "TotalNonCurrentAssets",
            "OtherNonCurrentAssets", "DefinedPensionBenefit", "NonCurrentPrepaidAssets",
            "NonCurrentDeferredAssets", "NonCurrentDeferredTaxesAssets", "DuefromRelatedPartiesNonCurrent",
            "NonCurrentNoteReceivables", "NonCurrentAccountsReceivable", "FinancialAssets",
            "InvestmentsAndAdvances", "OtherInvestments", "InvestmentinFinancialAssets",
            "HeldToMaturitySecurities", "AvailableForSaleSecurities",
            "FinancialAssetsDesignatedasFairValueThroughProfitorLossTotal", "TradingSecurities",
            "LongTermEquityInvestment", "InvestmentsinJointVenturesatCost",
            "InvestmentsInOtherVenturesUnderEquityMethod", "InvestmentsinAssociatesatCost",
            "InvestmentsinSubsidiariesatCost", "InvestmentProperties", "GoodwillAndOtherIntangibleAssets",
            "OtherIntangibleAssets", "Goodwill", "NetPPE", "AccumulatedDepreciation", "GrossPPE", "Leases",
            "ConstructionInProgress", "OtherProperties", "MachineryFurnitureEquipment",
            "BuildingsAndImprovements", "LandAndImprovements", "Properties", "CurrentAssets",
            "OtherCurrentAssets", "HedgingAssetsCurrent", "AssetsHeldForSaleCurrent", "CurrentDeferredAssets",
            "CurrentDeferredTaxesAssets", "RestrictedCash", "PrepaidAssets", "Inventory",
            "InventoriesAdjustmentsAllowances", "OtherInventories", "FinishedGoods", "WorkInProcess",
            "RawMaterials", "Receivables", "ReceivablesAdjustmentsAllowances", "OtherReceivables",
            "DuefromRelatedPartiesCurrent", "TaxesReceivable", "AccruedInterestReceivable", "NotesReceivable",
            "LoansReceivable", "AccountsReceivable", "AllowanceForDoubtfulAccountsReceivable",
            "GrossAccountsReceivable", "CashCashEquivalentsAndShortTermInvestments",
            "OtherShortTermInvestments", "CashAndCashEquivalents", "CashEquivalents", "CashFinancial",
        ];
        let result = match frequency {
            "annual" => {
                let out_str = balance_vec.iter().map(|x| format!("{}{}", "annual", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
            "quarterly" => {
                let out_str = balance_vec.iter().map(|x| format!("{}{}", "quarterly", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
            _ => {
                let out_str = balance_vec.iter().map(|x| format!("{}{}", "annual", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
        };
        result
    }

    pub fn get_cash_flow_items(&self, frequency: &str) -> String {
        let cash_vec = vec![
            "ForeignSales", "DomesticSales", "AdjustedGeographySegmentData", "FreeCashFlow",
            "RepurchaseOfCapitalStock", "RepaymentOfDebt", "IssuanceOfDebt", "IssuanceOfCapitalStock",
            "CapitalExpenditure", "InterestPaidSupplementalData", "IncomeTaxPaidSupplementalData",
            "EndCashPosition", "OtherCashAdjustmentOutsideChangeinCash", "BeginningCashPosition",
            "EffectOfExchangeRateChanges", "ChangesInCash", "OtherCashAdjustmentInsideChangeinCash",
            "CashFlowFromDiscontinuedOperation", "FinancingCashFlow", "CashFromDiscontinuedFinancingActivities",
            "CashFlowFromContinuingFinancingActivities", "NetOtherFinancingCharges", "InterestPaidCFF",
            "ProceedsFromStockOptionExercised", "CashDividendsPaid", "PreferredStockDividendPaid",
            "CommonStockDividendPaid", "NetPreferredStockIssuance", "PreferredStockPayments",
            "PreferredStockIssuance", "NetCommonStockIssuance", "CommonStockPayments", "CommonStockIssuance",
            "NetIssuancePaymentsOfDebt", "NetShortTermDebtIssuance", "ShortTermDebtPayments",
            "ShortTermDebtIssuance", "NetLongTermDebtIssuance", "LongTermDebtPayments", "LongTermDebtIssuance",
            "InvestingCashFlow", "CashFromDiscontinuedInvestingActivities",
            "CashFlowFromContinuingInvestingActivities", "NetOtherInvestingChanges", "InterestReceivedCFI",
            "DividendsReceivedCFI", "NetInvestmentPurchaseAndSale", "SaleOfInvestment", "PurchaseOfInvestment",
            "NetInvestmentPropertiesPurchaseAndSale", "SaleOfInvestmentProperties",
            "PurchaseOfInvestmentProperties", "NetBusinessPurchaseAndSale", "SaleOfBusiness",
            "PurchaseOfBusiness", "NetIntangiblesPurchaseAndSale", "SaleOfIntangibles", "PurchaseOfIntangibles",
            "NetPPEPurchaseAndSale", "SaleOfPPE", "PurchaseOfPPE", "CapitalExpenditureReported",
            "OperatingCashFlow", "CashFromDiscontinuedOperatingActivities",
            "CashFlowFromContinuingOperatingActivities", "TaxesRefundPaid", "InterestReceivedCFO",
            "InterestPaidCFO", "DividendReceivedCFO", "DividendPaidCFO", "ChangeInWorkingCapital",
            "ChangeInOtherWorkingCapital", "ChangeInOtherCurrentLiabilities", "ChangeInOtherCurrentAssets",
            "ChangeInPayablesAndAccruedExpense", "ChangeInAccruedExpense", "ChangeInInterestPayable",
            "ChangeInPayable", "ChangeInDividendPayable", "ChangeInAccountPayable", "ChangeInTaxPayable",
            "ChangeInIncomeTaxPayable", "ChangeInPrepaidAssets", "ChangeInInventory", "ChangeInReceivables",
            "ChangesInAccountReceivables", "OtherNonCashItems", "ExcessTaxBenefitFromStockBasedCompensation",
            "StockBasedCompensation", "UnrealizedGainLossOnInvestmentSecurities", "ProvisionandWriteOffofAssets",
            "AssetImpairmentCharge", "AmortizationOfSecurities", "DeferredTax", "DeferredIncomeTax",
            "DepreciationAmortizationDepletion", "Depletion", "DepreciationAndAmortization",
            "AmortizationCashFlow", "AmortizationOfIntangibles", "Depreciation", "OperatingGainsLosses",
            "PensionAndEmployeeBenefitExpense", "EarningsLossesFromEquityInvestments",
            "GainLossOnInvestmentSecurities", "NetForeignCurrencyExchangeGainLoss", "GainLossOnSaleOfPPE",
            "GainLossOnSaleOfBusiness", "NetIncomeFromContinuingOperations",
            "CashFlowsfromusedinOperatingActivitiesDirect", "TaxesRefundPaidDirect", "InterestReceivedDirect",
            "InterestPaidDirect", "DividendsReceivedDirect", "DividendsPaidDirect", "ClassesofCashPayments",
            "OtherCashPaymentsfromOperatingActivities", "PaymentsonBehalfofEmployees",
            "PaymentstoSuppliersforGoodsandServices", "ClassesofCashReceiptsfromOperatingActivities",
            "OtherCashReceiptsfromOperatingActivities", "ReceiptsfromGovernmentGrants", "ReceiptsfromCustomers",
        ];
        let result = match frequency {
            "annual" => {
                let out_str = cash_vec.iter().map(|x| format!("{}{}", "annual", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
            "quarterly" => {
                let out_str = cash_vec.iter().map(|x| format!("{}{}", "quarterly", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
            _ => {
                let out_str = cash_vec.iter().map(|x| format!("{}{}", "annual", x)).collect::<Vec<String>>();
                out_str.join(",")
            }
        };
        result
    }
}



