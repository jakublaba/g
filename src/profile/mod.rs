use std::fmt::Display;
use std::fs;
use std::path::Path;

use regex::Regex;
use ssh_key::HashAlg;

use crate::profile::error::Error;
use crate::profile::profile::{Profile, profile_path, profiles_dir};
use crate::ssh::key::{ED25519, generate_pair, key_pair_exists, private_key_path, public_key_path, regenerate_public_from_private, write_private_key, write_public_key};

pub mod profile;
pub mod error;

const PROFILE_REGEX: &str = r"g-profiles/(?<prof>.+)\.json";
pub type Result<T> = std::result::Result<T, Error>;

pub fn list_profiles() -> Vec<String> {
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

// TODO make this not generate any files if any of the stages fails
pub fn generate_profile(profile: Profile, force: bool) {
    let profile_name = profile.name.clone();
    let user_email = profile.user_email.clone();
    let profile_path = profile_path(&profile_name);
    if Path::new(&profile_path).exists() && !force {
        println!("Profile '{profile_name}' already exists, if you want to override it, re-run with --force");
    } else {
        let profiles_dir = profiles_dir();
        if !Path::new(&profiles_dir).exists() {
            fs::create_dir_all(profiles_dir).unwrap();
        }
        if let Err(e) = profile.write_json() { panic!("{}", e.to_string()) }
        println!("Profile written");
    }
    println!("Generating a new ssh-ed25519 key pair");
    let private_path = private_key_path(&profile_name);
    if !force {
        if Path::new(&private_path).exists() {
            println!("Found private key, re-generating public key from it.");
            regenerate_public_from_private(&profile_name).unwrap();
        } else {
            println!("ssh keys for profile '{profile_name}' already exist, if you want to re-generate them, re-run with --force");
        }
        return;
    }
    let (private, public) = generate_pair(&user_email);
    if let Err(e) = write_private_key(&profile_name, &private) { panic!("{}", e.to_string()) }
    if let Err(e) = write_public_key(&profile_name, &public) { panic!("{}", e.to_string()) }
    println!("Key pair written");
    let fingerprint = private.fingerprint(HashAlg::Sha256);
    let randomart = fingerprint.to_randomart(ED25519);
    println!("The key fingerprint is:\n{fingerprint}");
    println!("They key's randomart image is:\n{randomart}");
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

// TODO this should be moved to some util module
fn rm_file<P: AsRef<Path> + Display>(path: P) {
    println!("Removing {path}");
    if let Err(_) = fs::remove_file(&path) {
        println!("{path} doesn't exist, skipping");
    } else {
        println!("{path} removed");
    }
}
