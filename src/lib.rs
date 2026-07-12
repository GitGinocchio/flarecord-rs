pub (crate) mod crypto;
pub (crate) mod utils;
pub (crate) mod services;
pub (crate) mod dev;
pub (crate) mod api;

pub mod bot;
pub mod error;
pub mod models;
pub mod traits;

pub mod prelude;

#[cfg(feature = "macros")]
pub use flarecord_macros::command;

#[cfg(feature = "macros")]
use crate::models::command::CommandType;

#[cfg(feature = "macros")]
pub extern crate inventory;

#[cfg(feature = "macros")]
pub struct CommandRegistration {
    pub constructor: fn() -> CommandType
}
#[cfg(feature = "macros")]
inventory::collect!(CommandRegistration);

//pub extern crate twilight_model;
//pub extern crate twilight_util;
//pub extern crate twilight_validate;