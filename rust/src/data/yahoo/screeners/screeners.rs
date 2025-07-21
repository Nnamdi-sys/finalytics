use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;
use strum::{EnumIter, EnumString, Display, AsRefStr, IntoStaticStr, VariantNames, EnumProperty};
use crate::prelude::{Exchange, FundFamily, PeerGroup, Region, Sector, Industry, FundCategory};

const SCREENER_JSON: &str = include_str!("screeners.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub name: String,
    pub description: String,
    pub data_type: String,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenerData {
    pub equity: HashMap<String, FieldMetadata>,
    pub mutualfund: HashMap<String, FieldMetadata>,
    pub etf: HashMap<String, FieldMetadata>,
    pub index: HashMap<String, FieldMetadata>,
    pub future: HashMap<String, FieldMetadata>,
    pub crypto: HashMap<String, FieldMetadata>,
}

static SCREENER_DATA: Lazy<ScreenerData> = Lazy::new(|| {
    serde_json::from_str(SCREENER_JSON).expect("Failed to parse future screener JSON")
});


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum EquityScreener {
    #[strum(serialize = "region")]
    Region,

    #[strum(serialize = "ticker")]
    Symbol,

    #[strum(serialize = "intradayprice")]
    PriceIntraday,

    #[strum(serialize = "eodprice")]
    PriceEndOfDay,

    #[strum(serialize = "dayvolume")]
    Volume,

    #[strum(serialize = "eodvolume")]
    VolumeEndOfDay,

    #[strum(serialize = "avgdailyvol3m")]
    AvgVol3Month,

    #[strum(serialize = "intradaymarketcap")]
    MarketCapIntraday,

    #[strum(serialize = "sector")]
    Sector,

    #[strum(serialize = "industry")]
    Industry,

    #[strum(serialize = "beta")]
    Beta5YMonthly,

    #[strum(serialize = "exchange")]
    Exchange,

    #[strum(serialize = "indexmembership")]
    Index,

    #[strum(serialize = "days_to_cover_short.value")]
    ShortInterestRatio,

    #[strum(serialize = "short_interest.value")]
    ShortInterest,

    #[strum(serialize = "short_interest_percentage_change.value")]
    ShortInterestPercentChange,

    #[strum(serialize = "short_percentage_of_float.value")]
    ShortPercentOfFloat,

    #[strum(serialize = "short_percentage_of_shares_outstanding.value")]
    ShortPercentOfSharesOutstanding,

    #[strum(serialize = "peer_group")]
    PeerGroup,

    #[strum(serialize = "returnonequity.lasttwelvemonths")]
    ReturnOnEquity,

    #[strum(serialize = "epsgrowth.lasttwelvemonths")]
    Yr1PercentChangeInEPSBasic,

    #[strum(serialize = "totaldebtequity.lasttwelvemonths")]
    DebtEquity,

    #[strum(serialize = "currentratio.lasttwelvemonths")]
    CurrentRatio,

    #[strum(serialize = "grossprofitmargin.lasttwelvemonths")]
    GrossProfitMargin,

    #[strum(serialize = "returnonassets.lasttwelvemonths")]
    ReturnOnAssets,

    #[strum(serialize = "ltdebtequity.lasttwelvemonths")]
    LongTermDebtEquity,

    #[strum(serialize = "returnontotalcapital.lasttwelvemonths")]
    ReturnOnInvestedCapital,

    #[strum(serialize = "netincomemargin.lasttwelvemonths")]
    NetIncomeMargin,

    #[strum(serialize = "altmanzscoreusingtheaveragestockinformationforaperiod.lasttwelvemonths")]
    AltmanZScore,

    #[strum(serialize = "dividendyield")]
    DividendYield,

    #[strum(serialize = "quickratio.lasttwelvemonths")]
    QuickRatio,

    #[strum(serialize = "totaldebtebitda.lasttwelvemonths")]
    TotalDebtEBITDA,

    #[strum(serialize = "ebitdamargin.lasttwelvemonths")]
    EBITDAMargin,

    #[strum(serialize = "netdebtebitda.lasttwelvemonths")]
    NetDebtEBITDA,

    #[strum(serialize = "operatingcashflowtocurrentliabilities.lasttwelvemonths")]
    OperatingCashFlowRatio,

    #[strum(serialize = "forward_dividend_yield")]
    ForwardDividendYield,

    #[strum(serialize = "ebitdainterestexpense.lasttwelvemonths")]
    EBITDAInterestExpenseLTM,

    #[strum(serialize = "ebitinterestexpense.lasttwelvemonths")]
    EBITInterestExpenseLTM,

    #[strum(serialize = "fiftytwowkpercentchange")]
    Week52PricePercentChange,

    #[strum(serialize = "intradaypricechange")]
    PriceChangeIntraday,

    #[strum(serialize = "lastclose52weekhigh.lasttwelvemonths")]
    Week52PriceHighLastClose,

    #[strum(serialize = "lastclose52weeklow.lasttwelvemonths")]
    Week52PriceLowLastClose,

    #[strum(serialize = "percentchange")]
    PercentChangeInPriceIntraday,

    #[strum(serialize = "consecutive_years_of_dividend_growth_count")]
    ConsecutiveYearsOfDividendGrowthCount,

    #[strum(serialize = "price_signal_fifty_two_wk_high.datetime")]
    RecentHighTimeframe,

    #[strum(serialize = "price_signal_fifty_two_wk_low.datetime")]
    RecentLowTimeframe,

    #[strum(serialize = "totalrevenues1yrgrowth.lasttwelvemonths")]
    Yr1PercentChangeInTotalRevenue,

    #[strum(serialize = "netincome1yrgrowth.lasttwelvemonths")]
    Yr1PercentChangeInNetIncome,

    #[strum(serialize = "basicepscontinuingoperations.lasttwelvemonths")]
    EPSBasicContinuingOperations,

    #[strum(serialize = "quarterlyrevenuegrowth.quarterly")]
    QuarterlyRevenueGrowthYOY,

    #[strum(serialize = "totalrevenues.lasttwelvemonths")]
    TotalRevenue,

    #[strum(serialize = "netepsbasic.lasttwelvemonths")]
    EPSBasic,

    #[strum(serialize = "ebitda1yrgrowth.lasttwelvemonths")]
    Yr1PercentChangeInEBITDA,

    #[strum(serialize = "dilutedeps1yrgrowth.lasttwelvemonths")]
    Yr1PercentChangeInEPSDiluted,

    #[strum(serialize = "netepsdiluted.lasttwelvemonths")]
    EPSDiluted,

    #[strum(serialize = "netincomeis.lasttwelvemonths")]
    NetIncome,

    #[strum(serialize = "operatingincome.lasttwelvemonths")]
    OperatingIncome,

    #[strum(serialize = "grossprofit.lasttwelvemonths")]
    GrossProfit,

    #[strum(serialize = "ebitda.lasttwelvemonths")]
    EBITDA,

    #[strum(serialize = "dilutedepscontinuingoperations.lasttwelvemonths")]
    EPSDilutedContinuingOperations,

    #[strum(serialize = "ebit.lasttwelvemonths")]
    EBIT,

    #[strum(serialize = "bookvalueshare.lasttwelvemonths")]
    BookValuePerShare,

    #[strum(serialize = "totalsharesoutstanding")]
    TotalSharesOutstanding,

    #[strum(serialize = "totaldebt.lasttwelvemonths")]
    TotalDebt,

    #[strum(serialize = "totalassets.lasttwelvemonths")]
    TotalAssets,

    #[strum(serialize = "totalcashandshortterminvestments.lasttwelvemonths")]
    TotalCashAndShortTermInvestments,

    #[strum(serialize = "totalcurrentassets.lasttwelvemonths")]
    TotalCurrentAssets,

    #[strum(serialize = "totalequity.lasttwelvemonths")]
    TotalEquity,

    #[strum(serialize = "totalcommonsharesoutstanding.lasttwelvemonths")]
    TotalCommonSharesOutstanding,

    #[strum(serialize = "totalcurrentliabilities.lasttwelvemonths")]
    TotalCurrentLiabilities,

    #[strum(serialize = "totalcommonequity.lasttwelvemonths")]
    TotalCommonEquity,

    #[strum(serialize = "cashfromoperations1yrgrowth.lasttwelvemonths")]
    Yr1PercentChangeInCashFromOperations,

    #[strum(serialize = "unleveredfreecashflow.lasttwelvemonths")]
    UnleveredFreeCashFlow,

    #[strum(serialize = "leveredfreecashflow.lasttwelvemonths")]
    LeveredFreeCashFlow,

    #[strum(serialize = "leveredfreecashflow1yrgrowth.lasttwelvemonths")]
    Yr1PercentChangeInLeveredFreeCashFlow,

    #[strum(serialize = "dividendpershare.lasttwelvemonths")]
    DividendPerShare,

    #[strum(serialize = "capitalexpenditure.lasttwelvemonths")]
    CapitalExpenditure,

    #[strum(serialize = "forward_dividend_per_share")]
    ForwardDividendPerShare,

    #[strum(serialize = "cashfromoperations.lasttwelvemonths")]
    CashFromOperations,

    #[strum(serialize = "lastclosetevebitda.lasttwelvemonths")]
    TotalEnterpriseValueEBITDA,

    #[strum(serialize = "lastclosepricetangiblebookvalue.lasttwelvemonths")]
    PriceTangibleBookValue,

    #[strum(serialize = "lastclosetevebit.lasttwelvemonths")]
    TotalEnterpriseValueEBIT,

    #[strum(serialize = "lastclosemarketcaptotalrevenue.lasttwelvemonths")]
    PriceSales,

    #[strum(serialize = "lastclosetevtotalrevenue.lasttwelvemonths")]
    TotalEnterpriseValueTotalRevenue,

    #[strum(serialize = "lastclosepricebookvalue.lasttwelvemonths")]
    PriceBookValue,

    #[strum(serialize = "lastclosepriceearnings.lasttwelvemonths")]
    PriceEarnings,

    #[strum(serialize = "pegratio_5y")]
    PriceEarningsToGrowth,

    #[strum(serialize = "peratio.lasttwelvemonths")]
    TrailingPE,

    #[strum(serialize = "pricebookratio.quarterly")]
    PBMostRecentQuarterToMRQ,

    #[strum(serialize = "environmental_score")]
    EnvironmentalScore,

    #[strum(serialize = "esg_score")]
    ESGScore,

    #[strum(serialize = "governance_score")]
    GovernanceScore,

    #[strum(serialize = "social_score")]
    SocialScore,

    #[strum(serialize = "highest_controversy")]
    HighestControversy,

    #[strum(serialize = "pctheldinsider")]
    PercentOfSharesOutstandingHeldByInsiders,

    #[strum(serialize = "pctheldinst")]
    PercentOfSharesOutstandingHeldByInstitutions,
}

impl EquityScreener {
    /// Returns reference to the entire equity screener metadata HashMap
    pub fn metrics() -> &'static HashMap<String, FieldMetadata> {
        &SCREENER_DATA.equity
    }

    /// Gets the full metadata for this variant
    pub fn metadata(&self) -> &'static FieldMetadata {
        &SCREENER_DATA.equity[&self.to_string()]
    }

    /// Gets the display name for this variant
    pub fn name(&self) -> &'static str {
        &self.metadata().name
    }

    /// Gets the description for this variant
    pub fn description(&self) -> &'static str {
        &self.metadata().description
    }

    /// Gets the data type for this variant
    pub fn data_type(&self) -> &'static str {
        &self.metadata().data_type
    }

    /// Gets the unit for this variant
    pub fn unit(&self) -> &'static str {
        &self.metadata().unit
    }

    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            EquityScreener::Exchange => validate_enum_value::<Exchange>("Exchange", value),
            EquityScreener::Region => validate_enum_value::<Region>("Region", value),
            EquityScreener::Sector => validate_enum_value::<Sector>("Sector", value),
            EquityScreener::Industry => validate_enum_value::<Industry>("Industry", value),
            EquityScreener::PeerGroup => validate_enum_value::<PeerGroup>("PeerGroup", value),
            _ => Ok(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum MutualFundScreener {
    #[strum(serialize = "categoryname")]
    FundsByCategory,

    #[strum(serialize = "fundfamilyname")]
    FundsByCompany,

    #[strum(serialize = "fundnetassets")]
    FundNetAssets,

    #[strum(serialize = "eodprice")]
    PriceEndOfDay,

    #[strum(serialize = "intradayprice")]
    PriceIntraday,

    #[strum(serialize = "intradaypricechange")]
    Change,

    #[strum(serialize = "percentchange")]
    PercentChange,

    #[strum(serialize = "region")]
    Region,

    #[strum(serialize = "ticker")]
    Symbol,

    #[strum(serialize = "exchange")]
    Exchange,

    #[strum(serialize = "primary_sector")]
    Sector,

    #[strum(serialize = "initialinvestment")]
    InitialInvestment,

    #[strum(serialize = "marketcapitalvaluelong")]
    MarketCapitalValueLong,

    #[strum(serialize = "fiftytwowkpercentchange")]
    Week52PricePercentChange,

    #[strum(serialize = "annualreturnnavy1")]
    AnnualReturnNAVYear1,

    #[strum(serialize = "annualreturnnavy3")]
    AnnualReturnNAVYear3,

    #[strum(serialize = "annualreturnnavy5")]
    AnnualReturnNAVYear5,

    #[strum(serialize = "annualreturnnavy1categoryrank")]
    AnnualReturnNAVYear1CategoryRank,

    #[strum(serialize = "quarterendtrailingreturnytd")]
    QuarterEndTrailingReturnYTD,

    #[strum(serialize = "trailing_3m_return")]
    Trailing3MReturn,

    #[strum(serialize = "trailing_ytd_return")]
    TrailingYTDReturn,

    #[strum(serialize = "annualreportnetexpenseratio")]
    AnnualReportNetExpenseRatio,

    #[strum(serialize = "turnoverratio")]
    TurnoverRatio,

    #[strum(serialize = "annualreportgrossexpenseratio")]
    AnnualReportGrossExpenseRatio,

    #[strum(serialize = "performanceratingoverall")]
    MorningstarPerformanceRatingOverall,

    #[strum(serialize = "riskratingoverall")]
    MorningstarRiskRatingOverall,

    #[strum(serialize = "twohundreddaymovingavg")]
    Day200MovingAverage,

    #[strum(serialize = "fiftydaymovingavg")]
    Day50MovingAverage,
}

impl MutualFundScreener {
    /// Returns reference to the entire mutual fund screener metadata HashMap
    pub fn metrics() -> &'static HashMap<String, FieldMetadata> {
        &SCREENER_DATA.mutualfund
    }

    /// Gets the full metadata for this variant
    pub fn metadata(&self) -> &'static FieldMetadata {
        &SCREENER_DATA.mutualfund[&self.to_string()]
    }

    /// Gets the display name for this variant
    pub fn name(&self) -> &'static str {
        &self.metadata().name
    }

    /// Gets the description for this variant
    pub fn description(&self) -> &'static str {
        &self.metadata().description
    }

    /// Gets the data type for this variant
    pub fn data_type(&self) -> &'static str {
        &self.metadata().data_type
    }

    /// Gets the unit for this variant
    pub fn unit(&self) -> &'static str {
        &self.metadata().unit
    }

    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            MutualFundScreener::Exchange => validate_enum_value::<Exchange>("Exchange", value),
            MutualFundScreener::Region => validate_enum_value::<Region>("Region", value),
            MutualFundScreener::FundsByCompany => validate_enum_value::<FundFamily>("FundFamily", value),
            MutualFundScreener::FundsByCategory => validate_enum_value::<FundCategory>("FundCategory", value),
            _ => Ok(value.to_string()),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum EtfScreener {
    #[strum(serialize = "categoryname")]
    FundsByCategory,

    #[strum(serialize = "fundfamilyname")]
    FundsByCompany,

    #[strum(serialize = "fundnetassets")]
    FundNetAssets,

    #[strum(serialize = "eodprice")]
    PriceEndOfDay,

    #[strum(serialize = "intradayprice")]
    PriceIntraday,

    #[strum(serialize = "intradaypricechange")]
    Change,

    #[strum(serialize = "percentchange")]
    PercentChange,

    #[strum(serialize = "region")]
    Region,

    #[strum(serialize = "ticker")]
    Symbol,

    #[strum(serialize = "exchange")]
    Exchange,

    #[strum(serialize = "primary_sector")]
    Sector,

    #[strum(serialize = "initialinvestment")]
    InitialInvestment,

    #[strum(serialize = "marketcapitalvaluelong")]
    MarketCapitalValueLong,

    #[strum(serialize = "dayvolume")]
    Volume,

    #[strum(serialize = "eodvolume")]
    VolumeEndOfDay,

    #[strum(serialize = "avgdailyvol3m")]
    AvgVol3Month,

    #[strum(serialize = "fiftytwowkpercentchange")]
    Week52PricePercentChange,

    #[strum(serialize = "annualreturnnavy1")]
    AnnualReturnNAVYear1,

    #[strum(serialize = "annualreturnnavy3")]
    AnnualReturnNAVYear3,

    #[strum(serialize = "annualreturnnavy5")]
    AnnualReturnNAVYear5,

    #[strum(serialize = "annualreturnnavy1categoryrank")]
    AnnualReturnNAVYear1CategoryRank,

    #[strum(serialize = "trailing_3m_return")]
    Trailing3MReturn,

    #[strum(serialize = "quarterendtrailingreturnytd")]
    QuarterEndTrailingReturnYTD,

    #[strum(serialize = "trailing_ytd_return")]
    TrailingYTDReturn,

    #[strum(serialize = "annualreportgrossexpenseratio")]
    AnnualReportGrossExpenseRatio,

    #[strum(serialize = "annualreportnetexpenseratio")]
    AnnualReportNetExpenseRatio,

    #[strum(serialize = "turnoverratio")]
    TurnoverRatio,

    #[strum(serialize = "performanceratingoverall")]
    MorningstarPerformanceRatingOverall,

    #[strum(serialize = "riskratingoverall")]
    MorningstarRiskRatingOverall,
}

