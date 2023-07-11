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

    serde_json::Value
};


use crate::InventoryStatus;

/// The Inventory Status API provides event status for primary Ticketmaster inventory with inventory updates happening near real-time.
pub struct Inventory<'a> {
    client : Client,
    api_key : &'a str,
    error_handler : &'a ErrorHandler<()>
}

impl RequestInfo for Inventory<'_> {
    const BASE_URL : &'static str = " https://app.ticketmaster.com/inventory-status/v1";
}

impl RequestModifiers for Inventory<'_> {

}

impl RequestDefaults for Inventory<'_> { 
    fn client(&self) -> &Client {
        &self.client
    }
    
    fn default_parameters(&self,request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.query(&[("apikey", self.api_key)])
    }
}

impl RequestHandler for Inventory<'_> {}

impl<'a> Inventory<'a> {
    /// Creates a new `Inventory` instance.
    ///
    /// This method takes an API key and an error handler, and initializes
    /// a `Client` instance internally for making API requests.
    pub fn new(api_key: &'a str, error_handler: &'a ErrorHandler<()>) -> Self {
        let client = Client::new();
        Self::new_with_client(client, api_key, error_handler)
    }

    /// Creates a new `Inventory` instance with a specific `Client` instance.
    ///
    /// This method allows you to provide a pre-existing `Client` instance for making API requests,
    /// along with an API key and an error handler.
    pub fn new_with_client(client: Client, api_key: &'a str, error_handler: &'a ErrorHandler<()>) -> Self {
        Self {
            client,
            api_key,
            error_handler,
        }
    }

    /// Retrieves the inventory status for a list of event IDs.
    ///
    /// This method performs an API request to retrieve the inventory status for the specified event IDs.
    /// It takes a slice of event IDs and returns an optional `InventoryStatus` indicating the status of the inventory.
    pub async fn inventory_status(&self, event_ids: &[u32]) -> Option<InventoryStatus> {
        let parameters = std::collections::HashMap::from([("track_isrc", Value::from(event_ids))]);
        self.get_request_handler::<InventoryStatus, ()>("availability", parameters, self.error_handler)
            .await
    }
}