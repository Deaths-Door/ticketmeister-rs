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
    info : Info,

    #[serde(rename ="test")]
    is_test: bool,

    url: String,

    images: Vec<Image>,

    sales: Sales,

    dates: Dates,

    classifications: Vec<Classification>,

    promoter: Promoter,

    promoters: Vec<Promoter>,

    #[serde(rename = "pleaseNote")]
    please_note: String,

    #[serde(rename = "priceRanges")]
    price_ranges: Vec<PriceRange>,

    products: Vec<Product>,

    #[serde(rename = "seatmap/static_url")]
    seatmap: String,

    accessibility: Accessibility,

    #[serde(rename = "ticketLimit/info")]
    ticket_limit_infomation: String,

    #[serde(rename = "ageRestrictions/legalAgeEnforced")]
    is_legal_age_enforced: bool,

    ticketing: Ticketing,

    #[serde(rename = "_embedded/venues")]
    venues: Vec<Venue>,

    #[serde(rename = "_embedded/attractions")]
    attractions: Vec<Attraction>,
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
    start_date_time: String,

    #[serde(rename = "endDateTime")]
    end_date_time: String,

    name: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Public {
    #[serde(rename = "startDateTime")]
    start_date_time: String,

    #[serde(rename = "startTBD")]
    start_tbd: bool,

    #[serde(rename = "startTBA")]
    start_tba: bool,

    #[serde(rename = "endDateTime")]
    end_date_time: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Dates {
    start: Start,

    timezone: String,

    #[serde(rename = "status/code")]
    status: EventStatus,

    #[serde(rename = "spanMultipleDays")]
    span_multiple_days: bool,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Start {
    #[serde(rename = "localDate")]
    local_date: String,

    #[serde(rename = "localTime")]
    local_time: String,

    #[serde(rename = "dateTime")]
    date_time: String,

    #[serde(rename = "dateTBD")]
    date_tbd: bool,

    #[serde(rename = "dateTBA")]
    date_tba: bool,

    #[serde(rename = "timeTBA")]
    time_tba: bool,

    #[serde(rename = "noSpecificTime")]
    no_specific_time: bool,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Classification {
    #[serde(rename = "primary")]
    is_primary: bool,

    segment: Genre,

    genre: Genre,

    #[serde(rename = "subGenre")]
    sub_genre: Genre,

    r#type : Type,

    #[serde(rename = "subType")]
    sub_type: Genre,

    #[serde(rename = "family")]
    is_family: bool,
}


/// Represents a sub-genre.
#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct SubGenre(Info);

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
pub struct Segment(Info);

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
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "description")]
    description: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct PriceRange {
    currency: String,
    min: f64,
    max: f64,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Product {
    name: String,
    id: String,
    url: String,

    #[serde(rename = "type")]
    product_type: String,

    classifications: Vec<Classification>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Accessibility {
    information : String,

    #[serde(rename = "ticketLimit")]
    ticket_limit: u8
}


#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Ticketing {
    #[serde(rename = "safeTix/enabled")]
    is_safe_tix: bool,

    #[serde(rename = "allInclusivePricing/enabled")]
    is_all_inclusive_pricing: bool,
}


#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Venue {
    #[serde(flatten)]
    info : Info,

    #[serde(rename = "test")]
    test: bool,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "aliases")]
    aliases: Vec<String>,

    #[serde(rename = "images")]
    images: Vec<Image>,

    #[serde(rename = "postalCode")]
    postal_code: String,

    #[serde(rename = "timezone")]
    timezone: String,

    #[serde(rename = "city/name")]
    city: String,

    #[serde(rename = "state")]
    state: State,

    #[serde(rename = "country")]
    country: Country,

    #[serde(rename = "address")]
    address: Address,

    #[serde(rename = "location")]
    location: Location,

    #[serde(rename = "markets")]
    markets: Vec<Market>,

    #[serde(rename = "dmas")]
    dmas: Vec<Dma>,

    #[serde(rename = "boxOfficeInfo")]
    box_office_info: BoxOfficeInfo,

    #[serde(rename = "parkingDetail")]
    parking_detail: String,

    #[serde(rename = "accessibleSeatingDetail")]
    accessible_seating_detail: String,

    #[serde(rename = "generalInfo")]
    general_info: GeneralInfo,

    #[serde(rename = "upcomingEvents")]
    upcoming_events: UpcomingEvents
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Country {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "countryCode")]
    code: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct State {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "stateCode")]
    code: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Address {
    line1: String,
    line2: Option<String>,
    line3: Option<String>,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Location {
    longitude: String,
    latitude: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct BoxOfficeInfo {
    #[serde(rename = "phoneNumberDetail")]
    phone_number_detail: String,

    #[serde(rename = "openHoursDetail")]
    open_hours_detail: String,

    #[serde(rename = "acceptedPaymentDetail")]
    accepted_payment_detail: String,

    #[serde(rename = "willCallDetail")]
    will_call_detail: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct GeneralInfo {
    #[serde(rename = "generalRule")]
    general_rule: String,

    #[serde(rename = "childRule")]
    child_rule: String,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct UpcomingEvents {
    #[serde(rename = "ticketmaster")]
    ticketmaster: u32,

    #[serde(rename = "_total")]
    total: u32,

    #[serde(rename = "_filtered")]
    filtered: u32,

    tmr : Option<u32>
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Attraction {
    #[serde(flatten)]
    info : Info,

    #[serde(rename = "type")]
    attraction_type: String,

    #[serde(rename = "test")]
    is_test: bool,

    url: String,

    #[serde(rename = "externalLinks")]
    external_links: ExternalLinks,

    #[serde(rename = "aliases")]
    aliases: Option<Vec<String>>,

    #[serde(rename = "images")]
    images: Vec<Image>,

    #[serde(rename = "classifications")]
    classifications: Vec<Classification>,

    #[serde(rename = "upcomingEvents")]
    upcoming_events : UpcomingEvents,
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct ExternalLinks {
    #[serde(rename = "youtube")]
    youtube: Option<Vec<String>>,

    #[serde(rename = "twitter")]
    twitter: Option<Vec<String>>,

    #[serde(rename = "itunes")]
    itunes: Option<Vec<String>>,

    #[serde(rename = "lastfm")]
    lastfm: Option<Vec<String>>,

    #[serde(rename = "facebook")]
    facebook: Vec<String>,

    #[serde(rename = "wiki")]
    wiki: Option<Vec<String>>,

    #[serde(rename = "spotify")]
    spotify: Vec<String>,

    #[serde(rename = "musicbrainz")]
    musicbrainz: Option<Vec<String>>,

    #[serde(rename = "instagram")]
    instagram: Vec<String>,

    #[serde(rename = "homepage")]
    homepage: Vec<String>,
}