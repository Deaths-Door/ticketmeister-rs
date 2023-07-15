use api_request_utils_rs::serde::{Deserialize,Serialize};

use crate::{
    Market,
    Dma,
    ImageRatio,
    EventStatus
};

/// Total number of available elements in server
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Page {
    /// Size of page.
    pub size : u16,

    /// Total number of available elements in server
    #[serde(rename="totalElements")]
    pub total_elements : u16,

    /// Total number of available elements in server
    #[serde(rename="totalPages")]
    pub total_pages : u16,

    /// Current page number (counted from 0)
    #[serde(rename="number")]
    pub current_page : u16
}

/// Represents information about an entity.
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
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

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Event {
    #[serde(flatten)]
    pub info : Info,

    #[serde(rename ="test")]
    pub is_test: bool,

    pub url: String,

    pub images: Vec<Image>,

    pub sales: Sales,

    pub dates: Dates,

    pub classifications: Vec<Classification>,

    pub promoter: Promoter,

    pub promoters: Vec<Promoter>,

    #[serde(rename = "pleaseNote")]
    pub please_note: String,

    #[serde(rename = "priceRanges")]
    pub price_ranges: Vec<PriceRange>,

    pub products: Vec<Product>,

    #[serde(rename = "seatmap/static_url")]
    pub seatmap: String,

    pub accessibility: Accessibility,

    #[serde(rename = "ticketLimit/info")]
    pub ticket_limit_infomation: String,

    #[serde(rename = "ageRestrictions/legalAgeEnforced")]
    pub is_legal_age_enforced: bool,

    pub ticketing: Ticketing,

    #[serde(rename = "_embedded/venues")]
    pub venues: Vec<Venue>,

    #[serde(rename = "_embedded/attractions")]
    pub attractions: Vec<Attraction>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
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
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Sales {
    pub public: Public,
    pub presales: Vec<Presale>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Presale {
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,

    #[serde(rename = "endDateTime")]
    pub end_date_time: String,

    pub name: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Public {
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,

    #[serde(rename = "startTBD")]
    pub start_tbd: bool,

    #[serde(rename = "startTBA")]
    pub start_tba: bool,

    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Dates {
    pub start: Start,

    pub timezone: String,

    #[serde(rename = "status/code")]
    pub status: EventStatus,

    #[serde(rename = "spanMultipleDays")]
    pub span_multiple_days: bool,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Start {
    #[serde(rename = "localDate")]
    pub local_date: String,

    #[serde(rename = "localTime")]
    pub local_time: String,

    #[serde(rename = "dateTime")]
    pub date_time: String,

    #[serde(rename = "dateTBD")]
    pub date_tbd: bool,

    #[serde(rename = "dateTBA")]
    pub date_tba: bool,

    #[serde(rename = "timeTBA")]
    pub time_tba: bool,

    #[serde(rename = "noSpecificTime")]
    pub no_specific_time: bool,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Classification {
    #[serde(rename = "primary")]
    pub is_primary: bool,

    pub segment: Genre,

    pub genre: Genre,

    #[serde(rename = "subGenre")]
    pub sub_genre: Genre,

    pub r#type : Type,

    #[serde(rename = "subType")]
    pub sub_type: Genre,

    #[serde(rename = "family")]
    pub is_family: bool,
}


/// Represents a sub-genre.
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct SubGenre(pub Info);

/// Represents a genre.
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Genre {
    /// Information about the genre.
    #[serde(flatten)]
    pub info: Info,
    /// List of Tertiary Genre linked to the Secondary Genre
    #[serde(rename = "subGenres")]
    pub sub_genres: Vec<SubGenre>,
}

/// Represents a segment.
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Segment(pub Info);

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Type {
    #[serde(flatten)]
    pub info: Info,
    
    #[serde(rename = "subTypes")]
    pub sub_types: Vec<SubType>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct SubType(Info);

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Promoter {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct PriceRange {
    pub currency: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Product {
    pub name: String,
    pub id: String,
    pub url: String,

    #[serde(rename = "type")]
    pub product_type: String,

    pub classifications: Vec<Classification>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Accessibility {
    pub information : String,

    #[serde(rename = "ticketLimit")]
    pub ticket_limit: u8
}


#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Ticketing {
    #[serde(rename = "safeTix/enabled")]
    pub is_safe_tix: bool,

    #[serde(rename = "allInclusivePricing/enabled")]
    pub is_all_inclusive_pricing: bool,
}


#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Venue {
    #[serde(flatten)]
    pub info : Info,

    #[serde(rename = "test")]
    pub is_test: bool,
    pub url: String,
    pub aliases: Vec<String>,
    pub images: Vec<Image>,
    pub postal_code: String,
    pub timezone: String,
    #[serde(rename = "city/name")]
    pub city: String,

    #[serde(rename = "state")]
    pub state: State,

    #[serde(rename = "country")]
    pub country: Country,

    #[serde(rename = "address")]
    pub address: Address,

    pub location: Location,

    pub markets: Vec<Market>,

    #[serde(rename = "dmas")]
    pub dmas: Vec<Dma>,

    #[serde(rename = "boxOfficeInfo")]
    pub box_office_info: BoxOfficeInfo,

    #[serde(rename = "parkingDetail")]
    pub parking_detail: String,

    #[serde(rename = "accessibleSeatingDetail")]
    pub  accessible_seating_detail: String,

    #[serde(rename = "generalInfo")]
    pub general_info: GeneralInfo,

    #[serde(rename = "upcomingEvents")]
    pub upcoming_events: UpcomingEvents
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Country {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "countryCode")]
    pub code: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct State {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "stateCode")]
    pub code: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub line3: Option<String>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Location {
    pub longitude: String,
    pub latitude: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct BoxOfficeInfo {
    #[serde(rename = "phoneNumberDetail")]
    pub phone_number_detail: String,

    #[serde(rename = "openHoursDetail")]
    pub open_hours_detail: String,

    #[serde(rename = "acceptedPaymentDetail")]
    pub accepted_payment_detail: String,

    #[serde(rename = "willCallDetail")]
    pub  will_call_detail: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct GeneralInfo {
    #[serde(rename = "generalRule")]
    pub general_rule: String,

    #[serde(rename = "childRule")]
    pub child_rule: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct UpcomingEvents {
    #[serde(rename = "ticketmaster")]
    pub ticketmaster: u32,

    #[serde(rename = "_total")]
    pub total: u32,

    #[serde(rename = "_filtered")]
    pub filtered: u32,

    pub tmr : Option<u32>
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Attraction {
    #[serde(flatten)]
    pub info : Info,

    #[serde(rename = "type")]
    pub attraction_type: String,

    #[serde(rename = "test")]
    pub is_test: bool,

    pub url: String,

    #[serde(rename = "externalLinks")]
    pub external_links: ExternalLinks,

    #[serde(rename = "aliases")]
    pub aliases: Option<Vec<String>>,

    #[serde(rename = "images")]
    pub images: Vec<Image>,

    #[serde(rename = "classifications")]
    pub classifications: Vec<Classification>,

    #[serde(rename = "upcomingEvents")]
    pub upcoming_events : UpcomingEvents,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct ExternalLinks {
    #[serde(rename = "youtube")]
    pub youtube: Option<Vec<String>>,

    #[serde(rename = "twitter")]
    pub twitter: Option<Vec<String>>,

    #[serde(rename = "itunes")]
    pub itunes: Option<Vec<String>>,

    #[serde(rename = "lastfm")]
    pub lastfm: Option<Vec<String>>,

    #[serde(rename = "facebook")]
    pub facebook: Vec<String>,

    #[serde(rename = "wiki")]
    pub wiki: Option<Vec<String>>,

    #[serde(rename = "spotify")]
    pub spotify: Vec<String>,

    #[serde(rename = "musicbrainz")]
    pub musicbrainz: Option<Vec<String>>,

    #[serde(rename = "instagram")]
    pub instagram: Vec<String>,

    #[serde(rename = "homepage")]
    pub  homepage: Vec<String>,
}