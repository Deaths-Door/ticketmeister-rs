#[cfg(feature = "inventory")]
mod inventory_structs;

#[cfg(feature = "discovery")]
mod discovery_structs;

#[cfg(feature = "inventory")]
pub use self::inventory_structs::*;

#[cfg(feature = "discovery")]
pub use self::discovery_structs::*;