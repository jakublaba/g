use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
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
    #[error("{0}, path: {1}")]
    Io(#[source] io::Error, PathBuf),
    #[error(transparent)]
    Serde(#[from] bincode::Error),
}
