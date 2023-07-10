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

pub struct Inventory<'a> {
    client : Client,
    api_key : &'a str,
}

impl RequestInfo for Discovery<'_> {
    const BASE_URL : &'static str = " https://app.ticketmaster.com/inventory-status/v1/availability?";
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

impl<'a> Inventory<'a> {
    pub fn new(api_key : &'a str) -> Self {
        let client = Client::new();
        Self { client , api_key }
    }

    pub async fn inventory_status(event_ids : &[u32])
}