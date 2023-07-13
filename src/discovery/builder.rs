use api_request_utils_rs::{
    ParameterHashMap,
    serde_json::{
        Value,
        to_value
    }
};

use crate::{
    Unit,
    Market,
    SortingOrder,
    ClassificationName,
    Dma,
    Decision,
};

pub struct SearchQuery<'a>(pub(super) &'a ParameterHashMap<'a>);

impl<'a> SearchQuery<'a> {
    pub fn new() -> Self {
        let hashmap = ParameterHashMap::new();
        Self(&hashmap)
    }

    /// Filter entities by its id
    pub fn with_id(mut self, id: &'a str) -> Self {
        self.0.insert("id", Value::from(id));
        self
    }

    /// Keyword to search on	
    pub fn with_keyword(mut self, keyword: &'a str) -> Self {
        self.0.insert("keyword", Value::from(keyword));
        self
    }

    /// Filter by attraction id
    pub fn with_attraction_id(mut self, attraction_id: &'a str) -> Self {
        self.0.insert("attractionId", Value::from(attraction_id));
        self
    }

    /// Filter by venue id
    pub fn with_venue_id(mut self, venue_id: &'a str) -> Self {
        self.0.insert("venueId", Value::from(venue_id));
        self
    }

    /// Filter by postal code / zipcode
    pub fn with_postal_code(mut self, postal_code: &'a str) -> Self {
        self.0.insert("postalCode", Value::from(postal_code));
        self
    }

    /// Radius of the area in which we want to search for events.
    pub fn with_radius(mut self, radius: u16) -> Self {
        self.0.insert("radius", Value::from(radius));
        self
    }

    /// Unit of the radius
    pub fn with_unit(mut self, unit: Unit) -> Self {
        self.0.insert("unit", to_value(unit).unwrap());
        self
    }

    /// The locale in ISO code format. Multiple comma-separated values can be provided. When omitting the country part of the code (e.g. only 'en' or 'fr') then the first matching locale is used. When using a '*' it matches all locales. '*' can only be used at the end (e.g. 'en-us,en,*')
    pub fn with_locale(mut self, locale: &'a str) -> Self {
        self.0.insert("locale", Value::from(locale));
        self
    }

    /// Filter by market id
    pub fn with_market(mut self, market: Market) -> Self {
        self.0.insert("marketId", to_value(market).unwrap());
        self
    }

    /// Filter with a start date after this date
    pub fn with_start_date_time(mut self, start_date_time: &'a str) -> Self {
        self.0.insert("startDateTime", Value::from(start_date_time));
        self
    }

    /// Filter with a start date before this date
    pub fn with_end_date_time(mut self, end_date_time: &'a str) -> Self {
        self.0.insert("endDateTime", Value::from(end_date_time));
        self
    }

      /// yes, to include with date to be announce (TBA)
      pub fn with_include_tba(mut self, include_tba: Decision) -> Self {
        self.0.insert("includeTBA", to_value(include_tba).unwrap());
        self
    }

    /// yes, to include with a date to be defined (TBD)
    pub fn with_include_tbd(mut self, include_tbd: Decision) -> Self {
        self.0.insert("includeTBD", to_value(include_tbd).unwrap());
        self
    }

    /// Page size of the response 
    pub fn with_size(mut self, size: u16) -> Self {
        self.0.insert("size", Value::from(size));
        self
    }  

    /// Page number
    pub fn with_page(mut self, page: u16) -> Self {
        self.0.insert("page", Value::from(page));
        self
    }

    pub fn with_sort(mut self, sort: SortingOrder) -> Self {
        self.0.insert("sort", to_value(sort).unwrap());
        self
    }

    pub fn with_onsale_start_date_time(mut self, onsale_start_date_time: &'a str) -> Self {
        self.0.insert("onsaleStartDateTime", Value::from(onsale_start_date_time));
        self
    }

    pub fn with_onsale_end_date_time(mut self, onsale_end_date_time: &'a str) -> Self {
        self.0.insert("onsaleEndDateTime", Value::from(onsale_end_date_time));
        self
    }

