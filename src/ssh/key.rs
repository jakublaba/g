use std::fmt::{Display, Formatter};
use std::path::Path;

use rand::thread_rng;
use ssh_key::{LineEnding, PrivateKey, PublicKey};
use ssh_key::private::{DsaKeypair, Ed25519Keypair, RsaKeypair};

use crate::HOME;
use crate::ssh::error::Error;
use crate::ssh::Result;

pub(super) const SSH_DIR: &str = ".ssh";
pub(super) const DEFAULT_RSA_SIZE: usize = 3072;
pub(super) const MIN_RSA_SIZE: usize = 2048;

pub trait RandomartHeader {
    fn header(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum KeyType {
    Dsa,
    Rsa { size: Option<usize> },
    Ed25519,
}

impl KeyType {
    pub fn parse(arg: &str) -> Result<Self> {
        match arg.to_lowercase().as_str() {
            "dsa" => Ok(Self::Dsa),
            s if s.starts_with("rsa") => {
                let size = (&s[3..s.len()]).parse::<usize>().ok();
                Ok(Self::Rsa { size })
            }
            "ed25519" => Ok(Self::Ed25519),
            s => Err(Error::UnknownKeyType(s.to_string()))
        }
    }
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

pub fn pair(user_email: &str, key_type: &KeyType) -> Result<(PrivateKey, PublicKey)> {
    let mut rng = thread_rng();
    let pair = match key_type {
        KeyType::Dsa => {
            KeyPair::Dsa(DsaKeypair::random(&mut rng)?)
        }
        KeyType::Rsa { size } => {
            let size = size.unwrap_or(DEFAULT_RSA_SIZE);
            if size < MIN_RSA_SIZE {
                Err(Error::InvalidRsaLength(size))?
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

pub(super) fn public_from_private(profile_name: &str, user_email: &str) -> Result<PublicKey> {
    let private_key_path = path_private(profile_name);
    PrivateKey::read_openssh_file(Path::new(&private_key_path))
        .map(|private| {
            let mut public = PublicKey::from(&private);
            public.set_comment(user_email);
            public
        })
        .map_err(|e| e.into())
}

pub fn write_private(profile_name: &str, key: &PrivateKey) -> Result<()> {
    let key_path = path_private(profile_name);
    key.write_openssh_file(Path::new(&key_path), LineEnding::LF)
        .map_err(|e| e.into())
}

pub fn write_public(profile_name: &str, key: &PublicKey) -> Result<()> {
    let key_path = path_public(profile_name);
    key.write_openssh_file(Path::new(&key_path))
        .map_err(|e| e.into())
}

pub(crate) fn path_private(profile_name: &str) -> String {
    let ssh_dir = format!("{HOME}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}")
}

pub(crate) fn path_public(profile_name: &str) -> String {
    let ssh_dir = format!("{HOME}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}.pub")
}
