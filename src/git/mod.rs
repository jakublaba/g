use std::fmt::Display;
use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;

use crate::git::error::Error;

pub mod error;

const GITHUB: &str = "git@github.com";
const URL_REGEX: &str = r"git@github\.com-?.*:.+\/(?<repo>.+)\.git";

pub type Result<T> = std::result::Result<T, Error>;

// TODO for now idk if the Repository return type is useful or not
// TODO resolve cloning skill issue
pub fn clone(profile_name: &str, url: &str) -> Result<Repository> {
    let (substituted_url, repo) = parse_url(profile_name, url)?;
    repo_builder(profile_name)
        .clone(&substituted_url, Path::new(&repo))
        .map_err(Error::Clone)
}

fn parse_url(profile_name: &str, url: &str) -> Result<(String, String)> {
    let replacement = format!("{GITHUB}-{profile_name}:${{repo}}");
    // this only fails if you provide shitty regex, it should never happen and user shouldn't know
    let regex = Regex::new(URL_REGEX).unwrap();
    let profile_url = regex.replace(url, replacement);
    let repo = regex.captures(url)
        .ok_or(Error::InvalidUrl(String::from(url)))?
        .name("repo")
        .ok_or(Error::CantExtractRepo(String::from(url)))?
        .as_str();

    Ok((String::from(profile_url), String::from(repo)))
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
