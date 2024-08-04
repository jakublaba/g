use std::fmt::{Display, Formatter};

use crate::ssh::error::Error;
use crate::ssh::key::DEFAULT_RSA_SIZE;
use crate::ssh::Result;

#[derive(Debug, Clone)]
pub(crate) enum KeyType {
    Dsa,
    Rsa { size: Option<usize> },
    Ed25519,
}

pub(crate) trait RandomArtHeader {
    fn header(&self) -> String;
}

impl KeyType {
    pub fn parse(arg: &str) -> Result<Self> {
        match arg.to_lowercase().as_str() {
            "dsa" => Ok(Self::Dsa),
            s if s.starts_with("rsa") => {
                let size = (&s[3..]).parse::<usize>().ok();
                Ok(Self::Rsa { size })
            }
            "ed25519" => Ok(Self::Ed25519),
            s => Err(Error::UnknownKeyType(s.to_string()))
        }
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            KeyType::Dsa => "dsa",
            KeyType::Rsa { .. } => "rsa",
            KeyType::Ed25519 => "ed25519"
        })
    }
}

impl RandomArtHeader for KeyType {
    fn header(&self) -> String {
        match self {
            KeyType::Dsa => "DSA 1024".to_string(),
            KeyType::Rsa { size } => format!("RSA {}", size.unwrap_or(DEFAULT_RSA_SIZE)),
            KeyType::Ed25519 => "ED25519".to_string()
        }
    }
}
