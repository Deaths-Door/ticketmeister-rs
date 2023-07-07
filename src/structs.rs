use api_request_utils_rs::serde::{Deserialize,Serialize};

use crate::ImageRatio;

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct ItemInfo<'a> {
    id: &'a str,
    name: &'a str,
    locale: &'a str,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct SubGenre<'a>(#[serde(borrow)] ItemInfo<'a>);

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Genre<'a> {
    #[serde(borrow)]
    pub info : ItemInfo<'a>,

    #[serde(rename = "subGenres")]
    pub sub_genres : Vec<SubGenre<'a>>
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Segment<'a>(#[serde(borrow)] ItemInfo<'a>);

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
struct ImageInfo<'a> {
    url: &'a str,
    ratio: ImageRatio,
    width: u32,
    height: u32,
    fallback: bool,
    attribution: &'a str,
}