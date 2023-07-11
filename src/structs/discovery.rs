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

/// Represents information about an entity.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Info {
    /// The ID of the entity.
    pub id: String,
    /// The Name of the entity.
    pub name: String,
    /// Locale in which the content is returned.
    pub locale: String,
}

/// Represents a sub-genre.
#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct SubGenre(Info);

/// Represents a genre.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Genre {
    /// Information about the genre.
    #[serde(flatten)]
    pub info: Info,
    /// List of Tertiary Genre linked to the Secondary Genre
    #[serde(rename = "subGenres")]
    pub sub_genres: Vec<SubGenre>,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Represents a segment.
pub struct Segment(Info);