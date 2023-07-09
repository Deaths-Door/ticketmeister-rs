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
//TODO add discovery feature for this instead

pub struct Discovery<'a> {
    client : Client,
    api_key : &'a str,
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