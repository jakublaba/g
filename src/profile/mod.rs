use std::fmt::Display;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use const_format::formatcp;

use crate::HOME;
use crate::profile::profile::{Profile, profile_path};
use crate::ssh;

pub mod profile;
pub mod cache;
pub mod error;

pub(crate) const PROFILES_DIR: &str = formatcp!("{HOME}/.config/g-profiles");

type Result<T> = std::result::Result<T, error::Error>;

/// Loads list of profile names from [`PROFILES_DIR`].
pub fn list() -> Result<Vec<String>> {
    let paths = fs::read_dir(PROFILES_DIR)?;
    let names = paths
        .map(|dir_entry| dir_entry.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.components()
            .last().unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string()
        )
        .filter(|name| !name.starts_with('.'))
        .collect();

    Ok(names)
}

/// Removes profile with chosen `name` from [`PROFILES_DIR`]
pub fn remove(name: &str) -> Result<()> {
    rm_file(profile_path(name));
    rm_file(ssh::key::path_private(name));
    rm_file(ssh::key::path_public(name));

    cache::remove(name)
}

/// Changes `username` and/or `email` for profile with specified `name`
pub fn edit(name: &str, username: Option<String>, email: Option<String>) -> Result<()> {
    let mut profile = Profile::load(name)?;
    let mut width = 0;
    if let Some(usr_name) = username {
        width = width.max(usr_name.len());
        profile.username = usr_name.to_string();
    };
    if let Some(usr_email) = email {
        width = width.max(usr_email.len());
        profile.email = usr_email.to_string();
    };

    profile.save(true)
}

fn rm_file<P: AsRef<Path> + Display>(path: P) {
    match fs::remove_file(&path) {
        Ok(_) => println!("{path} removed"),
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => println!("{path} doesn't exist, skipping"),
                ErrorKind::PermissionDenied => println!("{path} can't delete due to permissions, skipping"),
                _ => ()
            }
        }
    }
}
