use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;

const GITHUB: &str = "git@github.com";
const URL_REGEX: &str = r"git@github\.com-?.*:.+\/(?<repo>.+)\.git";

pub type Result<T> = std::result::Result<T, GitError>;

#[derive(Debug)]
pub struct GitError(String);

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

// TODO for now idk if the repository return type is useful or not
// TODO resolve cloning skill issue
pub fn clone(profile_name: &str, url: &str) -> Result<Repository> {
    let (substituted_url, repo) = parse_url(profile_name, url);
    repo_builder(profile_name)
        .clone(&substituted_url, Path::new(&repo))
        .map_err(|_| GitError::from(format!("Error cloning repository: {url}")))
}

fn parse_url(profile_name: &str, url: &str) -> (String, String) {
    let replacement = format!("{GITHUB}-{profile_name}:${{repo}}");
    let regex = Regex::new(URL_REGEX).unwrap();
    let profile_url = regex.replace(url, replacement);
    // todo figure out what to do with those ugly unwraps
    let repo = regex.captures(url).unwrap().name("repo").unwrap().as_str();

    (String::from(profile_url), String::from(repo))
}

fn repo_builder(profile_name: &str) -> RepoBuilder {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _usr, _types| Cred::ssh_key_from_agent(profile_name));
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    repo_builder
}
