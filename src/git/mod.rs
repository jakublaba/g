use std::fmt::Display;
use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;
use serde::de::IntoDeserializer;
use serde_json::value::Index;

use crate::git::error::Error;
use crate::ssh::key::{private_key_path, public_key_path};

pub mod error;

const URL_REGEX: &str = r"git@github\.com:.+\/(?<repo>.+)\.git";

pub type Result<T> = std::result::Result<T, Error>;

// TODO for now idk if the Repository return type is useful or not
pub fn clone(profile_name: &str, url: &str) -> Result<Repository> {
    let repo_name = parse_repo_name(url)?;
    repo_builder(profile_name)
        .clone(url, Path::new(&repo_name))
        .map_err(Error::Git)
}

fn repo_builder(profile_name: &str) -> RepoBuilder {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, usr, _types| Cred::ssh_key(
        usr.unwrap(),
        Some(Path::new(&public_key_path(profile_name))),
        Path::new(&private_key_path(profile_name)),
        None,
    ));
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    repo_builder
}

fn parse_repo_name(url: &str) -> Result<String> {
    let regex = Regex::new(URL_REGEX).unwrap();
    regex.captures(url)
        .ok_or(Error::Url(url.into()))?
        .name("repo")
        .ok_or(Error::Url(url.into()))
        .map(|m| m.as_str().into())
}
