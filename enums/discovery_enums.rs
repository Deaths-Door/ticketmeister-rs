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
#[serde(rename_all = "lowercase")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum Unit {
    Miles,
    Km,
}
