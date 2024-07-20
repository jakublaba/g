use ssh_key::HashAlg;

use crate::profile::error::Error;
use crate::profile::profile::Profile;
use crate::ssh::key::{ED25519, generate_pair, write_private_key, write_public_key};

pub mod profile;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;

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
