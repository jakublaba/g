use std::fmt::{Display, Formatter};
use std::path::Path;

use anyhow::{anyhow, Result};
use rand::thread_rng;
use ssh_key::{LineEnding, PrivateKey, PublicKey};
use ssh_key::private::{DsaKeypair, Ed25519Keypair, RsaKeypair};

use crate::home;

pub(super) const SSH_DIR: &str = ".ssh";
const DEFAULT_RSA_SIZE: usize = 3072;
const MIN_RSA_SIZE: usize = 1024;
const DEFAULT_ECDSA_SIZE: usize = 256;

pub(super) trait RandomartHeader {
    fn header(&self) -> String;
}

pub enum KeyType {
    Dsa,
    Rsa { size: Option<usize> },
    Ed25519,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            KeyType::Dsa => "dsa",
            KeyType::Rsa { .. } => "rsa",
            KeyType::Ed25519 => "ed25519",
        })
    }
}

impl RandomartHeader for KeyType {
    fn header(&self) -> String {
        match self {
            KeyType::Dsa => "DSA 1024".to_string(),
            KeyType::Rsa { size } => format!("RSA {}", size.unwrap_or(DEFAULT_RSA_SIZE)),
            KeyType::Ed25519 => "ED25519".to_string()
        }
    }
}

// it's criminal these don't already have a common interface in the lib
enum KeyPair {
    Dsa(DsaKeypair),
    Rsa(RsaKeypair),
    Ed25519(Ed25519Keypair),
}

impl From<KeyPair> for PrivateKey {
    fn from(pair: KeyPair) -> Self {
        match pair {
            KeyPair::Dsa(pair) => PrivateKey::from(pair),
            KeyPair::Rsa(pair) => PrivateKey::from(pair),
            KeyPair::Ed25519(pair) => PrivateKey::from(pair),
        }
    }
}

pub(super) fn pair(user_email: &str, key_type: &KeyType) -> Result<(PrivateKey, PublicKey)> {
    let mut rng = thread_rng();
    let pair = match key_type {
        KeyType::Dsa => {
            KeyPair::Dsa(DsaKeypair::random(&mut rng)?)
        }
        KeyType::Rsa { size } => {
            let size = size.unwrap_or(DEFAULT_RSA_SIZE);
            if size < MIN_RSA_SIZE {
                Err(anyhow!("Invalid RSA key length: minimum is {MIN_RSA_SIZE} bits"))?
            }
            KeyPair::Rsa(RsaKeypair::random(&mut rng, size)?)
        }
        KeyType::Ed25519 => KeyPair::Ed25519(Ed25519Keypair::random(&mut rng))
    };
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(user_email);

    Ok((private, public))
}

pub(super) fn public_from_private(profile_name: &str, user_email: &str) -> Option<PublicKey> {
    let private_key_path = path_private(profile_name);
    PrivateKey::read_openssh_file(Path::new(&private_key_path))
        .map(|private| {
            let mut public = PublicKey::from(&private);
            public.set_comment(user_email);
            public
        })
        .ok()
}

pub(super) fn write_private(profile_name: &str, key: &PrivateKey) {
    let key_path = path_private(profile_name);
    if let Err(_) = key.write_openssh_file(Path::new(&key_path), LineEnding::LF) {
        println!("Error writing private key: {key_path}");
    }
}

pub(super) fn write_public(profile_name: &str, key: &PublicKey) {
    let key_path = path_public(profile_name);
    if let Err(_) = key.write_openssh_file(Path::new(&key_path)) {
        eprintln!("Error writing public key: {key_path}");
    }
}

pub(crate) fn path_private(profile_name: &str) -> String {
    let home = home();
    let ssh_dir = format!("{home}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}")
}

pub(crate) fn path_public(profile_name: &str) -> String {
    let home = home();
    let ssh_dir = format!("{home}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}.pub")
}
