use api_request_utils_rs::serde::{Deserialize,Serialize};

use crate::{
    ImageRatio,
    Unit,
    StatusCode
};

/// Represents information about an item.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct ItemInfo<'a> {
    /// The ID of the classification's level.
    pub id: &'a str,
    /// The Name of the classification's level.
    pub name: &'a str,
    /// Locale in which the content is returned.
    pub locale: &'a str,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Represents a sub-genre.
pub struct SubGenre<'a>(#[serde(borrow)] ItemInfo<'a>);

/// Represents a genre.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Genre<'a> {
    /// Information about the genre.
    #[serde(flatten)]
    #[serde(borrow)]
    pub info: ItemInfo<'a>,
    /// List of Tertiary Genre linked to the Secondary Genre
    #[serde(rename = "subGenres")]
    #[serde(borrow)]
    pub sub_genres: Vec<SubGenre<'a>>,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Represents a segment.
pub struct Segment<'a>(#[serde(borrow)] ItemInfo<'a>);

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Image Information
pub struct ImageInfo<'a> {
    /// Public URL of the image
    pub url: &'a str,
    /// Aspect ratio of the image
    pub ratio: ImageRatio,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
    /// true if the image is not the event's image but a fallbak image
    pub fallback: bool,
    /// Attribution of the image
    pub attribution: Option<&'a str>,
}


#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
pub struct Event<'a> {
    #[serde(flatten)]
    /// Represents a distance measurement.
    pub distance : Distance,

    /// Represents a location with longitude and latitude coordinates.
    #[serde(flatten)]
    pub location : Location,

    /// Represents information about this item.
    #[serde(flatten)]
    pub item_info : ItemInfo<'a>,
    
    /// URL of a web site detail page of the entity
    pub url : &'a str,
    
    /// Images of the entity
    pub images : Vec<ImageInfo<'a>>,

    /// Event's dates information
    pub dates : Dates<'a>,

    /// Event's sales dates information
    pub sales : Sales<'a>,

    /// Any information related to the event
    pub info : &'a str,

    /// Any notes related to the event
    #[serde(rename = "pleaseNote")]
    pub please_note : &'a str,

    /// Price ranges of this event
    #[serde(rename = "priceRanges")]
    pub price_ranges : Vec<Price<'a>>,

    pub classifications: Vec<Classification<'a>>,

    pub promoter: Promoter,
    pub promoters: Vec<Promoter2>,
    pub products: Vec<Product>,
    pub seatmap: Seatmap,
    pub accessibility: Accessibility,
    pub ticket_limit: TicketLimit,
    pub age_restrictions: AgeRestrictions,
    pub ticketing: Ticketing,

}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Represents a distance measurement.
pub struct Distance {
    /// The distance value.
    pub distance: f64,
    /// The unit of measurement for the distance.
    pub unit: Unit,
}


#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Represents a location with longitude and latitude coordinates.
pub struct Location {
    /// The longitude coordinate.
    pub longitude: f64,
    /// The latitude coordinate.
    pub latitude: f64,
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
/// Represents the dates information of an event.
pub struct Dates<'a> {
    /// The start dates of the event.
    pub start: StartTime<'a>,
    /// The end dates of the event.
    pub end: EndTime<'a>,
    /// The access dates of the event.
    pub access: Access<'a>,
    /// The status of the event.
    #[serde(rename = "status")]
    #[serde(flatten)]
    pub status_code: StatusCode,
    /// The timezone of the event.
    pub timezone: &'a str,
    /// Flag indicating if the event spans multiple days.
    #[serde(rename = "spanMultipleDays")]
    pub span_multiple_days: bool,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")] // must be below the derive attribute
/// Represents date and time information.
pub struct DateTime<'a> {
    /// The local date of the event.
    #[serde(rename = "localDate")]
    pub local_date: &'a str,
    /// The local time of the event.
    #[serde(rename = "localTime")]
    pub local_time: &'a str,
    /// Flag indicating if the event start time has no specific time.
    #[serde(rename = "noSpecificTime")]
    pub has_specific_start_time: bool,
}

/// Represents the start time of an event.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct StartTime<'a> {
    /// The date and time of the event start.
    #[serde(flatten)]
    pub date_time: DateTime<'a>,
    /// The event start datetime.
    #[serde(rename = "dateTime")]
    pub start_time: &'a str,
    /// Boolean flag indicating if the start date is TBD.
    #[serde(rename = "dateTBD")]
    pub date_tbd: bool,
    /// Boolean flag indicating if the start date is TBA.
    #[serde(rename = "dateTBA")]
    pub date_tba: bool,
    /// Boolean flag indicating if the start time is TBA.
    #[serde(rename = "timeTBA")]
    pub time_tba: bool,
}

/// Represents the end time of an event.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct EndTime<'a> {
    /// The date and time of the event end.
    #[serde(flatten)]
    pub date_time: DateTime<'a>,
    /// The event end datetime.
    #[serde(rename = "dateTime")]
    pub end_time: &'a str,
    /// Boolean flag indicating if the end date is approximated.
    pub approximate: bool,
}

/// Represents the access dates of an event.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Access<'a> {
    /// The start datetime of event access.
    #[serde(rename = "startDateTime")]
    pub start_date_time: &'a str,
    /// Boolean flag indicating if the access start date is approximated.
    #[serde(rename = "startApproximate")]
    pub is_start_time_approximate: bool,
    /// The end datetime of event access.
    #[serde(rename = "endDateTime")]
    pub end_date_time: &'a str,
    /// Boolean flag indicating if the access end date is approximated.
    #[serde(rename = "endApproximate")]
    pub is_end_time_approximate: bool,
}

/// Represents a price range.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct Price<'a> {
    /// The currency of the price range.
    pub currency: &'a str,
    /// The minimum value of the price range.
    pub min: f64,
    /// The maximum value of the price range.
    pub max: f64,
}