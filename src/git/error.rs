use std::error::Error;
use std::fmt::{Display, Formatter};

// TODO resolve lifetimes skill issue
#[derive(Debug)]
pub struct GitError<'a>(String, Option<&'a (dyn Error + 'static)>);

impl<S: AsRef<str>, E: Error> From<(S, E)> for GitError {
    fn from(args: (S, E)) -> Self {
        Self {
            0: String::from(args.0.as_ref()),
            1: Some(args.1),
        }
    }
}

impl<S: AsRef<str>> From<S> for GitError {
    fn from(msg: S) -> Self {
        Self {
            0: String::from(msg.as_ref()),
            1: None,
        }
    }
}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.1)
    }
}
