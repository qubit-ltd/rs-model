//! Demographic and social classification values.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Source-domain Blood classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Blood {
    /// Source variant `TYPE_A`.
    TypeA,
    /// Source variant `TYPE_B`.
    TypeB,
    /// Source variant `TYPE_AB`.
    TypeAb,
    /// Source variant `TYPE_O`.
    TypeO,
    /// Source variant `UNKNOWN`.
    Unknown,
}

/// Source-domain Education classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Education {
    /// Source variant `NONE`.
    None,
    /// Source variant `ELEMENTARY_SCHOOL`.
    ElementarySchool,
    /// Source variant `JUNIOR_HIGH_SCHOOL`.
    JuniorHighSchool,
    /// Source variant `SENIOR_HIGH_SCHOOL`.
    SeniorHighSchool,
    /// Source variant `VOCATIONAL_SENIOR_HIGH_SCHOOL`.
    VocationalSeniorHighSchool,
    /// Source variant `SECONDARY_VOCATIONAL_SCHOOL`.
    SecondaryVocationalSchool,
    /// Source variant `COLLEGE`.
    College,
    /// Source variant `BACHELOR`.
    Bachelor,
    /// Source variant `MASTER`.
    Master,
    /// Source variant `DOCTOR`.
    Doctor,
}

/// Source-domain Ethnic classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ethnic {
    /// Source variant `HAN`.
    Han,
    /// Source variant `MONGOL`.
    Mongol,
    /// Source variant `HUI`.
    Hui,
    /// Source variant `TIBETAN`.
    Tibetan,
    /// Source variant `UYGHUR`.
    Uyghur,
    /// Source variant `MIAO`.
    Miao,
    /// Source variant `YI`.
    Yi,
    /// Source variant `ZHUANG`.
    Zhuang,
    /// Source variant `BUYEI`.
    Buyei,
    /// Source variant `CHOSEN`.
    Chosen,
    /// Source variant `MAN`.
    Man,
    /// Source variant `DONG`.
    Dong,
    /// Source variant `YAO`.
    Yao,
    /// Source variant `BAI`.
    Bai,
    /// Source variant `TUJIA`.
    Tujia,
    /// Source variant `HANI`.
    Hani,
    /// Source variant `KAZAK`.
    Kazak,
    /// Source variant `DAI`.
    Dai,
    /// Source variant `LI`.
    Li,
    /// Source variant `LISU`.
    Lisu,
    /// Source variant `VA`.
    Va,
    /// Source variant `SHE`.
    She,
    /// Source variant `LAHU`.
    Lahu,
    /// Source variant `SUI`.
    Sui,
    /// Source variant `DONGXIANG`.
    Dongxiang,
    /// Source variant `NAXI`.
    Naxi,
    /// Source variant `JINGPO`.
    Jingpo,
    /// Source variant `KIRGIZ`.
    Kirgiz,
    /// Source variant `TU`.
    Tu,
    /// Source variant `DAUR`.
    Daur,
    /// Source variant `MULAO`.
    Mulao,
    /// Source variant `QIANG`.
    Qiang,
    /// Source variant `BLANG`.
    Blang,
    /// Source variant `SALAR`.
    Salar,
    /// Source variant `MAONAN`.
    Maonan,
    /// Source variant `GELAO`.
    Gelao,
    /// Source variant `XIBE`.
    Xibe,
    /// Source variant `ACHANG`.
    Achang,
    /// Source variant `PUMI`.
    Pumi,
    /// Source variant `TAJIK`.
    Tajik,
    /// Source variant `NU`.
    Nu,
    /// Source variant `UZBEK`.
    Uzbek,
    /// Source variant `RUSS`.
    Russ,
    /// Source variant `EWENKI`.
    Ewenki,
    /// Source variant `DEANG`.
    Deang,
    /// Source variant `BONAN`.
    Bonan,
    /// Source variant `YUGUR`.
    Yugur,
    /// Source variant `GIN`.
    Gin,
    /// Source variant `TATAR`.
    Tatar,
    /// Source variant `DERUNG`.
    Derung,
    /// Source variant `OROQEN`.
    Oroqen,
    /// Source variant `HEZHEN`.
    Hezhen,
    /// Source variant `MONBA`.
    Monba,
    /// Source variant `LHOBA`.
    Lhoba,
    /// Source variant `JINO`.
    Jino,
    /// Source variant `GAOSHAN`.
    Gaoshan,
    /// Source variant `FOREIGNER`.
    Foreigner,
    /// Source variant `OTHER`.
    Other,
}

