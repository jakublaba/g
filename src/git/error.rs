use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("Bad url: {0}")]
    Url(String),
    #[error("Can't read dir: {0}")]
    Io(#[from] io::Error),
    #[error("Can't open repo: {0}")]
    Repo(#[from] git2::Error),
    #[error("Can't obtain config: {0}")]
    Config(#[from] git2::Error),
}
