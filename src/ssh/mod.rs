use std::path::Path;

use ssh_key::HashAlg;

use crate::ssh::key::{KeyType, RandomartHeader};

pub mod key;
mod error;

type Result<T> = std::result::Result<T, error::Error>;

// TODO chop this func up
pub fn generate_key_pair(profile_name: &str, user_email: &str, key_type: KeyType, force: bool) -> Result<()> {
    println!("Generating a new ssh-{key_type} key pair");
    let private_path = key::path_private(profile_name);
    let public_path = key::path_public(profile_name);
    if !force && Path::new(&private_path).exists() {
        if Path::new(&public_path).exists() {
            println!("Key pair already exists, re-run with --force if you want to re-generate it");
            return Ok(());
        }
        println!("Found private key, re-generating public key from it.");
        match key::public_from_private(profile_name, user_email) {
            Some(key) => key::write_public(profile_name, &key),
            None => println!("Can't read private key for profile: '{profile_name}'")
        }
        return Ok(());
    }
    let (private, public) = key::pair(&user_email, &key_type)?;
    key::write_private(profile_name, &private);
    key::write_public(profile_name, &public);
    println!("Key pair written");
    let fingerprint = private.fingerprint(HashAlg::Sha256);
    let randomart = fingerprint.to_randomart(&key_type.header());
    println!("The key fingerprint is:\n{fingerprint}");
    println!("The key's randomart image is:\n{randomart}");

    Ok(())
}
