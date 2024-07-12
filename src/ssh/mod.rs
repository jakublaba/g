use std::fmt::Display;

use crate::ssh::error::Error;

pub mod key;
pub mod client;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;
