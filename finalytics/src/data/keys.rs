use crate::data::db::{get_distinct_asset_classes, get_distinct_categories, get_distinct_exchanges};

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

pub enum AssetClass {
    Stocks,
    ETFs,
    MutualFunds,
    Indices,
    Futures,
    Currencies,
    Cryptocurrencies,
    All
}

impl AssetClass {
    pub fn to_string_vec(&self) -> Vec<String> {
        match self {
            AssetClass::Stocks => vec!["Stocks".to_string()],
            AssetClass::ETFs => vec!["ETF".to_string()],
            AssetClass::MutualFunds => vec!["Mutual Fund".to_string()],
            AssetClass::Indices => vec!["Index".to_string()],
            AssetClass::Futures => vec!["Future".to_string()],
            AssetClass::Currencies => vec!["Currency".to_string()],
            AssetClass::Cryptocurrencies => vec!["CRYPTOCURRENCY".to_string()],
            AssetClass::All => get_distinct_asset_classes().unwrap(),
        }
    }
}

pub enum Category {
    ConsumerCyclical,
    CommunicationServices,
    FinancialServices,
    RealEstate,
    BasicMaterials,
    Utilities,
    Technology,
    ConsumerDefensive,
    Healthcare,
    Energy,
    Industrials,
    NA,
    Services,
    Financial,
    IndustrialGoods,
    ConsumerGoods,
    Conglomerates,
    All
}

impl Category {
    pub fn to_string_vec(&self) -> Vec<String> {
        match self {
            Category::ConsumerCyclical => vec!["Consumer Cyclical".to_string()],
            Category::CommunicationServices => vec!["Communication Services".to_string()],
            Category::FinancialServices => vec!["Financial Services".to_string()],
            Category::RealEstate => vec!["Real Estate".to_string()],
            Category::BasicMaterials => vec!["Basic Materials".to_string()],
            Category::Utilities => vec!["Utilities".to_string()],
            Category::Technology => vec!["Technology".to_string()],
            Category::ConsumerDefensive => vec!["Consumer Defensive".to_string()],
            Category::Healthcare => vec!["Healthcare".to_string()],
            Category::Energy => vec!["Energy".to_string()],
            Category::Industrials => vec!["Industrials".to_string()],
            Category::NA => vec!["N/A".to_string()],
            Category::Services => vec!["Services".to_string()],
            Category::Financial => vec!["Financial".to_string()],
            Category::IndustrialGoods => vec!["Industrial Goods".to_string()],
            Category::ConsumerGoods => vec!["Consumer Goods".to_string()],
            Category::Conglomerates => vec!["Conglomerates".to_string()],
            Category::All => get_distinct_categories().unwrap(),
        }
    }
}


