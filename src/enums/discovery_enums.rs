use api_request_utils_rs::serde::{Deserialize,Serialize};

/// Represents the aspect ratio of an image.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum ImageRatio {
    /// 16:9 aspect ratio.
    #[serde(rename = "16_9")]
    SixteenNine,
    /// 3:2 aspect ratio.
    #[serde(rename = "3_2")]
    ThreeTwo,
    /// 4:3 aspect ratio.
    #[serde(rename = "4_3")]
    FourThree,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum Decision {
    Yes,
    No,
    Only
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum Source {
    Ticketmaster,
    Universe,
    Frontgate,
    Tmr,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum Unit {
    Miles,
    Km,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum SortingOrder {
    #[serde(rename = "name,asc")]
    NameAsc,
    #[serde(rename = "name,desc")]
    NameDesc,
    #[serde(rename = "date,asc")]
    DateAsc,
    #[serde(rename = "date,desc")]
    DateDesc,
    #[serde(rename = "relevance,asc")]
    RelevanceAsc,
    #[serde(rename = "relevance,desc")]
    RelevanceDesc,
    #[serde(rename = "distance,asc")]
    DistanceAsc,
    #[serde(rename = "name,date,asc")]
    NameDateAsc,
    #[serde(rename = "name,date,desc")]
    NameDateDesc,
    #[serde(rename = "date,name,asc")]
    DateNameAsc,
    #[serde(rename = "date,name,desc")]
    DateNameDesc,
    #[serde(rename = "distance,date,asc")]
    DistanceDateAsc,
    #[serde(rename = "onSaleStartDate,asc")]
    OnSaleStartDateAsc,
    #[serde(rename = "id,asc")]
    IdAsc,
    #[serde(rename = "venueName,asc")]
    VenueNameAsc,
    #[serde(rename = "venueName,desc")]
    VenueNameDesc,
    #[serde(rename = "random")]
    Random,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum ClassificationName {
    Segment,
    Genre,
    SubGenre,
    Type,
    SubType,
}
