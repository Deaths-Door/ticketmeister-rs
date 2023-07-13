use api_request_utils_rs::serde::{Deserialize,Serialize};

use crate::{
    ImageRatio
};

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Image Information
pub struct Image {
    /// Public URL of the image
    pub url: String,
    /// Aspect ratio of the image
    pub ratio: ImageRatio,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
    /// true if the image is not the event's image but a fallbak image
    pub fallback: bool,
    /// Attribution of the image
    pub attribution: Option<String>,
}