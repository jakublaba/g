use thiserror::Error;

use crate::ssh::key::MIN_RSA_SIZE;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown ssh key type: {0}")]
    UnknownKeyType(String),
    #[error("{0}: Invalid RSA key length. Minimum is {MIN_RSA_SIZE} bits")]
    InvalidRsaLength(usize),
    #[error(transparent)]
    Crypto(#[from] ssh_key::Error),
}