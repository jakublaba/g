use std::fs;
use std::path::Path;

use anyhow::Result;
use regex::Regex;

use crate::profile::profile::{Profile, profile_path, profiles_dir};
use crate::ssh::{generate_key_pair, private_key_path, public_key_path};
use crate::util::rm_file;

pub mod profile;
const PROFILE_REGEX: &str = r"g-profiles/(?<prof>.+)\.json";

pub fn profile_list() -> Vec<String> {
    let profiles_dir = profiles_dir();
    let paths = fs::read_dir(&profiles_dir);
    let regex = Regex::new(PROFILE_REGEX).unwrap();
    return match paths {
        Ok(paths) => {
            paths.map(|p| { p.unwrap() })
                .map(|p| p.path())
                .map(|p| String::from(p.to_str().unwrap()))
                .map(|p| regex.captures(&p)
                    .expect("path doesn't match regex")
                    .name("prof")
                    .expect("can't extract profile name from path")
                    .as_str()
                    .into()
                )
                .collect()
        }
        Err(_) => vec![]
    };
}

pub fn add_profile(profile: Profile, force: bool) {
    let profile_name = profile.name.clone();
    let user_email = profile.user_email.clone();
    generate_profile(profile, force);
    generate_key_pair(&profile_name, &user_email, force);
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
    rm_file(private_key_path(profile_name));
    rm_file(public_key_path(profile_name));
}

pub fn edit_profile(name: String, user_name: Option<String>, user_email: Option<String>) {
    let path = profile_path(&name);
    if !Path::new(&path).exists() {
        panic!("Can't open {path}");
    }
    match Profile::read_json(&name) {
        Ok(mut profile) => {
            profile.name = name;
            if let Some(usr_name) = user_name { profile.user_name = usr_name };
            if let Some(usr_email) = user_email { profile.user_email = usr_email };
        }
        Err(e) => { panic!("{}", e.to_string()) }
    }
}
