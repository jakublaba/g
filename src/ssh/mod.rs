use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod config;
pub mod key;
pub mod client;

pub type Result<T> = std::result::Result<T, SshError>;

#[derive(Debug)]
pub struct SshError(pub String);

impl From<String> for SshError {
    fn from(msg: String) -> Self {
        Self { 0: msg }
    }
}

impl From<&str> for SshError {
    fn from(msg: &str) -> Self {
        Self { 0: String::from(msg) }
    }
}

impl Display for SshError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SshError {}
