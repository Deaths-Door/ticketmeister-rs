use api_request_utils_rs::serde::{Deserialize,Serialize};

#[cfg(feature = "inventory")]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(crate = "api_request_utils_rs::serde")]
/// Enum representing the ticket availability status.
///
/// This enum is used to indicate the availability of tickets through primary channels.
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

#[cfg(feature = "inventory")]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(crate = "api_request_utils_rs::serde")]
/// Enum representing the resale ticket availability status.
///
/// This enum is used to indicate the availability of tickets through resale channels.
pub enum ResaleAvailability {
    /// Tickets are available for purchase through resale channels.
    TicketsAvailable,
    /// Tickets are not available for purchase through resale channels.
    TicketsNotAvailable,
    /// The availability status is unknown or the event ID is invalid.
    Unknown,
}