    pub fn with_city(mut self, city: &'a [&'a str]) -> Self {
        self.0.insert("city", Value::from(city));
        self
    }

    pub fn with_country_code(mut self, country_code: &'a str) -> Self {
        self.0.insert("countryCode", Value::from(country_code));
        self
    }

    pub fn with_state_code(mut self, state_code: &'a str) -> Self {
        self.0.insert("stateCode", Value::from(state_code));
        self
    }

    pub fn with_classification_name(mut self, classification_name: ClassificationName) -> Self {
        self.0.insert("classificationName", to_value(classification_name).unwrap());
        self
    }

    pub fn with_classification_id(mut self, classification_id: &'a[&'a str]) -> Self {
        self.0.insert("classificationId", Value::from(classification_id));
        self
    }

    pub fn with_local_start_date_time(mut self, local_start_date_time: &'a [&'a str]) -> Self {
        self.0.insert("localStartDateTime", Value::from(local_start_date_time));
        self
    }

    pub fn with_local_start_end_date_time(mut self, local_start_end_date_time: &'a [&'a str]) -> Self {
        self.0.insert("localStartEndDateTime", Value::from(local_start_end_date_time));
        self
    }

    pub fn with_start_end_date_time(mut self, start_end_date_time: &'a [&'a str]) -> Self {
        self.0.insert("startEndDateTime", Value::from(start_end_date_time));
        self
    }

    pub fn with_public_visibility_start_date_time(mut self, public_visibility_start_date_time: &'a str) -> Self {
        self.0.insert("publicVisibilityStartDateTime", Value::from(public_visibility_start_date_time));
        self
    }

    pub fn with_pre_sale_date_time(mut self, pre_sale_date_time: &'a [&'a str]) -> Self {
        self.0.insert("preSaleDateTime", Value::from(pre_sale_date_time));
        self
    }

    pub fn with_onsale_on_start_date(mut self, onsale_on_start_date: &'a str) -> Self {
        self.0.insert("onsaleOnStartDate", Value::from(onsale_on_start_date));
        self
    }

    pub fn with_dma(mut self, dma: Dma) -> Self {
        self.0.insert("dmaId", to_value(dma).unwrap());
        self
    }

    pub fn with_onsale_on_after_start_date(mut self, onsale_on_after_start_date: &'a str) -> Self {
        self.0.insert("onsaleOnAfterStartDate", Value::from(onsale_on_after_start_date));
        self
    }

    pub fn with_collection_id(mut self, collection_id: &'a [&'a str]) -> Self {
        self.0.insert("collectionId", Value::from(collection_id));
        self
    }

    pub fn with_segment_id(mut self, segment_id: &'a [&'a str]) -> Self {
        self.0.insert("segmentId", Value::from(segment_id));
        self
    }

    pub fn with_segment_name(mut self, segment_name: &'a [&'a str]) -> Self {
        self.0.insert("segmentName", Value::from(segment_name));
        self
    }

    pub fn with_include_family(mut self, include_family: Decision) -> Self {
        self.0.insert("includeFamily", to_value(include_family).unwrap());
        self
    }

    pub fn with_promoter_id(mut self, promoter_id: &'a str) -> Self {
        self.0.insert("promoterId", Value::from(promoter_id));
        self
    }

    pub fn with_genre_id(mut self, genre_id: &'a [&'a str]) -> Self {
        self.0.insert("genreId", Value::from(genre_id));
        self
    }

    pub fn with_sub_genre_id(mut self, sub_genre_id: &'a [&'a str]) -> Self {
        self.0.insert("subGenreId", Value::from(sub_genre_id));
        self
    }

    pub fn with_type_id(mut self, type_id: &'a [&'a str]) -> Self {
        self.0.insert("typeId", Value::from(type_id));
        self
    }

    pub fn with_sub_type_id(mut self, sub_type_id: &'a [&'a str]) -> Self {
        self.0.insert("subTypeId", Value::from(sub_type_id));
        self
    }

    pub fn with_geo_point(mut self, geo_point: &'a str) -> Self {
        self.0.insert("geoPoint", Value::from(geo_point));
        self
    }

    pub fn with_preferred_country_is_usa(mut self, result : bool) -> Self {
        let string = if result {
            "usa"
        }
        else {
            "ca"
        };

        self.0.insert("preferredCountry", Value::from(string));
        self
    }

    pub fn with_include_spellcheck(mut self, include_spellcheck: Decision) -> Self {
        self.0.insert("includeSpellcheck", to_value(include_spellcheck).unwrap());
        self
    }

    pub fn with_domain(mut self, domain: &'a [&'a str]) -> Self {
        self.0.insert("domain", Value::from(domain));
        self
    }
}