impl EtfScreener {
    /// Returns reference to the entire etf screener metadata HashMap
    pub fn metrics() -> &'static HashMap<String, FieldMetadata> {
        &SCREENER_DATA.etf
    }

    /// Gets the full metadata for this variant
    pub fn metadata(&self) -> &'static FieldMetadata {
        &SCREENER_DATA.etf[&self.to_string()]
    }

    /// Gets the display name for this variant
    pub fn name(&self) -> &'static str {
        &self.metadata().name
    }

    /// Gets the description for this variant
    pub fn description(&self) -> &'static str {
        &self.metadata().description
    }

    /// Gets the data type for this variant
    pub fn data_type(&self) -> &'static str {
        &self.metadata().data_type
    }

    /// Gets the unit for this variant
    pub fn unit(&self) -> &'static str {
        &self.metadata().unit
    }

    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            EtfScreener::Exchange => validate_enum_value::<Exchange>("Exchange", value),
            EtfScreener::Region => validate_enum_value::<Region>("Region", value),
            EtfScreener::FundsByCompany => validate_enum_value::<FundFamily>("FundFamily", value),
            EtfScreener::FundsByCategory => validate_enum_value::<FundCategory>("FundCategory", value),
            _ => Ok(value.to_string()),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum IndexScreener {
    #[strum(serialize = "eodprice")]
    PriceEndOfDay,

    #[strum(serialize = "intradayprice")]
    PriceIntraday,

    #[strum(serialize = "intradaypricechange")]
    Change,

    #[strum(serialize = "percentchange")]
    PercentChange,

    #[strum(serialize = "eodvolume")]
    VolumeEndOfDay,

    #[strum(serialize = "region")]
    Region,

    #[strum(serialize = "ticker")]
    Symbol,

    #[strum(serialize = "exchange")]
    Exchange,

    #[strum(serialize = "sector")]
    Sector,

    #[strum(serialize = "industry")]
    Industry,

    #[strum(serialize = "fiftytwowkpercentchange")]
    Week52PricePercentChange,

    #[strum(serialize = "avgdailyvol3m")]
    AvgVol3Month,
}

