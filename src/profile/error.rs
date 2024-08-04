use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Profile name can't start with '.'")]
    InvalidName,
    #[error(
    "Can't use username/email combination: {username}/{email}\nAlready in use by profile: '{existing}'"
    )]
    CombinationExists {
        username: String,
        email: String,
        existing: String,
    },
    #[error("Profile with name '{0}' already exists")]
    ProfileExists(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] bincode::Error),
}
