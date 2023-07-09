use api_request_utils_rs::serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
/// Represents the aspect ratio of an image.
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
#[serde(crate = "api_request_utils_rs::serde")]
/// Represents a unit of measurement for distance.
pub enum Unit {
    /// Kilometer unit.
    #[serde(rename = "km")]
    Kilometer,
    /// Miles unit.
    #[serde(rename = "miles")]
    Miles,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(crate = "api_request_utils_rs::serde")]
/// Represents the status code for an event.
pub enum StatusCode {
    /// The event is currently on sale.
    OnSale,
    /// The event is currently off sale.
    OffSale,
    /// The event has been canceled.
    Canceled,
    /// The event has been postponed.
    Postponed,
    /// The event has been rescheduled.
    Rescheduled,
}