impl IndexScreener {
    /// Returns reference to the entire index screener metadata HashMap
    pub fn metrics() -> &'static HashMap<String, FieldMetadata> {
        &SCREENER_DATA.index
    }

    /// Gets the full metadata for this variant
    pub fn metadata(&self) -> &'static FieldMetadata {
        &SCREENER_DATA.index[&self.to_string()]
    }

    /// Gets the display name for this variant
    pub fn name(&self) -> &'static str {
        &self.metadata().name
    }

    /// Gets the description for this variant
    pub fn description(&self) -> &'static str {
        &self.metadata().description
    }

    /// Gets the data type for this variant
    pub fn data_type(&self) -> &'static str {
        &self.metadata().data_type
    }

    /// Gets the unit for this variant
    pub fn unit(&self) -> &'static str {
        &self.metadata().unit
    }

    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            IndexScreener::Exchange => validate_enum_value::<Exchange>("Exchange", value),
            IndexScreener::Region => validate_enum_value::<Region>("Region", value),
            IndexScreener::Sector => validate_enum_value::<Sector>("Sector", value),
            IndexScreener::Industry => validate_enum_value::<Industry>("Industry", value),
            _ => Ok(value.to_string()),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum FutureScreener {
    #[strum(serialize = "eodprice")]
    PriceEndOfDay,

    #[strum(serialize = "intradayprice")]
    PriceIntraday,

    #[strum(serialize = "intradaypricechange")]
    Change,

    #[strum(serialize = "percentchange")]
    PercentChange,

    #[strum(serialize = "dayvolume")]
    Volume,

    #[strum(serialize = "ticker")]
    Symbol,

    #[strum(serialize = "open_interest")]
    OpenInterest,

    #[strum(serialize = "region")]
    Region,

    #[strum(serialize = "exchange")]
    Exchange,

    #[strum(serialize = "fiftytwowkpercentchange")]
    Week52PricePercentChange,

    #[strum(serialize = "avgdailyvol3m")]
    AvgVol3Month,
}

