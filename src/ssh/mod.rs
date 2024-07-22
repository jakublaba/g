use crate::ssh::error::Error;

pub mod key;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;
