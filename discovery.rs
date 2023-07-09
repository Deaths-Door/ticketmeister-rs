use std::collections::HashMap;

use api_request_utils_rs::{
    RequestInfo,
    RequestModifiers,
    RequestDefaults,
    RequestHandler,
    RequestError,
    reqwest::{
        Client,
        RequestBuilder,
    },
};

use crate::{
    Genre,
    SubGenre,
    Segment
};

add custom logic to handle _links , _embedeed correctly

pub struct Discovery<'a> {
    client : Client,
    api_key : &'a str,
    error_resolver : Box<&'a dyn Fn(../* TODO decide type */)>
}

impl RequestInfo for Discovery<'_> {
    const BASE_URL : &'static str = " https://app.ticketmaster.com/discovery/v2";
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

//todo add embeeded stuff if wanted
impl<'a> Discovery<'a> {
    pub fn new(api_key : &'a str,error_resolver : &'a dyn Fn(&RequestError<Value>)) -> Self {
        let client = Client::new();
        Self { client , api_key , error_resolver : Box::new(error_resolver)}
    }

    async fn details<T>(&self,endpoint : &str,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<T> {
        let endpoint = format!("{}/{}",endpoint,id);
        let parameters = HashMap::from([("locale",locale),("domain",domain)]);
        Self::request(self.default_get_requestor(&endpoint, parameters)).await
    }

    /// Get Genre Details
    ///
    /// Get details for a specific genre using its unique identifier.
    ///
    /// # Endpoint
    /// `/discovery/v2/classifications/genres/{id}`
    ///
    /// ## Parameters
    ///
    /// - `id` : ID of the genre.
    ///
    /// - `locale` : The locale in ISO code format. Multiple comma-separated values can be provided. When omitting the country part of the code (e.g., only 'en' or 'fr'), then the first matching locale is used. When using a '*' it matches all locales. '*' can only be used at the end (e.g., 'en-us,en,*').
    ///
    /// - `domain`: Filter entities based on domains they are available on.
    pub async fn genre_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<Genre<'a>> {
        self.details("classifications/genres",id,locale,domain).await
    }

    /// Get Sub-Genre Details
    ///
    /// Get details for a specific sub-genre using its unique identifier.
    ///
    /// # Endpoint
    /// `/discovery/v2/classifications/subgenres/{id}`
    ///
    /// ## Parameters
    ///
    /// - `id` : ID of the sub-genre.
    ///
    /// - `locale` : The locale in ISO code format. Multiple comma-separated values can be provided. When omitting the country part of the code (e.g., only 'en' or 'fr'), then the first matching locale is used. When using a '*' it matches all locales. '*' can only be used at the end (e.g., 'en-us,en,*').
    ///
    /// - `domain`: Filter entities based on domains they are available on.
    pub async fn sub_genre_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<SubGenre<'a>> {
        self.details("classifications/subgenres",id,locale,domain).await
    }

    /// Get Segment Details
    ///
    /// Get details for a specific segment using its unique identifier.
    ///
    /// # Endpoint
    /// `/discovery/v2/classifications/segment/{id}`
    ///
    /// ## Parameters
    ///
    /// - `id` : ID of the sub-genre.
    ///
    /// - `locale` : The locale in ISO code format. Multiple comma-separated values can be provided. When omitting the country part of the code (e.g., only 'en' or 'fr'), then the first matching locale is used. When using a '*' it matches all locales. '*' can only be used at the end (e.g., 'en-us,en,*').
    ///
    /// - `domain`: Filter entities based on domains they are available on.
    pub async fn segment_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<Segment<'a>> {
        self.details("classifications/segment",id,locale,domain).await
    }

    pub async fn event_details(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<Event<'a>> {
        self.details("events",id,locale,domain).await
    }

    /*pub async fn event_images(&self,id : &str,locale : Option<&str>,domain : &[&str]) -> Option<Vec<ImageInfo<'a>>> {
        let format = format!("events/{}",id);
        self.details(&format,"images",locale,domain)
    }*/
}