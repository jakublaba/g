use thiserror::Error;

use crate::ssh::key::MIN_RSA_SIZE;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Unknown ssh key type: {0}")]
    UnknownKeyType(String),
    #[error("Key pair already exists")]
    KeyPairExists,
    #[error("Invalid RSA key length ({0}). Minimum is {MIN_RSA_SIZE} bits")]
    InvalidRsaLength(usize),
    #[error(transparent)]
    LibSsh2(#[from] ssh_key::Error),
}