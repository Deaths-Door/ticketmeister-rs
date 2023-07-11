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


/// Total number of available elements in server
#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Page {
    /// Size of page.
    size : u16,

    /// Total number of available elements in server
    #[serde(rename="totalElements")]
    total_elements : u16,

    /// Total number of available elements in server
    #[serde(rename="totalPages")]
    total_pages : u16,

    /// Current page number (counted from 0)
    #[serde(rename="number")]
    current_page : u16
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
struct Dma {
    id: u16,
    name: &'static str,
}

impl Dma {
    const fn new(id : u16,name : &'static str) -> Self {
        Self { id , name }
    }
    
    pub fn id(&self) -> &u16 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

static DMA_ALL_OF_US: Dma = Dma::new(200, "All of US");
static DMA_ABILENE_SWEETWATER: Dma = Dma::new(212, "Abilene - Sweetwater");
static DMA_ALBANY_SCHENECTADY_TROY: Dma = Dma::new(213, "Albany - Schenectady - Troy");
static DMA_ALBANY_GA: Dma = Dma::new(214, "Albany, GA");
static DMA_ALBUQUERQUE_SANTA_FE: Dma = Dma::new(215, "Albuquerque - Santa Fe");
static DMA_ALEXANDRIA_LA: Dma = Dma::new(216, "Alexandria, LA");
static DMA_ALPENA: Dma = Dma::new(217, "Alpena");
static DMA_AMARILLO: Dma = Dma::new(218, "Amarillo");
static DMA_ANCHORAGE: Dma = Dma::new(219, "Anchorage");
static DMA_ATLANTA: Dma = Dma::new(220, "Atlanta");
static DMA_AUGUSTA: Dma = Dma::new(221, "Augusta");
static DMA_AUSTIN: Dma = Dma::new(222, "Austin");
static DMA_BAKERSFIELD: Dma = Dma::new(223, "Bakersfield");
static DMA_BALTIMORE: Dma = Dma::new(224, "Baltimore");
static DMA_BANGOR: Dma = Dma::new(225, "Bangor");
static DMA_BATON_ROUGE: Dma = Dma::new(226, "Baton Rouge");
static DMA_BEAUMONT_PORT_ARTHUR: Dma = Dma::new(227, "Beaumont - Port Arthur");
static DMA_BEND_OR: Dma = Dma::new(228, "Bend, OR");
static DMA_BILLINGS: Dma = Dma::new(229, "Billings");
static DMA_BILOXI_GULFPORT: Dma = Dma::new(230, "Biloxi - Gulfport");