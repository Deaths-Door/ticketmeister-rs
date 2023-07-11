use api_request_utils_rs::serde::{Serialize,Deserialize};

use crate::{
    TicketAvailability,
    ResaleAvailability
};

/// Struct representing the inventory status for an event.
///
/// This struct contains information about the event ID and the availability statuses
/// for both primary and resale tickets.
#[derive(Serialize, Deserialize)]
#[serde(crate = "api_request_utils_rs::serde")]
pub struct InventoryStatus {
    /// The event ID for which the inventory status is being reported.
    pub event_id: String,
    /// The availability status of tickets through primary channels.
    pub status: TicketAvailability,
    /// The availability status of tickets through resale channels.
    pub resale_status: ResaleAvailability,
}