/// Source-domain Gender classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    /// Source variant `UNKNOWN`.
    Unknown,
    /// Source variant `MALE`.
    Male,
    /// Source variant `FEMALE`.
    Female,
    /// Source variant `UNSPECIFIED`.
    Unspecified,
}

/// Source-domain Incoming classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Incoming {
    /// Source variant `ANNUAL_25K_BELOW`.
    Annual25kBelow,
    /// Source variant `ANNUAL_25K_50K`.
    Annual25k50k,
    /// Source variant `ANNUAL_50K_100K`.
    Annual50k100k,
    /// Source variant `ANNUAL_100K_150K`.
    Annual100k150k,
    /// Source variant `ANNUAL_150K_200K`.
    Annual150k200k,
    /// Source variant `ANNUAL_200K_300K`.
    Annual200k300k,
    /// Source variant `ANNUAL_300K_400K`.
    Annual300k400k,
    /// Source variant `ANNUAL_400K_500K`.
    Annual400k500k,
    /// Source variant `ANNUAL_500K_800K`.
    Annual500k800k,
    /// Source variant `ANNUAL_800K_1000K`.
    Annual800k1000k,
    /// Source variant `ANNUAL_1000K_5000K`.
    Annual1000k5000k,
    /// Source variant `ANNUAL_5000K_10000K`.
    Annual5000k10000k,
    /// Source variant `ANNUAL_10000K_ABOVE`.
    Annual10000kAbove,
}

/// Source-domain Industry classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Industry {
    /// Source variant `AGRICULTURE_FORESTRY_PASTORAL_FISHERY`.
    AgricultureForestryPastoralFishery,
    /// Source variant `MINING`.
    Mining,
    /// Source variant `MANUFACTURING`.
    Manufacturing,
    /// Source variant `ELECTRICITY_HEAT_GAS_AND_WATER`.
    ElectricityHeatGasAndWater,
    /// Source variant `CONSTRUCTION`.
    Construction,
    /// Source variant `SALES`.
    Sales,
    /// Source variant `TRANSPORTATION_WAREHOUSING_AND_POSTAL`.
    TransportationWarehousingAndPostal,
    /// Source variant `ACCOMMODATION_AND_CATERING`.
    AccommodationAndCatering,
    /// Source variant `INFORMATION`.
    Information,
    /// Source variant `FINANCE`.
    Finance,
    /// Source variant `REAL_ESTATE`.
    RealEstate,
    /// Source variant `LEASING_AND_BUSINESS_SERVICES`.
    LeasingAndBusinessServices,
    /// Source variant `RESEARCH_AND_TECHNOLOGY`.
    ResearchAndTechnology,
    /// Source variant `CONSERVANCY_ENVIRONMENT_AND_PUBLIC_FACILITIES`.
    ConservancyEnvironmentAndPublicFacilities,
    /// Source variant `RESIDENTIAL_SERVICES`.
    ResidentialServices,
    /// Source variant `EDUCATION`.
    Education,
    /// Source variant `HEALTH_AND_SOCIAL_WORK`.
    HealthAndSocialWork,
    /// Source variant `CULTURE_SPORTS_AND_ENTERTAINMENT`.
    CultureSportsAndEntertainment,
    /// Source variant `PUBLIC_ADMIN_SOCIAL_SECURITY_AND_SOCIAL_ORGANIZATIONS`.
    PublicAdminSocialSecurityAndSocialOrganizations,
    /// Source variant `INTERNATIONAL_ORGANIZATIONS`.
    InternationalOrganizations,
}

