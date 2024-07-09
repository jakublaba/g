use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;

const GITHUB: &str = "git@github.com";
const URL_REGEX: Regex = Regex::new(r"(?<host>git@github\.com-?.*)(?<repo>:.+)").unwrap();

pub type Result<T> = std::result::Result<T, GitError>;

#[derive(Debug)]
pub struct GitError(pub String);

impl From<String> for GitError {
    fn from(msg: String) -> Self {
        Self { 0: msg }
    }
}

impl From<&str> for GitError {
    fn from(msg: &str) -> Self {
        Self { 0: String::from(msg) }
    }
}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GitError {}

// TODO implement adding keys to ssh-agent in crate::ssh
// TODO for now idk if the repository return type is useful or not
pub fn clone(profile_name: &str, url: &str) -> Result<Repository> {
    let substituted_url = substitute_url(profile_name, url);
    repo_builder(profile_name)
        .clone(&substituted_url, Path::new("."))
        .map_err(|_| GitError::from(format!("Error cloning repository: {url}")))
}

fn substitute_url(profile_name: &str, url: &str) -> String {
    let replacement = format!("{GITHUB}-{profile_name}:${{repo}}");
    String::from(URL_REGEX.replace(url, replacement))
}

fn repo_builder(profile_name: &str) -> RepoBuilder {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_| Cred::ssh_key_from_agent(profile_name));
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    repo_builder
}
