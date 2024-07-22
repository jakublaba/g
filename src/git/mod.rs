use std::env;
use std::path::Path;

use git2::{Config, Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;

use crate::git::error::Error;
use crate::home;
use crate::profile::profile::Profile;
use crate::ssh::key::{private_key_path, public_key_path};

pub mod error;

const URL_REGEX: &str = r"git@github\.com:.+\/(?<repo>.+)\.git";

pub type Result<T> = std::result::Result<T, Error>;

pub fn configure_user(profile: &Profile, global: bool) -> Result<()> {
    let is_inside_repo = is_inside_repo();
    if !is_inside_repo && !global {
        println!("No git repository detected, setting profile in global config");
    };
    if global || !is_inside_repo { set_config(profile, true)? };
    if is_inside_repo { set_config(profile, false)? };

    Ok(())
}

fn set_config(profile: &Profile, global: bool) -> Result<()> {
    let mut config = if global { open_global_config() } else { open_local_config() }?;
    config.set_str("user.name", &profile.user_name)
        .map_err(Error::Config)?;
    config.set_str("user.email", &profile.user_email)
        .map_err(Error::Config)?;
    config.set_str("core.sshCommand", &ssh_command(&profile.name))
        .map_err(Error::Config)?;

    Ok(())
}

fn is_inside_repo() -> bool {
    let current_dir = env::current_dir().unwrap();
    let path_str = format!("{}/.git", current_dir.to_str().unwrap());
    let path = Path::new(&path_str);

    path.exists() && path.is_dir()
}

fn open_global_config() -> Result<Config> {
    Config::open_default()
        .map_err(Error::Config)
}

fn open_local_config() -> Result<Config> {
    let current_dir = env::current_dir()
        .map_err(Error::Io)?;
    Repository::open(current_dir)
        .map_err(Error::Repo)?
        .config()
        .map_err(Error::Config)
}

// TODO Repository return type can be used to configure pre-push hook for auth right away
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

fn ssh_command(profile_name: &str) -> String {
    let home = home();
    format!("ssh -i {home}/.ssh/id_{profile_name} -F /dev/null")
}
