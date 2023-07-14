use std::collections::HashMap;

use api_request_utils_rs::{
    ParameterHashMap,
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
        from_value,
        to_value
    },
    serde::de::DeserializeOwned
};

use crate::{
    Source,
    Page,
    Event,
    Image,
    Attraction,
    Classification,
    Genre,
    SubGenre,
    Segment,
    Venue,
    EventSearchQuery,
    AttractionSearchQuery,
    ClassificationSearchQuery,
    VenueSearchQuery
};

impl RequestInfo for Discovery<'_> {
    const BASE_URL : &'static str = "https://app.ticketmaster.com/discovery/v2";
    
    fn client(&self) -> &Client {
        &self.client
    }
}

impl RequestModifiers for Discovery<'_> {
    fn create_endpoint(endpoint: &str) -> String {
        format!("{}/{}.json",Self::BASE_URL,endpoint)
    }
}

impl RequestDefaults for Discovery<'_> { 
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

/// attraction search ,venue search , event search, classifications search
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

    fn insert_source<'l>(hashmap : &mut ParameterHashMap<'l>,source : Option<Source>) {
        hashmap.insert("source",to_value(source).unwrap());
    }

    async fn embedded_details<K,V>(result : Option<Value>,array_name : &str) -> Option<(K,Vec<V>)> where K: DeserializeOwned , V: DeserializeOwned {
        result.and_then(|value|{
            let key : K = from_value(value.clone()).unwrap();
            let vector : Vec<V> = from_value(value.get("_embedded").unwrap().get(array_name).unwrap().clone()).unwrap();
            Some((key,vector))
        })
    }

    pub async fn search_events(&self,query : &mut EventSearchQuery<'_>,source : Option<Source>) -> Option<(Page,Vec<Event>)> {
        Self::insert_source(&mut query.0,source);
        let value = self.get_request_handler::<Value,Value>("events",&query.0,self.error_handler).await;
        Self::embedded_details(value,"events").await
    }

    pub async fn search_attractions(&self,query : &mut AttractionSearchQuery<'_>,source : Option<Source>) -> Option<(Page,Vec<Attraction>)> {
        Self::insert_source(&mut query.0,source);
        let value = self.get_request_handler::<Value,Value>("attractions",&query.0,self.error_handler).await;
        Self::embedded_details(value,"attractions").await
    }

    pub async fn search_classifications(&self,query : &mut ClassificationSearchQuery<'_>,source : Option<Source>) -> Option<(Page,Vec<Classification>)> {
        Self::insert_source(&mut query.0,source);
        let value = self.get_request_handler::<Value,Value>("classifications",&query.0,self.error_handler).await;
        Self::embedded_details(value,"classifications").await
    }

    pub async fn search_venues(&self,query : &mut VenueSearchQuery<'_>,source : Option<Source>) -> Option<(Page,Vec<Venue>)> {
        Self::insert_source(&mut query.0,source);
        let value = self.get_request_handler::<Value,Value>("venues",&query.0,self.error_handler).await;
        Self::embedded_details(value,"venues").await
    }

    async fn details<T>(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<T> where T: DeserializeOwned {
        let joined_endpoint = format!("{}/{}",endpoint,id);
        let parameters = HashMap::from([("locale",Value::from(locale)),("domain",Value::from(domain)),("source",to_value(source).unwrap())]);
        self.get_request_handler(&joined_endpoint, &parameters, self.error_handler).await
    }

    pub async fn event_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<Event> {
        self.details("events",id,locale,domain,source).await
    }

    pub async fn event_images(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<Image> {
        let format = format!("events/{}",id);
        self.details(&format,"images",locale,domain,source).await
    }

    pub async fn attraction_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<Attraction> {
        self.details("attractions",id,locale,domain,source).await
    }

    pub async fn classification_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<Classification> {
        self.details("classifications",id,locale,domain,source).await
    }

    /// genre , related subgenres
    pub async fn genre_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<(Genre,Vec<SubGenre>)> {
        let result = self.details::<Value>("classifications/genres",id,locale,domain,source).await;
        Self::embedded_details(result,"subgenres").await
    }

    /// segment , related genres
    pub async fn segment_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<(Segment,Vec<Genre>)> {
        let result = self.details::<Value>("classifications/segments",id,locale,domain,source ).await;
        Self::embedded_details(result,"genres").await
    }

    pub async fn subgenre_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<SubGenre> {
        self.details("classifications/subgenres",id,locale,domain,source ).await
    }

    pub async fn venue_details(&self,id : &str,locale : Option<&str>,domain : Option<&[&str]>,source : Option<Source>) -> Option<Classification> {
        self.details("venues",id,locale,domain,source ).await
    }

}