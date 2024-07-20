use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Git error: {0}")]
    Git(git2::Error),
    #[error("Bad url: {0}")]
    Url(String),
    #[error("Can't read dir: {0}")]
    Io(io::Error),
    #[error("Can't open repo: {0}")]
    Repo(git2::Error),
    #[error("Can't obtain config: {0}")]
    Config(git2::Error),
}
