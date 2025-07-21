use strum::{EnumIter, EnumString, Display, AsRefStr, IntoStaticStr, VariantNames, EnumProperty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum QuoteType {
    #[strum(serialize = "EQUITY")]
    Equity,
    #[strum(serialize = "ETF")]
    Etf,
    #[strum(serialize = "MUTUALFUND")]
    MutualFund,
    #[strum(serialize = "INDEX")]
    Index,
    #[strum(serialize = "FUTURE")]
    Future,
    #[strum(serialize = "CRYPTOCURRENCY")]
    Crypto,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum Sector {
    #[strum(serialize = "Basic Materials")]
    BasicMaterials,
    #[strum(serialize = "Communication Services")]
    CommunicationServices,
    #[strum(serialize = "Consumer Cyclical")]
    ConsumerCyclical,
    #[strum(serialize = "Consumer Defensive")]
    ConsumerDefensive,
    #[strum(serialize = "Energy")]
    Energy,
    #[strum(serialize = "Financial Services")]
    FinancialServices,
    #[strum(serialize = "Healthcare")]
    Healthcare,
    #[strum(serialize = "Industrials")]
    Industrials,
    #[strum(serialize = "Real Estate")]
    RealEstate,
    #[strum(serialize = "Technology")]
    Technology,
    #[strum(serialize = "Utilities")]
    Utilities,
}

impl Sector {
    pub fn industries(&self) -> &'static [&'static str] {
        match self {
            Sector::BasicMaterials => &[
                "Agricultural Inputs",
                "Building Materials",
                "Chemicals",
                "Specialty Chemicals",
                "Lumber & Wood Production",
                "Paper & Paper Products",
                "Aluminum",
                "Copper",
                "Other Industrial Metals & Mining",
                "Gold",
                "Silver",
                "Other Precious Metals & Mining",
                "Coking Coal",
                "Steel",
            ],
            Sector::ConsumerCyclical => &[
                "Auto & Truck Dealerships",
                "Auto Manufacturers",
                "Auto Parts",
                "Recreational Vehicles",
                "Furnishings, Fixtures & Appliances",
                "Residential Construction",
                "Textile Manufacturing",
                "Apparel Manufacturing",
                "Footwear & Accessories",
                "Packaging & Containers",
                "Personal Services",
                "Restaurants",
                "Apparel Retail",
                "Department Stores",
                "Home Improvement Retail",
                "Luxury Goods",
                "Internet Retail",
                "Specialty Retail",
                "Gambling",
                "Leisure",
                "Lodging",
                "Resorts & Casinos",
                "Travel Services",
            ],
            Sector::FinancialServices => &[
                "Asset Management",
                "Banks - Diversified",
                "Banks - Regional",
                "Mortgage Finance",
                "Capital Markets",
                "Financial Data & Stock Exchanges",
                "Insurance - Life",
                "Insurance - Property & Casualty",
                "Insurance - Reinsurance",
                "Insurance - Specialty",
                "Insurance Brokers",
                "Insurance - Diversified",
                "Shell Companies",
                "Financial Conglomerates",
                "Credit Services",
            ],
            Sector::RealEstate => &[
                "Real Estate - Development",
                "Real Estate Services",
                "Real Estate - Diversified",
                "REIT - Healthcare Facilities",
                "REIT - Hotel & Motel",
                "REIT - Industrial",
                "REIT - Office",
                "REIT - Residential",
                "REIT - Retail",
                "REIT - Mortgage",
                "REIT - Specialty",
                "REIT - Diversified",
            ],
            Sector::ConsumerDefensive => &[
                "Beverages - Brewers",
                "Beverages - Wineries & Distilleries",
                "Beverages - Non-Alcoholic",
                "Confectioners",
                "Farm Products",
                "Household & Personal Products",
                "Packaged Foods",
                "Education & Training Services",
                "Discount Stores",
                "Food Distribution",
                "Grocery Stores",
                "Tobacco",
            ],
            Sector::Healthcare => &[
                "Biotechnology",
                "Drug Manufacturers - General",
                "Drug Manufacturers - Specialty & Generic",
                "Healthcare Plans",
                "Medical Care Facilities",
                "Pharmaceutical Retailers",
                "Health Information Services",
                "Medical Devices",
                "Medical Instruments & Supplies",
                "Diagnostics & Research",
                "Medical Distribution",
            ],
            Sector::Utilities => &[
                "Utilities - Independent Power Producers",
                "Utilities - Renewable",
                "Utilities - Regulated Water",
                "Utilities - Regulated Electric",
                "Utilities - Regulated Gas",
                "Utilities - Diversified",
            ],
            Sector::CommunicationServices => &[
                "Telecom Services",
                "Advertising Agencies",
                "Publishing",
                "Broadcasting",
                "Entertainment",
                "Internet Content & Information",
                "Electronic Gaming & Multimedia",
            ],
            Sector::Energy => &[
                "Oil & Gas Drilling",
                "Oil & Gas E&P",
                "Oil & Gas Integrated",
                "Oil & Gas Midstream",
                "Oil & Gas Refining & Marketing",
                "Oil & Gas Equipment & Services",
                "Thermal Coal",
                "Uranium",
            ],
            Sector::Industrials => &[
                "Aerospace & Defense",
                "Specialty Business Services",
                "Consulting Services",
                "Rental & Leasing Services",
                "Security & Protection Services",
                "Staffing & Employment Services",
                "Conglomerates",
                "Engineering & Construction",
                "Infrastructure Operations",
                "Building Products & Equipment",
                "Farm & Heavy Construction Machinery",
                "Industrial Distribution",
                "Business Equipment & Supplies",
                "Specialty Industrial Machinery",
                "Metal Fabrication",
                "Pollution & Treatment Controls",
                "FILTERLABEL_TOOLS_ACCESSORIES",
                "Electrical Equipment & Parts",
                "Airports & Air Services",
                "Airlines",
                "Railroads",
                "Marine Shipping",
                "Trucking",
                "Integrated Freight & Logistics",
                "Waste Management",
            ],
            Sector::Technology => &[
                "Information Technology Services",
                "Software - Application",
                "Software - Infrastructure",
                "Communication Equipment",
                "Computer Hardware",
                "Consumer Electronics",
                "Electronic Components",
                "Electronics & Computer Distribution",
                "Scientific & Technical Instruments",
                "Semiconductor Equipment & Materials",
                "Semiconductors",
                "Solar",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum Exchange {
    #[strum(serialize = "NYQ", props(full_name = "New York Stock Exchange"))]
    NewYorkStockExchange,

    #[strum(serialize = "NMS", props(full_name = "NASDAQ"))]
    NASDAQ,

    #[strum(serialize = "STO", props(full_name = "Stockholm Stock Exchange"))]
    StockholmStockExchange,

    #[strum(serialize = "DJI", props(full_name = "Dow Jones Indices"))]
    DowJonesIndices,

    #[strum(serialize = "NCM", props(full_name = "Nasdaq Capital Market"))]
    NasdaqCapitalMarket,

    #[strum(serialize = "NGM", props(full_name = "Nasdaq Global Market"))]
    NasdaqGlobalMarket,

    #[strum(serialize = "CCY", props(full_name = "Currencies"))]
    Currencies,

    #[strum(serialize = "CCC", props(full_name = "Cryptocurrencies"))]
    Cryptocurrencies,

    #[strum(serialize = "PCX", props(full_name = "NYSE Arca"))]
    NYSEArca,

    #[strum(serialize = "NIM", props(full_name = "NYSE American"))]
    NYSEAmerican,

    #[strum(serialize = "NYM", props(full_name = "New York Mercantile Exchange"))]
    NewYorkMercantileExchange,

    #[strum(serialize = "CMX", props(full_name = "COMEX"))]
    COMEX,

    #[strum(serialize = "CBT", props(full_name = "Chicago Board of Trade"))]
    ChicagoBoardofTrade,

    #[strum(serialize = "CME", props(full_name = "Chicago Mercantile Exchange"))]
    ChicagoMercantileExchange,

    #[strum(serialize = "PNK", props(full_name = "Pink Open Market"))]
    PinkOpenMarket,

    #[strum(serialize = "TOR", props(full_name = "Toronto Stock Exchange"))]
    TorontoStockExchange,

    #[strum(serialize = "ASE", props(full_name = "NYSE American Options"))]
    NYSEAmericanOptions,

    #[strum(serialize = "NYB", props(full_name = "New York Board of Trade"))]
    NewYorkBoardofTrade,

    #[strum(serialize = "SNP", props(full_name = "SNP Indices"))]
    SNPIndices,

    #[strum(serialize = "WCB", props(full_name = "West Coast Board of Trade"))]
    WestCoastBoardofTrade,

    #[strum(serialize = "BTS", props(full_name = "BTS"))]
    BTS,

    #[strum(serialize = "CXI", props(full_name = "Currency Exchange International"))]
    CurrencyExchangeInternational,

    #[strum(serialize = "NAS", props(full_name = "NASDAQ Stock Market"))]
    NASDAQStockMarket,

    #[strum(serialize = "NSI", props(full_name = "Nagoya Stock Exchange"))]
    NagoyaStockExchange,

    #[strum(serialize = "LSE", props(full_name = "London Stock Exchange"))]
    LondonStockExchange,

    #[strum(serialize = "GER", props(full_name = "Xetra"))]
    Xetra,

    #[strum(serialize = "BER", props(full_name = "Berlin Stock Exchange"))]
    BerlinStockExchange,

    #[strum(serialize = "DUS", props(full_name = "Dusseldorf Stock Exchange"))]
    DusseldorfStockExchange,

    #[strum(serialize = "PAR", props(full_name = "Euronext Paris"))]
    EuronextParis,

    #[strum(serialize = "NYS", props(full_name = "New York Stock Exchange ARCA"))]
    NewYorkStockExchangeARCA,

    #[strum(serialize = "IOB", props(full_name = "London IOB"))]
    LondonIOB,

    #[strum(serialize = "ZRH", props(full_name = "SIX Swiss Exchange"))]
    SIXSwissExchange,

    #[strum(serialize = "BUE", props(full_name = "Buenos Aires Stock Exchange"))]
    BuenosAiresStockExchange,

    #[strum(serialize = "BSE", props(full_name = "Bombay Stock Exchange"))]
    BombayStockExchange,

    #[strum(serialize = "ASX", props(full_name = "Australian Securities Exchange"))]
    AustralianSecuritiesExchange,

    #[strum(serialize = "VAN", props(full_name = "Vancouver Stock Exchange"))]
    VancouverStockExchange,

    #[strum(serialize = "AMS", props(full_name = "Amsterdam Stock Exchange"))]
    AmsterdamStockExchange,

    #[strum(serialize = "JPX", props(full_name = "Japan Exchange Group"))]
    JapanExchangeGroup,

    #[strum(serialize = "CNQ", props(full_name = "Canadian National Stock Exchange"))]
    CanadianNationalStockExchange,

    #[strum(serialize = "FRA", props(full_name = "Frankfurt Stock Exchange"))]
    FrankfurtStockExchange,

    #[strum(serialize = "MUN", props(full_name = "Munich Stock Exchange"))]
    MunichStockExchange,

    #[strum(serialize = "IST", props(full_name = "Istanbul Stock Exchange"))]
    IstanbulStockExchange,

    #[strum(serialize = "MEX", props(full_name = "Mexican Stock Exchange"))]
    MexicanStockExchange,

    #[strum(serialize = "MIL", props(full_name = "Milan Stock Exchange"))]
    MilanStockExchange,

    #[strum(serialize = "NZE", props(full_name = "New Zealand Stock Exchange"))]
    NewZealandStockExchange,

    #[strum(serialize = "SAO", props(full_name = "Sao Paulo Stock Exchange"))]
    SaoPauloStockExchange,

    #[strum(serialize = "KSC", props(full_name = "Korea Stock Exchange"))]
    KoreaStockExchange,

    #[strum(serialize = "FGI", props(full_name = "Fukuoka Stock Exchange"))]
    FukuokaStockExchange,

    #[strum(serialize = "HKG", props(full_name = "Hong Kong Stock Exchange"))]
    HongKongStockExchange,

    #[strum(serialize = "SET", props(full_name = "Stock Exchange of Thailand"))]
    StockExchangeofThailand,

    #[strum(serialize = "SES", props(full_name = "Singapore Exchange Securities"))]
    SingaporeExchangeSecurities,

    #[strum(serialize = "SHH", props(full_name = "Shanghai Stock Exchange"))]
    ShanghaiStockExchange,

    #[strum(serialize = "EBS", props(full_name = "Swiss Electronic Bourse"))]
    SwissElectronicBourse,

    #[strum(serialize = "OSL", props(full_name = "Oslo Stock Exchange"))]
    OsloStockExchange,

    #[strum(serialize = "TLV", props(full_name = "Tel Aviv Stock Exchange"))]
    TelAvivStockExchange,

    #[strum(serialize = "KOE", props(full_name = "Korea Exchange"))]
    KoreaExchange,

    #[strum(serialize = "CPH", props(full_name = "Copenhagen Stock Exchange"))]
    CopenhagenStockExchange,

    #[strum(serialize = "STU", props(full_name = "Stuttgart Stock Exchange"))]
    StuttgartStockExchange,

    #[strum(serialize = "KLS", props(full_name = "Bursa Malaysia"))]
    BursaMalaysia,

    #[strum(serialize = "HAM", props(full_name = "Hamburg Stock Exchange"))]
    HamburgStockExchange,

    #[strum(serialize = "VIE", props(full_name = "Vienna Stock Exchange"))]
    ViennaStockExchange,

    #[strum(serialize = "PRA", props(full_name = "Prague Stock Exchange"))]
    PragueStockExchange,

    #[strum(serialize = "HAN", props(full_name = "Hanoi Stock Exchange"))]
    HanoiStockExchange,

    #[strum(serialize = "JNB", props(full_name = "Johannesburg Stock Exchange"))]
    JohannesburgStockExchange,

    #[strum(serialize = "DXE", props(full_name = "Cboe DXE"))]
    CboeDXE,

    #[strum(serialize = "MSC", props(full_name = "Moscow Exchange"))]
    MoscowExchange,

    #[strum(serialize = "CXA", props(full_name = "Cboe Australia"))]
    CboeAustralia,

    #[strum(serialize = "SHZ", props(full_name = "Shenzhen Stock Exchange"))]
    ShenzhenStockExchange,

    #[strum(serialize = "VSE", props(full_name = "Vietnam Stock Exchange"))]
    VietnamStockExchange,

    #[strum(serialize = "WSE", props(full_name = "Warsaw Stock Exchange"))]
    WarsawStockExchange,

    #[strum(serialize = "ICE", props(full_name = "Intercontinental Exchange"))]
    IntercontinentalExchange,

    #[strum(serialize = "RIS", props(full_name = "Riga Stock Exchange"))]
    RigaStockExchange,

    #[strum(serialize = "CXE", props(full_name = "Zagreb Stock Exchange"))]
    ZagrebStockExchange,

    #[strum(serialize = "JKT", props(full_name = "Jakarta Stock Exchange"))]
    JakartaStockExchange,

    #[strum(serialize = "TWO", props(full_name = "Taiwan OTC Exchange"))]
    TaiwanOTCExchange,

    #[strum(serialize = "OSA", props(full_name = "Osaka Stock Exchange"))]
    OsakaStockExchange,

    #[strum(serialize = "AQS", props(full_name = "Aquis Stock Exchange"))]
    AquisStockExchange,

    #[strum(serialize = "TAI", props(full_name = "Taiwan Stock Exchange"))]
    TaiwanStockExchange,

    #[strum(serialize = "DOH", props(full_name = "Qatar Stock Exchange"))]
    QatarStockExchange,

    #[strum(serialize = "HEL", props(full_name = "Helsinki Stock Exchange"))]
    HelsinkiStockExchange,

    #[strum(serialize = "TSI", props(full_name = "Tallinn Stock Exchange"))]
    TallinnStockExchange,

    #[strum(serialize = "MCE", props(full_name = "Moldova Stock Exchange"))]
    MoldovaStockExchange,

    #[strum(serialize = "NEO", props(full_name = "NEO Exchange"))]
    NEOExchange,

    #[strum(serialize = "BRU", props(full_name = "Euronext Brussels"))]
    EuronextBrussels,

    #[strum(serialize = "LIT", props(full_name = "Vilnius Stock Exchange"))]
    VilniusStockExchange,

    #[strum(serialize = "BUD", props(full_name = "Budapest Stock Exchange"))]
    BudapestStockExchange,

    #[strum(serialize = "LIS", props(full_name = "Euronext Lisbon"))]
    EuronextLisbon,

    #[strum(serialize = "SGO", props(full_name = "Santiago Stock Exchange"))]
    SantiagoStockExchange,

    #[strum(serialize = "FSI", props(full_name = "FSI"))]
    FSI,

    #[strum(serialize = "ISE", props(full_name = "Irish Stock Exchange"))]
    IrishStockExchange,

    #[strum(serialize = "ATH", props(full_name = "Athens Stock Exchange"))]
    AthensStockExchange,

    #[strum(serialize = "SAU", props(full_name = "Saudi Stock Exchange"))]
    SaudiStockExchange,

    #[strum(serialize = "TLO", props(full_name = "Trinidad and Tobago Stock Exchange"))]
    TrinidadandTobagoStockExchange,

    #[strum(serialize = "CBO", props(full_name = "Cboe BXE"))]
    CboeBXE,

    #[strum(serialize = "BVC", props(full_name = "BVP Bratislava Stock Exchange"))]
    BVPBratislavaStockExchange,

    #[strum(serialize = "TAL", props(full_name = "TAL"))]
    TAL,

    #[strum(serialize = "KUW", props(full_name = "Boursa Kuwait"))]
    BoursaKuwait,

    #[strum(serialize = "CAI", props(full_name = "Egyptian Exchange"))]
    EgyptianExchange,

    #[strum(serialize = "CSE", props(full_name = "Colombo Stock Exchange"))]
    ColomboStockExchange,

    #[strum(serialize = "DFM", props(full_name = "Dubai Financial Market"))]
    DubaiFinancialMarket,

    #[strum(serialize = "PHS", props(full_name = "Philippine Stock Exchange"))]
    PhilippineStockExchange,

    #[strum(serialize = "FKA", props(full_name = "Kazakhstan Stock Exchange"))]
    KazakhstanStockExchange,

    #[strum(serialize = "OBB", props(full_name = "OTC Bulletin Board"))]
    OTCBulletinBoard,

    #[strum(serialize = "YHD", props(full_name = "YHD"))]
    YHD,

    #[strum(serialize = "SAP", props(full_name = "SAP"))]
    SAP,

    #[strum(serialize = "CCS", props(full_name = "Caracas Stock Exchange"))]
    CaracasStockExchange,

    #[strum(serialize = "OPI", props(full_name = "OPI"))]
    OPI,

    #[strum(serialize = "ENX", props(full_name = "Euronext"))]
    Euronext,
}

impl Exchange {
    pub fn full_name(&self) -> &'static str {
        self.get_str("full_name").unwrap_or("")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames, EnumProperty)]
pub enum Region {
    #[strum(serialize = "ar", props(full_name = "Argentina"))]
    Argentina,

    #[strum(serialize = "at", props(full_name = "Austria"))]
    Austria,

    #[strum(serialize = "au", props(full_name = "Australia"))]
    Australia,

    #[strum(serialize = "be", props(full_name = "Belgium"))]
    Belgium,

    #[strum(serialize = "br", props(full_name = "Brazil"))]
    Brazil,

    #[strum(serialize = "ca", props(full_name = "Canada"))]
    Canada,

    #[strum(serialize = "ch", props(full_name = "Switzerland"))]
    Switzerland,

    #[strum(serialize = "cl", props(full_name = "Chile"))]
    Chile,

    #[strum(serialize = "cn", props(full_name = "China"))]
    China,

    #[strum(serialize = "cz", props(full_name = "Czechia"))]
    Czechia,

    #[strum(serialize = "de", props(full_name = "Germany"))]
    Germany,

    #[strum(serialize = "dk", props(full_name = "Denmark"))]
    Denmark,

    #[strum(serialize = "ee", props(full_name = "Estonia"))]
    Estonia,

    #[strum(serialize = "eg", props(full_name = "Egypt"))]
    Egypt,

    #[strum(serialize = "es", props(full_name = "Spain"))]
    Spain,

    #[strum(serialize = "fi", props(full_name = "Finland"))]
    Finland,

    #[strum(serialize = "fr", props(full_name = "France"))]
    France,

    #[strum(serialize = "gb", props(full_name = "United Kingdom"))]
    UnitedKingdom,

    #[strum(serialize = "gr", props(full_name = "Greece"))]
    Greece,

    #[strum(serialize = "hk", props(full_name = "Hong Kong Sar China"))]
    HongKongSarChina,

    #[strum(serialize = "hu", props(full_name = "Hungary"))]
    Hungary,

    #[strum(serialize = "id", props(full_name = "Indonesia"))]
    Indonesia,

    #[strum(serialize = "ie", props(full_name = "Ireland"))]
    Ireland,

    #[strum(serialize = "il", props(full_name = "Israel"))]
    Israel,

    #[strum(serialize = "in", props(full_name = "India"))]
    India,

    #[strum(serialize = "is", props(full_name = "Iceland"))]
    Iceland,

    #[strum(serialize = "it", props(full_name = "Italy"))]
    Italy,

    #[strum(serialize = "jp", props(full_name = "Japan"))]
    Japan,

    #[strum(serialize = "kr", props(full_name = "South Korea"))]
    SouthKorea,

    #[strum(serialize = "kw", props(full_name = "Kuwait"))]
    Kuwait,

    #[strum(serialize = "lk", props(full_name = "Sri Lanka"))]
    SriLanka,

    #[strum(serialize = "lt", props(full_name = "Lithuania"))]
    Lithuania,

    #[strum(serialize = "lv", props(full_name = "Latvia"))]
    Latvia,

    #[strum(serialize = "mx", props(full_name = "Mexico"))]
    Mexico,

    #[strum(serialize = "my", props(full_name = "Malaysia"))]
    Malaysia,

    #[strum(serialize = "nl", props(full_name = "Netherlands"))]
    Netherlands,

    #[strum(serialize = "no", props(full_name = "Norway"))]
    Norway,

    #[strum(serialize = "nz", props(full_name = "New Zealand"))]
    NewZealand,

    #[strum(serialize = "pe", props(full_name = "Peru"))]
    Peru,

    #[strum(serialize = "ph", props(full_name = "Philippines"))]
    Philippines,

    #[strum(serialize = "pk", props(full_name = "Pakistan"))]
    Pakistan,

    #[strum(serialize = "pl", props(full_name = "Poland"))]
    Poland,

    #[strum(serialize = "pt", props(full_name = "Portugal"))]
    Portugal,

    #[strum(serialize = "qa", props(full_name = "Qatar"))]
    Qatar,

    #[strum(serialize = "ru", props(full_name = "Russia"))]
    Russia,

    #[strum(serialize = "sa", props(full_name = "Saudi Arabia"))]
    SaudiArabia,

    #[strum(serialize = "se", props(full_name = "Sweden"))]
    Sweden,

    #[strum(serialize = "sg", props(full_name = "Singapore"))]
    Singapore,

    #[strum(serialize = "sr", props(full_name = "Suriname"))]
    Suriname,

    #[strum(serialize = "th", props(full_name = "Thailand"))]
    Thailand,

    #[strum(serialize = "tr", props(full_name = "Turkey"))]
    Turkey,

    #[strum(serialize = "tw", props(full_name = "Taiwan"))]
    Taiwan,

    #[strum(serialize = "us", props(full_name = "United States"))]
    UnitedStates,

    #[strum(serialize = "ve", props(full_name = "Venezuela"))]
    Venezuela,

    #[strum(serialize = "vn", props(full_name = "Vietnam"))]
    Vietnam,

    #[strum(serialize = "za", props(full_name = "South Africa"))]
    SouthAfrica,
}

impl Region {
    pub fn full_name(&self) -> &'static str {
        self.get_str("full_name").unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames)]
pub enum PeerGroup {
    #[strum(serialize = "Aerospace & Defense")]
    AerospaceDefense,

    #[strum(serialize = "Auto Components")]
    AutoComponents,

    #[strum(serialize = "Automobiles")]
    Automobiles,

    #[strum(serialize = "Banks")]
    Banks,

    #[strum(serialize = "Building Products")]
    BuildingProducts,

    #[strum(serialize = "Chemicals")]
    Chemicals,

    #[strum(serialize = "China Fund Aggressive Allocation Fund")]
    ChinaFundAggressiveAllocationFund,

    #[strum(serialize = "China Fund Equity Funds")]
    ChinaFundEquityFunds,

    #[strum(serialize = "China Fund QDII Greater China Equity")]
    ChinaFundQDIIGreaterChinaEquity,

    #[strum(serialize = "China Fund QDII Sector Equity")]
    ChinaFundQDIISectorEquity,

    #[strum(serialize = "China Fund Sector Equity Financial and Real Estate")]
    ChinaFundSectorEquityFinancialAndRealEstate,

    #[strum(serialize = "Commercial Services")]
    CommercialServices,

    #[strum(serialize = "Construction & Engineering")]
    ConstructionEngineering,

    #[strum(serialize = "Construction Materials")]
    ConstructionMaterials,

    #[strum(serialize = "Consumer Durables")]
    ConsumerDurables,

    #[strum(serialize = "Consumer Services")]
    ConsumerServices,

    #[strum(serialize = "Containers & Packaging")]
    ContainersPackaging,

    #[strum(serialize = "Diversified Financials")]
    DiversifiedFinancials,

    #[strum(serialize = "Diversified Metals")]
    DiversifiedMetals,

    #[strum(serialize = "EAA CE Global Large-Cap Blend Equity")]
    EAACEGlobalLargeCapBlendEquity,

    #[strum(serialize = "EAA CE Other")]
    EAACEOther,

    #[strum(serialize = "EAA CE Sector Equity Biotechnology")]
    EAACESectorEquityBiotechnology,

    #[strum(serialize = "EAA CE UK Large-Cap Equity")]
    EAACEUKLargeCapEquity,

    #[strum(serialize = "EAA CE UK Small-Cap Equity")]
    EAACEUKSmallCapEquity,

    #[strum(serialize = "EAA Fund Asia ex-Japan Equity")]
    EAAFundAsiaExJapanEquity,

    #[strum(serialize = "EAA Fund China Equity - A Shares")]
    EAAFundChinaEquityAShares,

    #[strum(serialize = "EAA Fund China Equity")]
    EAAFundChinaEquity,

    #[strum(serialize = "EAA Fund Denmark Equity")]
    EAAFundDenmarkEquity,

    #[strum(serialize = "EAA Fund Emerging Europe ex-Russia Equity")]
    EAAFundEmergingEuropeExRussiaEquity,

    #[strum(serialize = "EAA Fund EUR Aggressive Allocation - Global")]
    EAAFundEURAggressiveAllocationGlobal,

    #[strum(serialize = "EAA Fund EUR Corporate Bond")]
    EAAFundEURCorporateBond,

    #[strum(serialize = "EAA Fund EUR Moderate Allocation - Global")]
    EAAFundEURModerateAllocationGlobal,

    #[strum(serialize = "EAA Fund Europe Large-Cap Blend Equity")]
    EAAFundEuropeLargeCapBlendEquity,

    #[strum(serialize = "EAA Fund Eurozone Large-Cap Equity")]
    EAAFundEurozoneLargeCapEquity,

    #[strum(serialize = "EAA Fund Germany Equity")]
    EAAFundGermanyEquity,

    #[strum(serialize = "EAA Fund Global Emerging Markets Equity")]
    EAAFundGlobalEmergingMarketsEquity,

    #[strum(serialize = "EAA Fund Global Equity Income")]
    EAAFundGlobalEquityIncome,

    #[strum(serialize = "EAA Fund Global Flex-Cap Equity")]
    EAAFundGlobalFlexCapEquity,

    #[strum(serialize = "EAA Fund Global Large-Cap Blend Equity")]
    EAAFundGlobalLargeCapBlendEquity,

    #[strum(serialize = "EAA Fund Global Large-Cap Growth Equity")]
    EAAFundGlobalLargeCapGrowthEquity,

    #[strum(serialize = "EAA Fund Hong Kong Equity")]
    EAAFundHongKongEquity,

    #[strum(serialize = "EAA Fund Japan Large-Cap Equity")]
    EAAFundJapanLargeCapEquity,

    #[strum(serialize = "EAA Fund Other Bond")]
    EAAFundOtherBond,

    #[strum(serialize = "EAA Fund Other Equity")]
    EAAFundOtherEquity,

    #[strum(serialize = "EAA Fund RMB Bond - Onshore")]
    EAAFundRMBBondOnshore,

    #[strum(serialize = "EAA Fund Sector Equity Consumer Goods & Services")]
    EAAFundSectorEquityConsumerGoodsServices,

    #[strum(serialize = "EAA Fund Sector Equity Financial Services")]
    EAAFundSectorEquityFinancialServices,

    #[strum(serialize = "EAA Fund Sector Equity Industrial Materials")]
    EAAFundSectorEquityIndustrialMaterials,

    #[strum(serialize = "EAA Fund Sector Equity Technology")]
    EAAFundSectorEquityTechnology,

    #[strum(serialize = "EAA Fund South Africa & Namibia Equity")]
    EAAFundSouthAfricaNamibiaEquity,

    #[strum(serialize = "EAA Fund Switzerland Equity")]
    EAAFundSwitzerlandEquity,

    #[strum(serialize = "EAA Fund US Large-Cap Blend Equity")]
    EAAFundUSLargeCapBlendEquity,

    #[strum(serialize = "EAA Fund USD Corporate Bond")]
    EAAFundUSDCorporateBond,

    #[strum(serialize = "Electrical Equipment")]
    ElectricalEquipment,

    #[strum(serialize = "Energy Services")]
    EnergyServices,

    #[strum(serialize = "Food Products")]
    FoodProducts,

    #[strum(serialize = "Food Retailers")]
    FoodRetailers,

    #[strum(serialize = "Healthcare")]
    Healthcare,

    #[strum(serialize = "Homebuilders")]
    Homebuilders,

    #[strum(serialize = "Household Products")]
    HouseholdProducts,

    #[strum(serialize = "India CE Multi-Cap")]
    IndiaCEMultiCap,

    #[strum(serialize = "India Fund Large-Cap")]
    IndiaFundLargeCap,

    #[strum(serialize = "India Fund Sector - Financial Services")]
    IndiaFundSectorFinancialServices,

    #[strum(serialize = "Industrial Conglomerates")]
    IndustrialConglomerates,

    #[strum(serialize = "Insurance")]
    Insurance,

    #[strum(serialize = "Machinery")]
    Machinery,

    #[strum(serialize = "Media")]
    Media,

    #[strum(serialize = "Mexico Fund Mexico Equity")]
    MexicoFundMexicoEquity,

    #[strum(serialize = "Oil & Gas Producers")]
    OilGasProducers,

    #[strum(serialize = "Paper & Forestry")]
    PaperForestry,

    #[strum(serialize = "Pharmaceuticals")]
    Pharmaceuticals,

    #[strum(serialize = "Precious Metals")]
    PreciousMetals,

    #[strum(serialize = "Real Estate")]
    RealEstate,

    #[strum(serialize = "Refiners & Pipelines")]
    RefinersPipelines,

    #[strum(serialize = "Retailing")]
    Retailing,

    #[strum(serialize = "Semiconductors")]
    Semiconductors,

    #[strum(serialize = "Software & Services")]
    SoftwareServices,

    #[strum(serialize = "Steel")]
    Steel,

    #[strum(serialize = "Technology Hardware")]
    TechnologyHardware,

    #[strum(serialize = "Telecommunication Services")]
    TelecommunicationServices,

    #[strum(serialize = "Textiles & Apparel")]
    TextilesApparel,

    #[strum(serialize = "Traders & Distributors")]
    TradersDistributors,

    #[strum(serialize = "Transportation Infrastructure")]
    TransportationInfrastructure,

    #[strum(serialize = "Transportation")]
    Transportation,

    #[strum(serialize = "US CE Convertibles")]
    USCEConvertibles,

    #[strum(serialize = "US CE Options-based")]
    USCEOptionsBased,

    #[strum(serialize = "US CE Preferred Stock")]
    USCEPreferredStock,

    #[strum(serialize = "US Fund China Region")]
    USFundChinaRegion,

    #[strum(serialize = "US Fund Consumer Cyclical")]
    USFundConsumerCyclical,

    #[strum(serialize = "US Fund Diversified Emerging Mkts")]
    USFundDiversifiedEmergingMkts,

    #[strum(serialize = "US Fund Equity Energy")]
    USFundEquityEnergy,

    #[strum(serialize = "US Fund Equity Precious Metals")]
    USFundEquityPreciousMetals,

    #[strum(serialize = "US Fund Financial")]
    USFundFinancial,

    #[strum(serialize = "US Fund Foreign Large Blend")]
    USFundForeignLargeBlend,

    #[strum(serialize = "US Fund Health")]
    USFundHealth,

    #[strum(serialize = "US Fund Large Blend")]
    USFundLargeBlend,

    #[strum(serialize = "US Fund Large Growth")]
    USFundLargeGrowth,

    #[strum(serialize = "US Fund Large Value")]
    USFundLargeValue,

    #[strum(serialize = "US Fund Miscellaneous Region")]
    USFundMiscellaneousRegion,

    #[strum(serialize = "US Fund Natural Resources")]
    USFundNaturalResources,

    #[strum(serialize = "US Fund Technology")]
    USFundTechnology,

    #[strum(serialize = "US Fund Trading--Leveraged Equity")]
    USFundTradingLeveragedEquity,

    #[strum(serialize = "Utilities")]
    Utilities,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames)]
pub enum FundFamily {
    #[strum(serialize = "ALPS")]
    ALPS,
    #[strum(serialize = "AMG Funds")]
    AMGFunds,
    #[strum(serialize = "AQR Funds")]
    AQRFunds,
    #[strum(serialize = "Aberdeen")]
    Aberdeen,
    #[strum(serialize = "Alger")]
    Alger,
    #[strum(serialize = "AllianceBernstein")]
    AllianceBernstein,
    #[strum(serialize = "Allianz Funds")]
    AllianzFunds,
    #[strum(serialize = "American Beacon")]
    AmericanBeacon,
    #[strum(serialize = "American Century Investments")]
    AmericanCenturyInvestments,
    #[strum(serialize = "American Funds")]
    AmericanFunds,
    #[strum(serialize = "Aquila")]
    Aquila,
    #[strum(serialize = "Artisan")]
    Artisan,
    #[strum(serialize = "BMO Funds")]
    BMOFunds,
    #[strum(serialize = "BNY Mellon Funds")]
    BNYMellonFunds,
    #[strum(serialize = "Baird")]
    Baird,
    #[strum(serialize = "Barclays Funds")]
    BarclaysFunds,
    #[strum(serialize = "Barings Funds")]
    BaringsFunds,
    #[strum(serialize = "Baron Capital Group")]
    BaronCapitalGroup,
    #[strum(serialize = "BlackRock")]
    BlackRock,
    #[strum(serialize = "Brown Advisory Funds")]
    BrownAdvisoryFunds,
    #[strum(serialize = "Calamos")]
    Calamos,
    #[strum(serialize = "Calvert Investments")]
    CalvertInvestments,
    #[strum(serialize = "Catalyst Mutual Funds")]
    CatalystMutualFunds,
    #[strum(serialize = "Cohen & Steers")]
    CohenSteers,
    #[strum(serialize = "Columbia")]
    Columbia,
    #[strum(serialize = "Commerz Funds Solutions SA")]
    CommerzFundsSolutionsSA,
    #[strum(serialize = "Commerzbank AG, Frankfurt am Main")]
    CommerzbankAGFrankfurtAmMain,
    #[strum(serialize = "Davis Funds")]
    DavisFunds,
    #[strum(serialize = "Delaware Investments")]
    DelawareInvestments,
    #[strum(serialize = "Deutsche Asset Management")]
    DeutscheAssetManagement,
    #[strum(serialize = "Deutsche Bank AG")]
    DeutscheBankAG,
    #[strum(serialize = "Diamond Hill Funds")]
    DiamondHillFunds,
    #[strum(serialize = "Dimensional Fund Advisors")]
    DimensionalFundAdvisors,
    #[strum(serialize = "Direxion Funds")]
    DirexionFunds,
    #[strum(serialize = "DoubleLine")]
    DoubleLine,
    #[strum(serialize = "Dreyfus")]
    Dreyfus,
    #[strum(serialize = "Dunham Funds")]
    DunhamFunds,
    #[strum(serialize = "Eagle Funds")]
    EagleFunds,
    #[strum(serialize = "Eaton Vance")]
    EatonVance,
    #[strum(serialize = "Federated")]
    Federated,
    #[strum(serialize = "Fidelity Investments")]
    FidelityInvestments,
    #[strum(serialize = "First Investors")]
    FirstInvestors,
    #[strum(serialize = "First Trust")]
    FirstTrust,
    #[strum(serialize = "Flexshares Trust")]
    FlexsharesTrust,
    #[strum(serialize = "Franklin Templeton Investments")]
    FranklinTempletonInvestments,
    #[strum(serialize = "GMO")]
    GMO,
    #[strum(serialize = "Gabelli")]
    Gabelli,
    #[strum(serialize = "Global X Funds")]
    GlobalXFunds,
    #[strum(serialize = "Goldman Sachs")]
    GoldmanSachs,
    #[strum(serialize = "Great-West Funds")]
    GreatWestFunds,
    #[strum(serialize = "Guggenheim Investments")]
    GuggenheimInvestments,
    #[strum(serialize = "GuideStone Funds")]
    GuideStoneFunds,
    #[strum(serialize = "HSBC")]
    HSBC,
    #[strum(serialize = "Hancock Horizon")]
    HancockHorizon,
    #[strum(serialize = "Harbor")]
    Harbor,
    #[strum(serialize = "Hartford Mutual Funds")]
    HartfordMutualFunds,
    #[strum(serialize = "Henderson Global")]
    HendersonGlobal,
    #[strum(serialize = "Hennessy")]
    Hennessy,
    #[strum(serialize = "Highland Funds")]
    HighlandFunds,
    #[strum(serialize = "ICON Funds")]
    ICONFunds,
    #[strum(serialize = "Invesco")]
    Invesco,
    #[strum(serialize = "Ivy Funds")]
    IvyFunds,
    #[strum(serialize = "JPMorgan")]
    JPMorgan,
    #[strum(serialize = "Janus")]
    Janus,
    #[strum(serialize = "John Hancock")]
    JohnHancock,
    #[strum(serialize = "Lazard")]
    Lazard,
    #[strum(serialize = "Legg Mason")]
    LeggMason,
    #[strum(serialize = "Lord Abbett")]
    LordAbbett,
    #[strum(serialize = "MFS")]
    MFS,
    #[strum(serialize = "Madison Funds")]
    MadisonFunds,
    #[strum(serialize = "MainStay")]
    MainStay,
    #[strum(serialize = "Manning & Napier")]
    ManningNapier,
    #[strum(serialize = "Market Vectors")]
    MarketVectors,
    #[strum(serialize = "MassMutual")]
    MassMutual,
    #[strum(serialize = "Matthews Asia Funds")]
    MatthewsAsiaFunds,
    #[strum(serialize = "Morgan Stanley")]
    MorganStanley,
    #[strum(serialize = "Nationwide")]
    Nationwide,
    #[strum(serialize = "Natixis Funds")]
    NatixisFunds,
    #[strum(serialize = "Neuberger Berman")]
    NeubergerBerman,
    #[strum(serialize = "Northern Funds")]
    NorthernFunds,
    #[strum(serialize = "Nuveen")]
    Nuveen,
    #[strum(serialize = "OppenheimerFunds")]
    OppenheimerFunds,
    #[strum(serialize = "PNC Funds")]
    PNCFunds,
    #[strum(serialize = "Pacific funds series trust")]
    PacificFundsSeriesTrust,
    #[strum(serialize = "Pax World")]
    PaxWorld,
    #[strum(serialize = "Paydenfunds")]
    Paydenfunds,
    #[strum(serialize = "Pimco")]
    Pimco,
    #[strum(serialize = "Pioneer Investments")]
    PioneerInvestments,
    #[strum(serialize = "PowerShares")]
    PowerShares,
    #[strum(serialize = "Principal Funds")]
    PrincipalFunds,
    #[strum(serialize = "ProFunds")]
    ProFunds,
    #[strum(serialize = "ProShares")]
    ProShares,
    #[strum(serialize = "Prudential Investments")]
    PrudentialInvestments,
    #[strum(serialize = "Putnam")]
    Putnam,
    #[strum(serialize = "RBC Global Asset Management")]
    RBCGlobalAssetManagement,
    #[strum(serialize = "RidgeWorth")]
    RidgeWorth,
    #[strum(serialize = "Royce")]
    Royce,
    #[strum(serialize = "Russell")]
    Russell,
    #[strum(serialize = "Rydex Funds")]
    RydexFunds,
    #[strum(serialize = "SEI")]
    SEI,
    #[strum(serialize = "SPDR State Street Global Advisors")]
    SPDRStateStreetGlobalAdvisors,
    #[strum(serialize = "Salient Funds")]
    SalientFunds,
    #[strum(serialize = "Saratoga")]
    Saratoga,
    #[strum(serialize = "Schwab Funds")]
    SchwabFunds,
    #[strum(serialize = "Sentinel")]
    Sentinel,
    #[strum(serialize = "Shelton Capital Management")]
    SheltonCapitalManagement,
    #[strum(serialize = "State Farm")]
    StateFarm,
    #[strum(serialize = "State Street Global Advisors (Chicago)")]
    StateStreetGlobalAdvisorsChicago,
    #[strum(serialize = "Sterling Capital Funds")]
    SterlingCapitalFunds,
    #[strum(serialize = "SunAmerica")]
    SunAmerica,
    #[strum(serialize = "T. Rowe Price")]
    TRowePrice,
    #[strum(serialize = "TCW")]
    TCW,
    #[strum(serialize = "TIAA-CREF Asset Management")]
    TIAACREFAsssetManagement,
    #[strum(serialize = "Teton Westwood Funds")]
    TetonWestwoodFunds,
    #[strum(serialize = "Thornburg")]
    Thornburg,
    #[strum(serialize = "Thrivent")]
    Thrivent,
    #[strum(serialize = "Timothy Plan")]
    TimothyPlan,
    #[strum(serialize = "Touchstone")]
    Touchstone,
    #[strum(serialize = "Transamerica")]
    Transamerica,
    #[strum(serialize = "UBS")]
    UBS,
    #[strum(serialize = "UBS Group AG")]
    UBSGroupAG,
    #[strum(serialize = "USAA")]
    USAA,
    #[strum(serialize = "VALIC")]
    VALIC,
    #[strum(serialize = "Vanguard")]
    Vanguard,
    #[strum(serialize = "Vantagepoint Funds")]
    VantagepointFunds,
    #[strum(serialize = "Victory")]
    Victory,
    #[strum(serialize = "Virtus")]
    Virtus,
    #[strum(serialize = "Voya")]
    Voya,
    #[strum(serialize = "Waddell & Reed")]
    WaddellReed,
    #[strum(serialize = "Wasatch")]
    Wasatch,
    #[strum(serialize = "Wells Fargo Funds")]
    WellsFargoFunds,
    #[strum(serialize = "William Blair")]
    WilliamBlair,
    #[strum(serialize = "WisdomTree")]
    WisdomTree,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, EnumProperty, VariantNames)]
pub enum Industry {
    // Basic Materials
    #[strum(serialize = "Agricultural Inputs", props(sector = "Basic Materials"))]
    AgriculturalInputs,
    #[strum(serialize = "Building Materials", props(sector = "Basic Materials"))]
    BuildingMaterials,
    #[strum(serialize = "Chemicals", props(sector = "Basic Materials"))]
    Chemicals,
    #[strum(serialize = "Specialty Chemicals", props(sector = "Basic Materials"))]
    SpecialtyChemicals,
    #[strum(serialize = "Lumber & Wood Production", props(sector = "Basic Materials"))]
    LumberAndWoodProduction,
    #[strum(serialize = "Paper & Paper Products", props(sector = "Basic Materials"))]
    PaperAndPaperProducts,
    #[strum(serialize = "Aluminum", props(sector = "Basic Materials"))]
    Aluminum,
    #[strum(serialize = "Copper", props(sector = "Basic Materials"))]
    Copper,
    #[strum(serialize = "Other Industrial Metals & Mining", props(sector = "Basic Materials"))]
    OtherIndustrialMetalsAndMining,
    #[strum(serialize = "Gold", props(sector = "Basic Materials"))]
    Gold,
    #[strum(serialize = "Silver", props(sector = "Basic Materials"))]
    Silver,
    #[strum(serialize = "Other Precious Metals & Mining", props(sector = "Basic Materials"))]
    OtherPreciousMetalsAndMining,
    #[strum(serialize = "Coking Coal", props(sector = "Basic Materials"))]
    CokingCoal,
    #[strum(serialize = "Steel", props(sector = "Basic Materials"))]
    Steel,

    // Consumer Cyclical
    #[strum(serialize = "Auto & Truck Dealerships", props(sector = "Consumer Cyclical"))]
    AutoAndTruckDealerships,
    #[strum(serialize = "Auto Manufacturers", props(sector = "Consumer Cyclical"))]
    AutoManufacturers,
    #[strum(serialize = "Auto Parts", props(sector = "Consumer Cyclical"))]
    AutoParts,
    #[strum(serialize = "Recreational Vehicles", props(sector = "Consumer Cyclical"))]
    RecreationalVehicles,
    #[strum(serialize = "Furnishings, Fixtures & Appliances", props(sector = "Consumer Cyclical"))]
    FurnishingsFixturesAndAppliances,
    #[strum(serialize = "Residential Construction", props(sector = "Consumer Cyclical"))]
    ResidentialConstruction,
    #[strum(serialize = "Textile Manufacturing", props(sector = "Consumer Cyclical"))]
    TextileManufacturing,
    #[strum(serialize = "Apparel Manufacturing", props(sector = "Consumer Cyclical"))]
    ApparelManufacturing,
    #[strum(serialize = "Footwear & Accessories", props(sector = "Consumer Cyclical"))]
    FootwearAndAccessories,
    #[strum(serialize = "Packaging & Containers", props(sector = "Consumer Cyclical"))]
    PackagingAndContainers,
    #[strum(serialize = "Personal Services", props(sector = "Consumer Cyclical"))]
    PersonalServices,
    #[strum(serialize = "Restaurants", props(sector = "Consumer Cyclical"))]
    Restaurants,
    #[strum(serialize = "Apparel Retail", props(sector = "Consumer Cyclical"))]
    ApparelRetail,
    #[strum(serialize = "Department Stores", props(sector = "Consumer Cyclical"))]
    DepartmentStores,
    #[strum(serialize = "Home Improvement Retail", props(sector = "Consumer Cyclical"))]
    HomeImprovementRetail,
    #[strum(serialize = "Luxury Goods", props(sector = "Consumer Cyclical"))]
    LuxuryGoods,
    #[strum(serialize = "Internet Retail", props(sector = "Consumer Cyclical"))]
    InternetRetail,
    #[strum(serialize = "Specialty Retail", props(sector = "Consumer Cyclical"))]
    SpecialtyRetail,
    #[strum(serialize = "Gambling", props(sector = "Consumer Cyclical"))]
    Gambling,
    #[strum(serialize = "Leisure", props(sector = "Consumer Cyclical"))]
    Leisure,
    #[strum(serialize = "Lodging", props(sector = "Consumer Cyclical"))]
    Lodging,
    #[strum(serialize = "Resorts & Casinos", props(sector = "Consumer Cyclical"))]
    ResortsAndCasinos,
    #[strum(serialize = "Travel Services", props(sector = "Consumer Cyclical"))]
    TravelServices,

    // Financial Services
    #[strum(serialize = "Asset Management", props(sector = "Financial Services"))]
    AssetManagement,
    #[strum(serialize = "Banks - Diversified", props(sector = "Financial Services"))]
    BanksDiversified,
    #[strum(serialize = "Banks - Regional", props(sector = "Financial Services"))]
    BanksRegional,
    #[strum(serialize = "Mortgage Finance", props(sector = "Financial Services"))]
    MortgageFinance,
    #[strum(serialize = "Capital Markets", props(sector = "Financial Services"))]
    CapitalMarkets,
    #[strum(serialize = "Financial Data & Stock Exchanges", props(sector = "Financial Services"))]
    FinancialDataAndStockExchanges,
    #[strum(serialize = "Insurance - Life", props(sector = "Financial Services"))]
    InsuranceLife,
    #[strum(serialize = "Insurance - Property & Casualty", props(sector = "Financial Services"))]
    InsurancePropertyAndCasualty,
    #[strum(serialize = "Insurance - Reinsurance", props(sector = "Financial Services"))]
    InsuranceReinsurance,
    #[strum(serialize = "Insurance - Specialty", props(sector = "Financial Services"))]
    InsuranceSpecialty,
    #[strum(serialize = "Insurance Brokers", props(sector = "Financial Services"))]
    InsuranceBrokers,
    #[strum(serialize = "Insurance - Diversified", props(sector = "Financial Services"))]
    InsuranceDiversified,
    #[strum(serialize = "Shell Companies", props(sector = "Financial Services"))]
    ShellCompanies,
    #[strum(serialize = "Financial Conglomerates", props(sector = "Financial Services"))]
    FinancialConglomerates,
    #[strum(serialize = "Credit Services", props(sector = "Financial Services"))]
    CreditServices,

    // Real Estate
    #[strum(serialize = "Real Estate - Development", props(sector = "Real Estate"))]
    RealEstateDevelopment,
    #[strum(serialize = "Real Estate Services", props(sector = "Real Estate"))]
    RealEstateServices,
    #[strum(serialize = "Real Estate - Diversified", props(sector = "Real Estate"))]
    RealEstateDiversified,
    #[strum(serialize = "REIT - Healthcare Facilities", props(sector = "Real Estate"))]
    ReitHealthcareFacilities,
    #[strum(serialize = "REIT - Hotel & Motel", props(sector = "Real Estate"))]
    ReitHotelAndMotel,
    #[strum(serialize = "REIT - Industrial", props(sector = "Real Estate"))]
    ReitIndustrial,
    #[strum(serialize = "REIT - Office", props(sector = "Real Estate"))]
    ReitOffice,
    #[strum(serialize = "REIT - Residential", props(sector = "Real Estate"))]
    ReitResidential,
    #[strum(serialize = "REIT - Retail", props(sector = "Real Estate"))]
    ReitRetail,
    #[strum(serialize = "REIT - Mortgage", props(sector = "Real Estate"))]
    ReitMortgage,
    #[strum(serialize = "REIT - Specialty", props(sector = "Real Estate"))]
    ReitSpecialty,
    #[strum(serialize = "REIT - Diversified", props(sector = "Real Estate"))]
    ReitDiversified,

    // Healthcare
    #[strum(serialize = "Biotechnology", props(sector = "Healthcare"))]
    Biotechnology,
    #[strum(serialize = "Drug Manufacturers - General", props(sector = "Healthcare"))]
    DrugManufacturersGeneral,
    #[strum(serialize = "Drug Manufacturers - Specialty & Generic", props(sector = "Healthcare"))]
    DrugManufacturersSpecialtyAndGeneric,
    #[strum(serialize = "Healthcare Plans", props(sector = "Healthcare"))]
    HealthcarePlans,
    #[strum(serialize = "Medical Care Facilities", props(sector = "Healthcare"))]
    MedicalCareFacilities,
    #[strum(serialize = "Pharmaceutical Retailers", props(sector = "Healthcare"))]
    PharmaceuticalRetailers,
    #[strum(serialize = "Health Information Services", props(sector = "Healthcare"))]
    HealthInformationServices,
    #[strum(serialize = "Medical Devices", props(sector = "Healthcare"))]
    MedicalDevices,
    #[strum(serialize = "Medical Instruments & Supplies", props(sector = "Healthcare"))]
    MedicalInstrumentsAndSupplies,
    #[strum(serialize = "Diagnostics & Research", props(sector = "Healthcare"))]
    DiagnosticsAndResearch,
    #[strum(serialize = "Medical Distribution", props(sector = "Healthcare"))]
    MedicalDistribution,

    // Energy
    #[strum(serialize = "Oil & Gas Drilling", props(sector = "Energy"))]
    OilAndGasDrilling,
    #[strum(serialize = "Oil & Gas E&P", props(sector = "Energy"))]
    OilAndGasEP,
    #[strum(serialize = "Oil & Gas Integrated", props(sector = "Energy"))]
    OilAndGasIntegrated,
    #[strum(serialize = "Oil & Gas Midstream", props(sector = "Energy"))]
    OilAndGasMidstream,
    #[strum(serialize = "Oil & Gas Refining & Marketing", props(sector = "Energy"))]
    OilAndGasRefiningAndMarketing,
    #[strum(serialize = "Oil & Gas Equipment & Services", props(sector = "Energy"))]
    OilAndGasEquipmentAndServices,
    #[strum(serialize = "Thermal Coal", props(sector = "Energy"))]
    ThermalCoal,
    #[strum(serialize = "Uranium", props(sector = "Energy"))]
    Uranium,

    // Utilities
    #[strum(serialize = "Utilities - Independent Power Producers", props(sector = "Utilities"))]
    UtilitiesIndependentPowerProducers,
    #[strum(serialize = "Utilities - Renewable", props(sector = "Utilities"))]
    UtilitiesRenewable,
    #[strum(serialize = "Utilities - Regulated Water", props(sector = "Utilities"))]
    UtilitiesRegulatedWater,
    #[strum(serialize = "Utilities - Regulated Electric", props(sector = "Utilities"))]
    UtilitiesRegulatedElectric,
    #[strum(serialize = "Utilities - Regulated Gas", props(sector = "Utilities"))]
    UtilitiesRegulatedGas,
    #[strum(serialize = "Utilities - Diversified", props(sector = "Utilities"))]
    UtilitiesDiversified,

    // Communication Services
    #[strum(serialize = "Telecom Services", props(sector = "Communication Services"))]
    TelecomServices,
    #[strum(serialize = "Advertising Agencies", props(sector = "Communication Services"))]
    AdvertisingAgencies,
    #[strum(serialize = "Publishing", props(sector = "Communication Services"))]
    Publishing,
    #[strum(serialize = "Broadcasting", props(sector = "Communication Services"))]
    Broadcasting,
    #[strum(serialize = "Entertainment", props(sector = "Communication Services"))]
    Entertainment,
    #[strum(serialize = "Internet Content & Information", props(sector = "Communication Services"))]
    InternetContentAndInformation,
    #[strum(serialize = "Electronic Gaming & Multimedia", props(sector = "Communication Services"))]
    ElectronicGamingAndMultimedia,

    // Technology
    #[strum(serialize = "Information Technology Services", props(sector = "Technology"))]
    InformationTechnologyServices,
    #[strum(serialize = "Software - Application", props(sector = "Technology"))]
    SoftwareApplication,
    #[strum(serialize = "Software - Infrastructure", props(sector = "Technology"))]
    SoftwareInfrastructure,
    #[strum(serialize = "Communication Equipment", props(sector = "Technology"))]
    CommunicationEquipment,
    #[strum(serialize = "Computer Hardware", props(sector = "Technology"))]
    ComputerHardware,
    #[strum(serialize = "Consumer Electronics", props(sector = "Technology"))]
    ConsumerElectronics,
    #[strum(serialize = "Electronic Components", props(sector = "Technology"))]
    ElectronicComponents,
    #[strum(serialize = "Electronics & Computer Distribution", props(sector = "Technology"))]
    ElectronicsAndComputerDistribution,
    #[strum(serialize = "Scientific & Technical Instruments", props(sector = "Technology"))]
    ScientificAndTechnicalInstruments,
    #[strum(serialize = "Semiconductor Equipment & Materials", props(sector = "Technology"))]
    SemiconductorEquipmentAndMaterials,
    #[strum(serialize = "Semiconductors", props(sector = "Technology"))]
    Semiconductors,
    #[strum(serialize = "Solar", props(sector = "Technology"))]
    Solar,
}

impl Industry {
    pub fn sector(&self) -> &'static str {
        self.get_str("sector").unwrap()
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, AsRefStr, IntoStaticStr, EnumIter, VariantNames)]
pub enum FundCategory {
    #[strum(serialize = "Allocation - 15% to 30% Equity")]
    Allocation15to30Equity,
    #[strum(serialize = "Allocation - 30% to 50% Equity")]
    Allocation30to50Equity,
    #[strum(serialize = "Allocation - 50% to 70% Equity")]
    Allocation50to70Equity,
    #[strum(serialize = "Allocation - 70% to 85% Equity")]
    Allocation70to85Equity,
    #[strum(serialize = "Allocation - 85%+ Equity")]
    Allocation85PlusEquity,
    #[strum(serialize = "Bank Loan")]
    BankLoan,
    #[strum(serialize = "Bear Market")]
    BearMarket,
    #[strum(serialize = "China Region")]
    ChinaRegion,
    #[strum(serialize = "Commodities Agriculture")]
    CommoditiesAgriculture,
    #[strum(serialize = "Commodities Broad Basket")]
    CommoditiesBroadBasket,
    #[strum(serialize = "Convertibles")]
    Convertibles,
    #[strum(serialize = "Corporate Bond")]
    CorporateBond,
    #[strum(serialize = "Diversified Emerging Mkts")]
    DiversifiedEmergingMkts,
    #[strum(serialize = "Diversified Pacific/Asia")]
    DiversifiedPacificAsia,
    #[strum(serialize = "Emerging Markets Bond")]
    EmergingMarketsBond,
    #[strum(serialize = "Emerging-Markets Local-Currency Bond")]
    EmergingMarketsLocalCurrencyBond,
    #[strum(serialize = "Energy Limited Partnership")]
    EnergyLimitedPartnership,
    #[strum(serialize = "Equity Energy")]
    EquityEnergy,
    #[strum(serialize = "Equity Precious Metals")]
    EquityPreciousMetals,
    #[strum(serialize = "Europe Stock")]
    EuropeStock,
    #[strum(serialize = "Financial")]
    Financial,
    #[strum(serialize = "Foreign Large Blend")]
    ForeignLargeBlend,
    #[strum(serialize = "Foreign Large Growth")]
    ForeignLargeGrowth,
    #[strum(serialize = "Foreign Large Value")]
    ForeignLargeValue,
    #[strum(serialize = "Foreign Small/Mid Blend")]
    ForeignSmallMidBlend,
    #[strum(serialize = "Foreign Small/Mid Growth")]
    ForeignSmallMidGrowth,
    #[strum(serialize = "Foreign Small/Mid Value")]
    ForeignSmallMidValue,
    #[strum(serialize = "Global Real Estate")]
    GlobalRealEstate,
    #[strum(serialize = "Health")]
    Health,
    #[strum(serialize = "High Yield Bond")]
    HighYieldBond,
    #[strum(serialize = "High Yield Muni")]
    HighYieldMuni,
    #[strum(serialize = "Inflation-Protected Bond")]
    InflationProtectedBond,
    #[strum(serialize = "Infrastructure")]
    Infrastructure,
    #[strum(serialize = "Intermediate Government")]
    IntermediateGovernment,
    #[strum(serialize = "Intermediate-Term Bond")]
    IntermediateTermBond,
    #[strum(serialize = "Japan Stock")]
    JapanStock,
    #[strum(serialize = "Large Blend")]
    LargeBlend,
    #[strum(serialize = "Large Growth")]
    LargeGrowth,
    #[strum(serialize = "Large Value")]
    LargeValue,
    #[strum(serialize = "Long Government")]
    LongGovernment,
    #[strum(serialize = "Long-Short Credit")]
    LongShortCredit,
    #[strum(serialize = "Long-Short Equity")]
    LongShortEquity,
    #[strum(serialize = "Long-Term Bond")]
    LongTermBond,
    #[strum(serialize = "Managed Futures")]
    ManagedFutures,
    #[strum(serialize = "Market Neutral")]
    MarketNeutral,
    #[strum(serialize = "Mid-Cap Blend")]
    MidCapBlend,
    #[strum(serialize = "Mid-Cap Growth")]
    MidCapGrowth,
    #[strum(serialize = "Mid-Cap Value")]
    MidCapValue,
    #[strum(serialize = "Miscellaneous Region")]
    MiscellaneousRegion,
    #[strum(serialize = "Multialternative")]
    Multialternative,
    #[strum(serialize = "Multicurrency")]
    Multicurrency,
    #[strum(serialize = "Multisector Bond")]
    MultisectorBond,
    #[strum(serialize = "Muni California Intermediate")]
    MuniCaliforniaIntermediate,
    #[strum(serialize = "Muni California Long")]
    MuniCaliforniaLong,
    #[strum(serialize = "Muni Massachusetts")]
    MuniMassachusetts,
    #[strum(serialize = "Muni Minnesota")]
    MuniMinnesota,
    #[strum(serialize = "Muni National Interm")]
    MuniNationalInterm,
    #[strum(serialize = "Muni National Long")]
    MuniNationalLong,
    #[strum(serialize = "Muni National Short")]
    MuniNationalShort,
    #[strum(serialize = "Muni New Jersey")]
    MuniNewJersey,
    #[strum(serialize = "Muni New York Intermediate")]
    MuniNewYorkIntermediate,
    #[strum(serialize = "Muni New York Long")]
    MuniNewYorkLong,
    #[strum(serialize = "Muni Ohio")]
    MuniOhio,
    #[strum(serialize = "Muni Pennsylvania")]
    MuniPennsylvania,
    #[strum(serialize = "Muni Single State Interm")]
    MuniSingleStateInterm,
    #[strum(serialize = "Muni Single State Long")]
    MuniSingleStateLong,
    #[strum(serialize = "Muni Single State Short")]
    MuniSingleStateShort,
    #[strum(serialize = "Natural Resources")]
    NaturalResources,
    #[strum(serialize = "Nontraditional Bond")]
    NontraditionalBond,
    #[strum(serialize = "Option Writing")]
    OptionWriting,
    #[strum(serialize = "Other")]
    Other,
    #[strum(serialize = "Other Allocation")]
    OtherAllocation,
    #[strum(serialize = "Pacific/Asia ex-Japan Stk")]
    PacificAsiaExJapanStk,
    #[strum(serialize = "Preferred Stock")]
    PreferredStock,
    #[strum(serialize = "Real Estate")]
    RealEstate,
    #[strum(serialize = "Short Government")]
    ShortGovernment,
    #[strum(serialize = "Short-Term Bond")]
    ShortTermBond,
    #[strum(serialize = "Small Blend")]
    SmallBlend,
    #[strum(serialize = "Small Growth")]
    SmallGrowth,
    #[strum(serialize = "Small Value")]
    SmallValue,
    #[strum(serialize = "Tactical Allocation")]
    TacticalAllocation,
    #[strum(serialize = "Target-Date 2000-2010")]
    TargetDate20002010,
    #[strum(serialize = "Target-Date 2015")]
    TargetDate2015,
    #[strum(serialize = "Target-Date 2020")]
    TargetDate2020,
    #[strum(serialize = "Target-Date 2025")]
    TargetDate2025,
    #[strum(serialize = "Target-Date 2030")]
    TargetDate2030,
    #[strum(serialize = "Target-Date 2035")]
    TargetDate2035,
    #[strum(serialize = "Target-Date 2040")]
    TargetDate2040,
    #[strum(serialize = "Target-Date 2045")]
    TargetDate2045,
    #[strum(serialize = "Target-Date 2050")]
    TargetDate2050,
    #[strum(serialize = "Target-Date 2055")]
    TargetDate2055,
    #[strum(serialize = "Target-Date 2060+")]
    TargetDate2060Plus,
    #[strum(serialize = "Target-Date Retirement")]
    TargetDateRetirement,
    #[strum(serialize = "Technology")]
    Technology,
    #[strum(serialize = "Trading - Leveraged/Inverse Commodities")]
    TradingLeveragedInverseCommodities,
    #[strum(serialize = "Trading - Leveraged/Inverse Equity")]
    TradingLeveragedInverseEquity,
    #[strum(serialize = "Trading - Inverse Equity")]
    TradingInverseEquity,
    #[strum(serialize = "Trading - Leveraged Equity")]
    TradingLeveragedEquity,
    #[strum(serialize = "Ultrashort Bond")]
    UltrashortBond,
    #[strum(serialize = "Utilities")]
    Utilities,
    #[strum(serialize = "World Allocation")]
    WorldAllocation,
    #[strum(serialize = "World Bond")]
    WorldBond,
    #[strum(serialize = "World Stock")]
    WorldStock,
}
