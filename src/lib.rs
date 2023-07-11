#![doc = include_str!("../README.md")]

#![forbid(
    missing_docs,

    unsafe_code,
    
    unused_imports,
    unused_variables,
    unused_mut,
    unused_results,
    unused_allocation,
    unused_must_use,

    unreachable_patterns,

    trivial_casts,

    unsafe_op_in_unsafe_fn,
    
    overflowing_literals,
)]

mod structs;
mod enums;

#[cfg(feature = "inventory")]
mod inventory;

#[cfg(feature = "discovery")]
mod discovery;

#[cfg(feature="discovery")]
mod builder;

pub use self::structs::*;
pub use self::enums::*;

#[cfg(feature = "inventory")]
pub use self::inventory::*;

#[cfg(feature = "discovery")]
pub use self::discovery::*;

#[cfg(feature="discovery")]
pub use self::builder::*;