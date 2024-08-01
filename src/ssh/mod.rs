use std::path::Path;

use crate::ssh::error::Error;

pub mod key;
pub mod error;

type Result<T> = std::result::Result<T, error::Error>;

pub fn try_regenerate_pair(profile_name: &str, email: &str, force: bool) -> Result<()> {
    let path_priv = key::path_private(profile_name);
    let path_pub = key::path_public(profile_name);
    if !force && Path::new(&path_priv).exists() {
        if Path::new(&path_pub).exists() { Err(Error::KeyPairExists)? }
        println!("Private key found, attempting to re-generate public key");
        let pub_key = key::public_from_private(profile_name, email)?;
        return key::write_public(profile_name, &pub_key);
    }

    Ok(())
}
