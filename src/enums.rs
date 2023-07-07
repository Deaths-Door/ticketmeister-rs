use strum::Display;
use api_request_utils_rs::serde::{Deserialize,Serialize};

#[derive(Display)]
#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub enum ImageRatio {
    #[strum(serialize = "16_9")]
    SixteenNine,
    #[strum(serialize = "3_2")]
    ThreeTwo,
    #[strum(serialize = "4_3")]
    FourThree,
}
