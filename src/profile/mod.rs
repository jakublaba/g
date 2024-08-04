use std::fs;

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
///
/// ```
/// let names = list().expect("Can't load list of profile names");
/// for profile_name in names {
///     println!("{profile_name}");
/// }
/// ```
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

/// Removes profile with chosen `name` from [`PROFILES_DIR`] and profile cache.
///
/// ```
/// let profile = "example";
/// remove(profile).expect(&format!("Can't remove {profile}"));
/// ```
pub fn remove(name: &str) -> Result<()> {
    fs::remove_file(profile_path(name))?;
    fs::remove_file(ssh::key::path_private(name))?;
    fs::remove_file(ssh::key::path_public(name))?;

    cache::remove(name)
}

/// Changes `username` and/or `email` for profile with specified `name`
///
/// ```
/// let profile = "example";
/// edit(profile, None, Some("new@email.com".to_string()))).expect(&format!("Can't edit {profile}"));
/// ```
pub fn edit(name: &str, username: Option<String>, email: Option<String>) -> Result<()> {
    let mut profile = Profile::load(name)?;
    if let Some(usr_name) = username {
        profile.username = usr_name.to_string();
    };
    if let Some(usr_email) = email {
        profile.email = usr_email.to_string();
    };

    profile.save(true)
}