pub enum Exchange {
    NewYorkStockExchange,     // NYQ
    NASDAQ,                   // NMS
    StockholmStockExchange,   // STO
    DowJonesIndices, // DJI
    NasdaqCapitalMarket,       // NCM
    NasdaqGlobalMarket,       // NGM
    Currencies,               // CCY
    Crytpocurrencies,   // CCC
    NYSEArca,                 // PCX
    NYSEAmerican,             // NIM
    NewYorkMercantileExchange, // NYM
    COMEX,                    // CMX
    ChicagoBoardofTrade,      // CBT
    ChicagoMercantileExchange,       // CME
    PinkOpenMarket,               // PNK
    TorontoStockExchange,     // TOR
    NYSEAmericanOptions,      // ASE
    NewYorkBoardofTrade,      // NYB
    SNPIndices,                   // SNP
    WestCoastBoardofTrade,    // WCB
    BTS,                     // BTS
    CurrencyExchangeInternational,       // CXI
    NASDAQStockMarket,        // NAS
    NagoyaStockExchange,      // NSI
    LondonStockExchange,      // LSE
    Xetra,                    // GER
    BerlinStockExchange,      // BER
    DusseldorfStockExchange,  // DUS
    EuronextParis,            // PAR
    NewYorkStockExchangeARCA, // NYS
    LondonIOB,                // IOB
    SIXSwissExchange,         // ZRH
    BuenosAiresStockExchange, // BUE
    BombayStockExchange,      // BSE
    AustralianSecuritiesExchange, // ASX
    VancouverStockExchange,   // VAN
    AmsterdamStockExchange,   // AMS
    JapanExchangeGroup,       // JPX
    CanadianNationalStockExchange, // CNQ
    FrankfurtStockExchange,   // FRA
    MunichStockExchange,      // MUN
    IstanbulStockExchange,   // IST
    MexicanStockExchange,     // MEX
    MilanStockExchange,       // MIL
    NewZealandStockExchange,  // NZE
    SaoPauloStockExchange,    // SAO
    KoreaStockExchange,       // KSC
    FukuokaStockExchange,     // FGI
    HongKongStockExchange,    // HKG
    StockExchangeofThailand,  // SET
    SingaporeExchangeSecurities, // SES
    ShanghaiStockExchange,    // SHH
    SwissElectronicBourse,         // EBS
    OsloStockExchange,        // OSL
    TelAvivStockExchange,     // TLV
    KoreaExchange,            // KOE
    CopenhagenStockExchange,  // CPH
    StuttgartStockExchange,   // STU
    BursaMalaysia,            // KLS
    HamburgStockExchange,     // HAM
    ViennaStockExchange,      // VIE
    PragueStockExchange,      // PRA
    HanoiStockExchange,       // HAN
    JohannesburgStockExchange, // JNB
    CboeDXE,     // DXE
    MoscowExchange,           // MSC
    CboeAustralia,  // CXA
    ShenzhenStockExchange,    // SHZ
    VietnamStockExchange,      // VSE
    WarsawStockExchange,      // WSE
    IntercontinentalExchange,  // ICE
    RigaStockExchange,        // RIS
    ZagrebStockExchange,      // CXE
    JakartaStockExchange,     // JKT
    TaiwanOTCExchange,                     // TWO
    OsakaStockExchange,       // OSA
    AquisStockExchange,               // AQS
    TaiwanStockExchange,      // TAI
    QatarStockExchange,       // DOH
    HelsinkiStockExchange,    // HEL
    TallinnStockExchange,     // TSI
    MoldovaStockExchange,     // MCE
    NEOExchange,              // NEO
    EuronextBrussels,         // BRU
    VilniusStockExchange,     // LIT
    BudapestStockExchange,    // BUD
    EuronextLisbon,           // LIS
    SantiagoStockExchange,    // SGO
    FSI,     // FSI
    IrishStockExchange,       // ISE
    AthensStockExchange,      // ATH
    SaudiStockExchange,       // SAU
    TrinidadandTobagoStockExchange, // TLO
    CboeBXE,                  // CBO
    BVPBratislavaStockExchange, // BVC
    TAL,                     // TAL
    BoursaKuwait,             // KUW
    EgyptianExchange,         // CAI
    ColomboStockExchange,     // CSE
    DubaiFinancialMarket,     // DFM
    PhilippineStockExchange,  // PHS
    KazakhstanStockExchange,  // FKA
    OTCBulletinBoard,      // OBB
    YHD,     // YHD
    SAP,    // SAP
    CaracasStockExchange,     // CCS
    OPI,                    // OPI
    Euronext,                 // ENX
    All,
    }


