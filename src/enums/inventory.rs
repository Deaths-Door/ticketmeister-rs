use api_request_utils_rs::serde::{Deserialize,Serialize};

/// Enum representing the ticket availability status.
///
/// This enum is used to indicate the availability of tickets through primary channels.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum TicketAvailability {
    /// Tickets are available for purchase through primary channels.
    TicketsAvailable,
    /// Only a few tickets are left for purchase through primary channels.
    FewTicketsLeft,
    /// Tickets are not available for purchase through primary channels.
    TicketsNotAvailable,
    /// The availability status is unknown or the event ID is invalid.
    Unknown,
}

/// Enum representing the resale ticket availability status.
///
/// This enum is used to indicate the availability of tickets through resale channels.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(crate = "api_request_utils_rs::serde")]
pub enum ResaleAvailability {
    /// Tickets are available for purchase through resale channels.
    TicketsAvailable,
    /// Tickets are not available for purchase through resale channels.
    TicketsNotAvailable,
    /// The availability status is unknown or the event ID is invalid.
    Unknown,
}