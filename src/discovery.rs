use std::collections::HashMap;

use api_request_utils_rs::{
    RequestInfo,
    RequestModifiers,
    RequestDefaults,
    RequestHandler,

    ErrorHandler,
    
    reqwest::{
        Client,
        RequestBuilder,
    },

    serde_json::{
        Value,
        from_value
    }
};

use crate::{
  //  Image,
    Genre,
    SubGenre,
    Segment,
    Page,
    
    SearchQuery
};

impl RequestInfo for Discovery<'_> {
    const BASE_URL : &'static str = "https://app.ticketmaster.com/discovery/v2";
}

impl RequestModifiers for Discovery<'_> {
    fn create_endpoint(endpoint: &str) -> String {
        format!("{}/{}.json",Self::BASE_URL,endpoint)
    }
}

impl RequestDefaults for Discovery<'_> { 
    fn client(&self) -> &Client {
        &self.client
    }
    
    fn default_parameters(&self,request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("apikey", self.api_key)])
    }
}

impl RequestHandler for Discovery<'_> {}

/// The Ticketmaster Discovery API allows you to search for events, attractions, or venues.
pub struct Discovery<'a> {
    client : Client,
    api_key : &'a str,
    error_handler : &'a ErrorHandler<Value>
}

/// TODO add  The API provides access to content sourced from various platform, including Ticketmaster, Universe, FrontGate Tickets and Ticketmaster Resale (TMR). By default, the API returns events from all sources. To specify a specifc source(s), use the &source= parameter. Multiple, comma separated values are OK.
impl<'a> Discovery<'a> {
    /// Creates a new `Discovery` instance.
    ///
    /// This method takes an API key and an error handler, and initializes
    /// a `Client` instance internally for making API requests.
    pub fn new(api_key: &'a str, error_handler: &'a ErrorHandler<Value>) -> Self {
        let client = Client::new();
        Self::new_with_client(client, api_key, error_handler)
    }

    /// Creates a new `Discovery` instance with a specific `Client` instance.
    ///
    /// This method allows you to provide a pre-existing `Client` instance for making API requests,
    /// along with an API key and an error handler.
    pub fn new_with_client(client: Client, api_key: &'a str, error_handler: &'a ErrorHandler<Value>) -> Self {
        Self {
            client,
            api_key,
            error_handler,
        }
    }

    fn embedded(value : Value) -> Value {
        value.get("_embedded").unwrap()
    }
    
    async fn details<T>(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<T> where T: api_request_utils_rs::serde::de::DeserializeOwned {
        let joined_endpoint = format!("{}/{}",endpoint,id);
        let parameters = HashMap::from([("locale",Value::from(locale)),("domain",Value::from(domain))]);
        self.get_request_handler(&joined_endpoint, parameters, self.error_handler).await
    }

    pub async fn event_search(&self,query : SearchQuery<'_>) -> (Page,Vec<Event>) {
        let value = self.get_request_handler::<Value,Value>("events",query.hashmap());
        let page = from_value::<Page>(value.get("page").unwrap()).unwrap();
        let events = from_value::<Vec<Event>>(value.embedded().get("events").unwrap()).unwrap();
        (events,page)
    }

    pub async fn genre_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<Genre> {
        self.details("classifications/genres",id,locale,domain).await
    }

    pub async fn sub_genre_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<SubGenre> {
        self.details("classifications/subgenres",id,locale,domain).await
    }

    pub async fn segment_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<Segment> {
        self.details("classifications/segment",id,locale,domain).await
    }
}