impl Exchange {
    pub fn to_string_vec(&self) -> Vec<String> {
        match self {
            Exchange::NewYorkStockExchange => vec!["NYQ".to_string()],
            Exchange::NASDAQ => vec!["NMS".to_string()],
            Exchange::StockholmStockExchange => vec!["STO".to_string()],
            Exchange::DowJonesIndices => vec!["DJI".to_string()],
            Exchange::NasdaqCapitalMarket => vec!["NCM".to_string()],
            Exchange::NasdaqGlobalMarket => vec!["NGM".to_string()],
            Exchange::Currencies => vec!["CCY".to_string()],
            Exchange::Crytpocurrencies => vec!["CCC".to_string()],
            Exchange::NYSEArca => vec!["PCX".to_string()],
            Exchange::NYSEAmerican => vec!["NIM".to_string()],
            Exchange::NewYorkMercantileExchange => vec!["NYM".to_string()],
            Exchange::COMEX => vec!["CMX".to_string()],
            Exchange::ChicagoBoardofTrade => vec!["CBT".to_string()],
            Exchange::ChicagoMercantileExchange => vec!["CME".to_string()],
            Exchange::PinkOpenMarket => vec!["PNK".to_string()],
            Exchange::TorontoStockExchange => vec!["TOR".to_string()],
            Exchange::NYSEAmericanOptions => vec!["ASE".to_string()],
            Exchange::NewYorkBoardofTrade => vec!["NYB".to_string()],
            Exchange::SNPIndices => vec!["SNP".to_string()],
            Exchange::WestCoastBoardofTrade => vec!["WCB".to_string()],
            Exchange::BTS => vec!["BTS".to_string()],
            Exchange::CurrencyExchangeInternational => vec!["CXI".to_string()],
            Exchange::NASDAQStockMarket => vec!["NAS".to_string()],
            Exchange::NagoyaStockExchange => vec!["NSI".to_string()],
            Exchange::LondonStockExchange => vec!["LSE".to_string()],
            Exchange::Xetra => vec!["GER".to_string()],
            Exchange::BerlinStockExchange => vec!["BER".to_string()],
            Exchange::DusseldorfStockExchange => vec!["DUS".to_string()],
            Exchange::EuronextParis => vec!["PAR".to_string()],
            Exchange::NewYorkStockExchangeARCA => vec!["NYS".to_string()],
            Exchange::LondonIOB => vec!["IOB".to_string()],
            Exchange::SIXSwissExchange => vec!["ZRH".to_string()],
            Exchange::BuenosAiresStockExchange => vec!["BUE".to_string()],
            Exchange::BombayStockExchange => vec!["BSE".to_string()],
            Exchange::AustralianSecuritiesExchange => vec!["ASX".to_string()],
            Exchange::VancouverStockExchange => vec!["VAN".to_string()],
            Exchange::AmsterdamStockExchange => vec!["AMS".to_string()],
            Exchange::JapanExchangeGroup => vec!["JPX".to_string()],
            Exchange::CanadianNationalStockExchange => vec!["CNQ".to_string()],
            Exchange::FrankfurtStockExchange => vec!["FRA".to_string()],
            Exchange::MunichStockExchange => vec!["MUN".to_string()],
            Exchange::IstanbulStockExchange => vec!["IST".to_string()],
            Exchange::MexicanStockExchange => vec!["MEX".to_string()],
            Exchange::MilanStockExchange => vec!["MIL".to_string()],
            Exchange::NewZealandStockExchange => vec!["NZE".to_string()],
            Exchange::SaoPauloStockExchange => vec!["SAO".to_string()],
            Exchange::KoreaStockExchange => vec!["KSC".to_string()],
            Exchange::FukuokaStockExchange => vec!["FGI".to_string()],
            Exchange::HongKongStockExchange => vec!["HKG".to_string()],
            Exchange::StockExchangeofThailand => vec!["SET".to_string()],
            Exchange::SingaporeExchangeSecurities => vec!["SES".to_string()],
            Exchange::ShanghaiStockExchange => vec!["SHH".to_string()],
            Exchange::SwissElectronicBourse => vec!["EBS".to_string()],
            Exchange::OsloStockExchange => vec!["OSL".to_string()],
            Exchange::TelAvivStockExchange => vec!["TLV".to_string()],
            Exchange::KoreaExchange => vec!["KOE".to_string()],
            Exchange::CopenhagenStockExchange => vec!["CPH".to_string()],
            Exchange::StuttgartStockExchange => vec!["STU".to_string()],
            Exchange::BursaMalaysia => vec!["KLS".to_string()],
            Exchange::HamburgStockExchange => vec!["HAM".to_string()],
            Exchange::ViennaStockExchange => vec!["VIE".to_string()],
            Exchange::PragueStockExchange => vec!["PRA".to_string()],
            Exchange::HanoiStockExchange => vec!["HAN".to_string()],
            Exchange::JohannesburgStockExchange => vec!["JNB".to_string()],
            Exchange::CboeDXE => vec!["DXE".to_string()],
            Exchange::MoscowExchange => vec!["MSC".to_string()],
            Exchange::CboeAustralia => vec!["CXA".to_string()],
            Exchange::ShenzhenStockExchange => vec!["SHZ".to_string()],
            Exchange::VietnamStockExchange => vec!["VSE".to_string()],
            Exchange::WarsawStockExchange => vec!["WSE".to_string()],
            Exchange::IntercontinentalExchange => vec!["ICE".to_string()],
            Exchange::RigaStockExchange => vec!["RIS".to_string()],
            Exchange::ZagrebStockExchange => vec!["CXE".to_string()],
            Exchange::JakartaStockExchange => vec!["JKT".to_string()],
            Exchange::TaiwanOTCExchange => vec!["TWO".to_string()],
            Exchange::OsakaStockExchange => vec!["OSA".to_string()],
            Exchange::AquisStockExchange => vec!["AQS".to_string()],
            Exchange::TaiwanStockExchange => vec!["TAI".to_string()],
            Exchange::QatarStockExchange => vec!["DOH".to_string()],
            Exchange::HelsinkiStockExchange => vec!["HEL".to_string()],
            Exchange::TallinnStockExchange => vec!["TSI".to_string()],
            Exchange::MoldovaStockExchange => vec!["MCE".to_string()],
            Exchange::NEOExchange => vec!["NEO".to_string()],
            Exchange::EuronextBrussels => vec!["BRU".to_string()],
            Exchange::VilniusStockExchange => vec!["LIT".to_string()],
            Exchange::BudapestStockExchange => vec!["BUD".to_string()],
            Exchange::EuronextLisbon => vec!["LIS".to_string()],
            Exchange::SantiagoStockExchange => vec!["SGO".to_string()],
            Exchange::FSI => vec!["FSI".to_string()],
            Exchange::IrishStockExchange => vec!["ISE".to_string()],
            Exchange::AthensStockExchange => vec!["ATH".to_string()],
            Exchange::SaudiStockExchange => vec!["SAU".to_string()],
            Exchange::TrinidadandTobagoStockExchange => vec!["TLO".to_string()],
            Exchange::CboeBXE => vec!["CBO".to_string()],
            Exchange::BVPBratislavaStockExchange => vec!["BVC".to_string()],
            Exchange::TAL => vec!["TAL".to_string()],
            Exchange::BoursaKuwait => vec!["KUW".to_string()],
            Exchange::EgyptianExchange => vec!["CAI".to_string()],
            Exchange::ColomboStockExchange => vec!["CSE".to_string()],
            Exchange::DubaiFinancialMarket => vec!["DFM".to_string()],
            Exchange::PhilippineStockExchange => vec!["PHS".to_string()],
            Exchange::KazakhstanStockExchange => vec!["FKA".to_string()],
            Exchange::OTCBulletinBoard => vec!["OBB".to_string()],
            Exchange::YHD => vec!["YHD".to_string()],
            Exchange::SAP => vec!["SAP".to_string()],
            Exchange::CaracasStockExchange => vec!["CCS".to_string()],
            Exchange::OPI => vec!["OPI".to_string()],
            Exchange::Euronext => vec!["ENX".to_string()],
            Exchange::All => get_distinct_exchanges().unwrap(),
        }
    }
}