/// Source-domain JobTitle classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobTitle {
    /// Source variant `FREELANCE`.
    Freelance,
    /// Source variant `EMPLOYEE`.
    Employee,
    /// Source variant `JUNIOR_TITLE`.
    JuniorTitle,
    /// Source variant `MIDDLE_TITLE`.
    MiddleTitle,
    /// Source variant `SENIOR_TITLE`.
    SeniorTitle,
    /// Source variant `JUNIOR_MANAGER`.
    JuniorManager,
    /// Source variant `MIDDLE_MANAGER`.
    MiddleManager,
    /// Source variant `SENIOR_MANAGER`.
    SeniorManager,
    /// Source variant `OWNER`.
    Owner,
}

/// Source-domain Marriage classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Marriage {
    /// Source variant `UNMARRIED`.
    Unmarried,
    /// Source variant `MARRIED`.
    Married,
    /// Source variant `MARRIED_FIRST_TIME`.
    MarriedFirstTime,
    /// Source variant `MARRIED_AGAIN`.
    MarriedAgain,
    /// Source variant `MARRIED_RESTORED`.
    MarriedRestored,
    /// Source variant `WIDOWED`.
    Widowed,
    /// Source variant `DIVORCED`.
    Divorced,
    /// Source variant `SEPARATED`.
    Separated,
    /// Source variant `UNPROVIDED`.
    Unprovided,
}

/// Source-domain Politics classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Politics {
    /// Source variant `COMMUNIST_PARTY_MEMBER`.
    CommunistPartyMember,
    /// Source variant `COMMUNIST_PARTY_PROBATIONARY_MEMBER`.
    CommunistPartyProbationaryMember,
    /// Source variant `COMMUNIST_YOUTH_LEAGUE_MEMBER`.
    CommunistYouthLeagueMember,
    /// Source variant `KUOMINTANG_MEMBER`.
    KuomintangMember,
    /// Source variant `DEMOCRATIC_LEAGUE_MEMBER`.
    DemocraticLeagueMember,
    /// Source variant `NATIONAL_DEMOCRATIC_CONSTRUCTION_ASSOCIATION_MEMBER`.
    NationalDemocraticConstructionAssociationMember,
    /// Source variant `PROMOTING_DEMOCRACY_ASSOCIATION_MEMBER`.
    PromotingDemocracyAssociationMember,
    /// Source variant `PEASANTS_WORKERS_DEMOCRATIC_PARTY_MEMBER`.
    PeasantsWorkersDemocraticPartyMember,
    /// Source variant `ZHI_GONG_PARTY_MEMBER`.
    ZhiGongPartyMember,
    /// Source variant `NINE_THREE_ACADEMIC_SOCIETY_MEMBER`.
    NineThreeAcademicSocietyMember,
    /// Source variant `TAIWAN_DEMOCRATIC_SELF_GOVERNMENT_LEAGUE_MEMBER`.
    TaiwanDemocraticSelfGovernmentLeagueMember,
    /// Source variant `INDEPENDENT_POLITICIAN`.
    IndependentPolitician,
    /// Source variant `MASSES`.
    Masses,
}

/// Source-domain Religion classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Religion {
    /// Source variant `NONE`.
    None,
    /// Source variant `CONFUCIANISM`.
    Confucianism,
    /// Source variant `TAOISM`.
    Taoism,
    /// Source variant `BUDDHISM`.
    Buddhism,
    /// Source variant `SHINTO`.
    Shinto,
    /// Source variant `CHRISTIANITY`.
    Christianity,
    /// Source variant `JUDAISM`.
    Judaism,
    /// Source variant `ISLAM`.
    Islam,
}

/// Source-domain SexOrientation classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SexOrientation {
    /// Source variant `HETEROSEXUAL`.
    Heterosexual,
    /// Source variant `HOMOSEXUAL`.
    Homosexual,
    /// Source variant `BISEXUAL`.
    Bisexual,
    /// Source variant `SECRECY`.
    Secrecy,
}

/// Source-domain SocialNetwork classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SocialNetwork {
    /// Source variant `WECHAT`.
    Wechat,
    /// Source variant `SINA`.
    Sina,
    /// Source variant `ZHIHU`.
    Zhihu,
    /// Source variant `DOUYIN`.
    Douyin,
    /// Source variant `BILIBILI`.
    Bilibili,
}
