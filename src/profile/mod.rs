use crate::profile::error::Error;

pub mod profile;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;
