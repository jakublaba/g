use std::fmt::Display;
use std::fs;
use std::path::Path;

use regex::Regex;

use crate::profile::profile::{Profile, profile_path, profiles_dir};
use crate::ssh;

pub mod profile;
pub mod cache;
pub mod error;

const PROFILE_REGEX: &str = r"g-profiles/(?<prof>[^\.]+)$";

type Result<T> = std::result::Result<T, error::Error>;

pub fn load_profile_list() -> Vec<String> {
    let profiles_dir = profiles_dir();
    let paths = fs::read_dir(&profiles_dir);
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

pub fn remove(name: &str) -> Result<()> {
    rm_file(profile_path(name));
    rm_file(ssh::key::path_private(name));
    rm_file(ssh::key::path_public(name));

    cache::remove(name)
}

pub fn edit(name: &str, user_name: &Option<String>, user_email: &Option<String>) -> Result<()> {
    let mut profile = Profile::read(&name)?;
    let user_name_old = profile.username.clone();
    let user_email_old = profile.email.clone();
    let mut width = 0;
    if let Some(usr_name) = user_name {
        width = width.max(usr_name.len());
        profile.username = usr_name.to_string();
        println!("[{name}] username:\t{user_name_old:width$} -> {:width$}", profile.username, width = width);
    };
    if let Some(usr_email) = user_email {
        width = width.max(usr_email.len());
        profile.email = usr_email.to_string();
        println!("[{name}] email:\t\t{user_email_old:width$} -> {:width$}", profile.email, width = width);
    };

    profile.write(true)
}

fn rm_file<P: AsRef<Path> + Display>(path: P) {
    if let Err(_) = fs::remove_file(&path) {
        println!("Skipping, file doesn't exist: {path}");
    } else {
        println!("Removed {path}");
    }
}