impl FutureScreener {
    /// Returns reference to the entire future screener metadata HashMap
    pub fn metrics() -> &'static HashMap<String, FieldMetadata> {
        &SCREENER_DATA.future
    }

    /// Gets the full metadata for this variant
    pub fn metadata(&self) -> &'static FieldMetadata {
        &SCREENER_DATA.future[&self.to_string()]
    }

    /// Gets the display name for this variant
    pub fn name(&self) -> &'static str {
        &self.metadata().name
    }

    /// Gets the description for this variant
    pub fn description(&self) -> &'static str {
        &self.metadata().description
    }

    /// Gets the data type for this variant
    pub fn data_type(&self) -> &'static str {
        &self.metadata().data_type
    }

    /// Gets the unit for this variant
    pub fn unit(&self) -> &'static str {
        &self.metadata().unit
    }

    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            FutureScreener::Exchange => validate_enum_value::<Exchange>("Exchange", value),
            FutureScreener::Region => validate_enum_value::<Region>("Region", value),
            _ => Ok(value.to_string()),
        }
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum CryptoScreener {
    #[strum(serialize = "percentchange")]
    PercentChange,

    #[strum(serialize = "dayvolume")]
    Volume,

    #[strum(serialize = "ticker")]
    Symbol,

    #[strum(serialize = "currency")]
    Currency,

    #[strum(serialize = "intradaymarketcap")]
    MarketCapIntraday,

    #[strum(serialize = "exchange")]
    Exchange,

    #[strum(serialize = "fiftytwowkpercentchange")]
    Week52PricePercentChange,

    #[strum(serialize = "avgdailyvol3m")]
    AvgVol3Month,
}

