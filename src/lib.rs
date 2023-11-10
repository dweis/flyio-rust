pub mod api;
pub mod data;
pub mod error;

use error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

