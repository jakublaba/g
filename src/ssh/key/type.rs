use std::fmt::{Display, Formatter};

use crate::ssh::error::Error;
use crate::ssh::key::DEFAULT_RSA_SIZE;
use crate::ssh::Result;

/// Types of ssh keys that g supports
#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Dsa,
    Rsa { size: Option<usize> },
    Ed25519,
}

/// Provides a utility method to quickly obtain randomart header for each [`KeyType`]
pub trait RandomArtHeader {
    fn random_art_header(&self) -> String;
}

impl KeyType {
    pub fn parse(arg: &str) -> Result<Self> {
        match arg.to_lowercase().as_str() {
            "dsa" => Ok(Self::Dsa),
            s if s.starts_with("rsa") => {
                let size = s[3..].parse::<usize>().ok();
                Ok(Self::Rsa { size })
            }
            "ed25519" => Ok(Self::Ed25519),
            s => Err(Error::UnknownKeyType(s.to_string())),
        }
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyType::Dsa => "dsa",
                KeyType::Rsa { .. } => "rsa",
                KeyType::Ed25519 => "ed25519",
            }
        )
    }
}

impl RandomArtHeader for KeyType {
    fn random_art_header(&self) -> String {
        match self {
            KeyType::Dsa => "DSA 1024".to_string(),
            KeyType::Rsa { size } => format!("RSA {}", size.unwrap_or(DEFAULT_RSA_SIZE)),
            KeyType::Ed25519 => "ED25519".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use spectral::assert_that;

    mod parse {
        use super::*;

        #[rstest]
        #[case::dsa("dsa", KeyType::Dsa)]
        #[case::rsa_default("rsa", KeyType::Rsa {size: None})]
        #[case::rsa_valid_size("rsa2048", KeyType::Rsa {size: Some(2048)})]
        #[case::ed25519("ed25519", KeyType::Ed25519)]
        fn ok(#[case] key_type_str: &str, #[case] expected_key_type: KeyType) {
            assert_that!(KeyType::parse(key_type_str).unwrap()).is_equal_to(expected_key_type);
        }

        #[rstest]
        #[case::unknown_key_type("ecdsa")]
        fn err(#[case] key_type_str: &str) {
            let err_msg = format!("{}", KeyType::parse(key_type_str).unwrap_err());
            let expected_err_msg = format!("{}", Error::UnknownKeyType(key_type_str.to_string()));

            assert_that!(err_msg).is_equal_to(expected_err_msg);
        }
    }

    #[rstest]
    #[case::dsa(KeyType::Dsa, "dsa")]
    #[case::rsa_default_size(KeyType::Rsa{ size: None }, "rsa")]
    #[case::rsa_custom_size(KeyType::Rsa{ size: Some(2048) }, "rsa")]
    #[case::ed25519(KeyType::Ed25519, "ed25519")]
    fn display(#[case] key_type: KeyType, #[case] expected_display: &str) {
        let display = format!("{}", key_type);

        assert_that!(display).is_equal_to(expected_display.to_string());
    }

    #[rstest]
    #[case::dsa(KeyType::Dsa, "DSA 1024")]
    #[case::rsa_default_size(KeyType::Rsa{ size: None }, "RSA 3072")]
    #[case::rsa_custom_size(KeyType::Rsa {size: Some(2048) }, "RSA 2048")]
    #[case::ed25519(KeyType::Ed25519, "ED25519")]
    fn random_art_header(#[case] key_type: KeyType, #[case] expected_header: &str) {
        assert_that!(key_type.random_art_header()).is_equal_to(expected_header.to_string());
    }
}
