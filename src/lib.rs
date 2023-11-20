pub mod api;
pub mod data;
pub mod error;
pub mod templates;
pub mod validators;

use error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
