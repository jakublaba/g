use std::fmt::Display;
use std::fs;
use std::path::Path;

use ssh_key::HashAlg;

use crate::profile::error::Error;
use crate::profile::profile::{Profile, profile_path};
use crate::ssh::key::{ED25519, generate_pair, private_key_path, public_key_path, write_private_key, write_public_key};

pub mod profile;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;

// TODO make this not generate any files if any of the stages fails
pub fn generate_profile(profile: Profile) {
    println!("Generating a new ssh-ed25519 key pair");
    let (private, public) = generate_pair(&profile.user_email);
    if let Err(e) = write_private_key(&profile.name, &private) { panic!("{}", e.to_string()) }
    if let Err(e) = write_public_key(&profile.name, &public) { panic!("{}", e.to_string()) }
    println!("Key pair written");
    let fingerprint = private.fingerprint(HashAlg::Sha256);
    let randomart = fingerprint.to_randomart(ED25519);
    println!("The key fingerprint is:\n{fingerprint}");
    println!("They key's randomart image is:\n{randomart}");
    if let Err(e) = profile.write_json() { panic!("{}", e.to_string()) }
    println!("Profile written");
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

fn rm_file<P: AsRef<Path> + Display>(path: P) {
    println!("Removing {path}");
    if let Err(_) = fs::remove_file(&path) {
        println!("{path} doesn't exist, skipping");
    } else {
        println!("{path} removed");
    }
}
