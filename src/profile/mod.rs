use std::fmt::Display;
use std::fs;
use std::path::Path;

use regex::Regex;

use crate::{SafeUnwrap, ssh};
use crate::profile::profile::{Profile, profile_path, profiles_dir};
use crate::ssh::key::KeyType;

pub mod profile;
pub mod cache;

const PROFILE_REGEX: &str = r"g-profiles/(?<prof>[^\.]+)$";

pub fn profile_list() -> Vec<String> {
    let profiles_dir = profiles_dir();
    let paths = fs::read_dir(&profiles_dir);
    let regex = Regex::new(PROFILE_REGEX).unwrap();
    return match paths {
        Ok(paths) => {
            paths.map(Result::unwrap)
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

pub fn show_profile(profile_name: &str) {
    if let Ok(profile) = Profile::read_json(profile_name) {
        println!("{profile}");
    } else {
        println!("Profile '{profile_name}' not found");
    }
}

pub fn add_profile(profile: Profile, key_type: KeyType, force: bool) {
    if profile.name.starts_with('.') {
        println!("Profile name cannot start with '.'");
        return;
    }
    if let Some(p) = cache::get(&profile.user_name, &profile.user_email) {
        println!("Profile with this combination of username/email already exists: {p}");
        return;
    }
    let profile_name = profile.name.clone();
    let user_email = profile.user_email.clone();
    cache::insert(&profile).unwrap();
    generate_profile(profile, force);
    ssh::generate_key_pair(&profile_name, &user_email, key_type, force).safe_unwrap();
}

fn generate_profile(profile: Profile, force: bool) {
    let profile_path = profile_path(&profile.name);
    if Path::new(&profile_path).exists() && !force {
        println!("Profile '{}' already exists, if you want to override it, re-run with --force", &profile.name);
    } else {
        let profiles_dir = profiles_dir();
        if !Path::new(&profiles_dir).exists() {
            fs::create_dir_all(profiles_dir).unwrap();
        }
        if let Err(_) = profile.write_json() {
            println!("Cannot write profile: {profile_path}");
        }
        println!("Profile written");
    }
}

pub fn remove_profile(profile_name: &str) {
    rm_file(profile_path(profile_name));
    rm_file(ssh::key::path_private(profile_name));
    rm_file(ssh::key::path_public(profile_name));
    cache::remove(profile_name).unwrap();
}

pub fn edit_profile(name: String, user_name: Option<String>, user_email: Option<String>) {
    match Profile::read_json(&name) {
        Ok(mut profile) => {
            let user_name_old = profile.user_name.clone();
            let user_email_old = profile.user_email.clone();
            let mut width = 0;
            if let Some(usr_name) = user_name {
                width = width.max(usr_name.len());
                profile.user_name = usr_name
            };
            if let Some(usr_email) = user_email {
                width = width.max(usr_email.len());
                profile.user_email = usr_email
            };
            println!("[{name}] username:\t{user_name_old:width$} -> {:width$}", profile.user_name, width = width);
            println!("[{name}] email:\t\t{user_email_old:width$} -> {:width$}", profile.user_email, width = width);
            if let Err(e) = profile.write_json() {
                println!("{e}");
            }
        }
        Err(_) => println!("Profile '{name}' doesn't exist")
    }
}

fn rm_file<P: AsRef<Path> + Display>(path: P) {
    if let Err(_) = fs::remove_file(&path) {
        println!("Skipping, file doesn't exist: {path}");
    } else {
        println!("Removed {path}");
    }
}
