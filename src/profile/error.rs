use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Profile name can't contain dots")]
    InvalidName,
    #[error(
    "Can't use username/email combination: {username}/{email}\nAlready in use by profile: '{existing_profile}'"
    )]
    CombinationExists {
        username: String,
        email: String,
        existing_profile: String,
    },
    // TODO should they be transparent?
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] bincode::Error),
}