impl CryptoScreener {
    /// Returns reference to the entire crypto screener metadata HashMap
    pub fn metrics() -> &'static HashMap<String, FieldMetadata> {
        &SCREENER_DATA.crypto
    }

    /// Gets the full metadata for this variant
    pub fn metadata(&self) -> &'static FieldMetadata {
        &SCREENER_DATA.crypto[&self.to_string()]
    }

    /// Gets the display name for this variant
    pub fn name(&self) -> &'static str {
        &self.metadata().name
    }

    /// Gets the description for this variant
    pub fn description(&self) -> &'static str {
        &self.metadata().description
    }

    /// Gets the data type for this variant
    pub fn data_type(&self) -> &'static str {
        &self.metadata().data_type
    }

    /// Gets the unit for this variant
    pub fn unit(&self) -> &'static str {
        &self.metadata().unit
    }

    pub fn validate_value(&self, value: &str) -> Result<String, String> {
        match self {
            CryptoScreener::Exchange => {
                if value == "CCC" {
                    Ok(value.to_string())
                } else {
                    Err("For CryptoScreener::Exchange only 'CCC' is allowed.".to_string())
                }
            }
            _ => Ok(value.to_string()),
        }
    }
}


fn validate_enum_value<T>(field_name: &str, value: &str) -> Result<String, String>
where
    T: FromStr + VariantNames,
{
    if T::from_str(value).is_ok() {
        Ok(value.to_string())
    } else {
        let allowed = T::VARIANTS.join(", ");
        Err(format!("Invalid value '{value}' for {field_name}. Allowed values: [{allowed}]"))
    }
}