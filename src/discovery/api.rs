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
    },
    serde::de::DeserializeOwned
};

use crate::{
    Source,
    SearchQuery,
    Page,
    Event,
    Image,
    Attraction,
    Classification,
    Genre,
    SubGenre,
    Segment
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

    async fn embedded_details<K,V>(result : Option<Value>,array_name : &str) -> Option<(K,Vec<V>)> where K: DeserializeOwned , V: DeserializeOwned {
        result.and_then(|value|{
            let key : K = from_value(value.clone()).unwrap();
            let vector : Vec<V> = from_value(value.get("_embedded").unwrap().get(array_name).unwrap().clone()).unwrap();
            Some((key,vector))
        })
    }


    pub async fn search_events(&self,query : SearchQuery<'_>,source : Source) -> Option<(Page,Vec<Event>)> {
        let value = self.get_request_handler::<Value,Value>("events",query.0,self.error_handler).await;
        Self::embedded_details(value,"events").await
    }

    async fn details<T>(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<T> where T: DeserializeOwned {
        let joined_endpoint = format!("{}/{}",endpoint,id);
        let parameters = HashMap::from([("locale",Value::from(locale)),("domain",Value::from(domain))]);
        self.get_request_handler(&joined_endpoint, parameters, self.error_handler).await
    }

    pub async fn event_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<Event> {
        self.details("events",id,locale,domain).await
    }

    pub async fn event_images(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<Image> {
        let format = format!("events/{}",id);
        self.details(&format,"images",locale,domain).await
    }

    pub async fn attraction_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<Attraction> {
        self.details("attractions",id,locale,domain).await
    }

    pub async fn classification_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<Classification> {
        self.details("classifications",id,locale,domain).await
    }

    /// genre , related subgenres
    pub async fn genre_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<(Genre,Vec<SubGenre>)> {
        let result = self.details::<Value>("classifications/genres",id,locale,domain).await;
        Self::embedded_details(result,"subgenres").await
    }

    /// segment , related genres
    pub async fn segment_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<(Segment,Vec<Genre>)> {
        let result = self.details::<Value>("classifications/segments",id,locale,domain).await;
        Self::embedded_details(result,"genres").await
    }

    pub async fn subgenre_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<SubGenre> {
        self.details("classifications/subgenres",id,locale,domain).await
    }

    pub async fn venue_details(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : Option<&[&str]>) -> Option<Classification> {
        self.details("venues",id,locale,domain).await
    }

}