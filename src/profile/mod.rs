use std::fmt::Display;
use std::fs;
use std::path::Path;

use const_format::formatcp;
use regex::Regex;

use crate::HOME;
use crate::profile::profile::{Profile, profile_path};
use crate::ssh;

const PROFILES_DIR: &str = formatcp!("{HOME}/.config/g-profiles");
pub mod profile;
pub mod cache;
pub mod error;

const PROFILE_REGEX: &str = r"g-profiles/(?<prof>[^\.]+)$";

type Result<T> = std::result::Result<T, error::Error>;

pub(crate) fn list() -> Vec<String> {
    let paths = fs::read_dir(PROFILES_DIR);
    let regex = Regex::new(PROFILE_REGEX).unwrap();
    return match paths {
        Ok(paths) => {
            // TODO there must be a way to clean up whatever the fuck is going on with this iterator
            paths.map(|r| r.unwrap())
                .map(|p| p.path())
                .map(|p| p.to_str().unwrap().to_string())
                .filter(|p| regex.is_match(p))
                .map(|p| regex.captures(&p)
                    .unwrap()
                    .name("prof")
                    .unwrap()
                    .as_str()
                    .into()
                )
                .collect()
        }
        Err(_) => vec![]
    };
}

pub(crate) fn remove(name: &str) -> Result<()> {
    rm_file(profile_path(name));
    rm_file(ssh::key::path_private(name));
    rm_file(ssh::key::path_public(name));

    cache::remove(name)
}

pub(crate) fn edit(name: &str, username: Option<String>, email: Option<String>) -> Result<()> {
    let mut profile = Profile::load(name)?;
    let old = profile.clone();
    let mut width = 0;
    let some_username = username.is_some();
    let some_email = email.is_some();
    if let Some(usr_name) = username {
        width = width.max(usr_name.len());
        profile.username = usr_name.to_string();
    };
    if let Some(usr_email) = email {
        width = width.max(usr_email.len());
        profile.email = usr_email.to_string();
    };
    if some_username {
        println!("[{name}] username:\t{:width$} -> {:width$}", old.username, profile.username, width = width);
    }
    if some_email {
        println!("[{name}] email:\t\t{:width$} -> {:width$}", old.email, profile.email, width = width);
    }


    profile.save(true)
}

fn rm_file<P: AsRef<Path> + Display>(path: P) {
    if let Err(_) = fs::remove_file(&path) {
        println!("Skipping, file doesn't exist: {path}");
    } else {
        println!("Removed {path}");
    }
}
