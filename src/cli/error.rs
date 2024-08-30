use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("No profile set")]
    NoProfileSet,
    #[error("{err}\nTip: {tip}")]
    WithTip { err: Box<dyn std::error::Error>, tip: &'static str },
    #[error(transparent)]
    Git(#[from] crate::git::error::Error),
    #[error(transparent)]
    Profile(#[from] crate::profile::error::Error),
    #[error(transparent)]
    Ssh(#[from] crate::ssh::error::Error),
}