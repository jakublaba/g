use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {path}")]
    Io {
        path: String,
        #[source]
        cause: io::Error,
    },
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
