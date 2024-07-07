use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

use rand::thread_rng;
use ssh_key::{HashAlg, LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::model::Profile;

const SSH_KEYS_PATH: &str = "~/.ssh";
const RANDOMART_HEADER: &str = "ED25519";

pub type Result<T> = std::result::Result<T, SshError>;

#[derive(Debug)]
pub struct SshError {
    pub msg: String,
}

impl From<String> for SshError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for SshError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SshError {}

pub fn generate_keys(profile: &Profile) -> (PrivateKey, PublicKey) {
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(&profile.user_email);

    (private, public)
}

pub fn generate_randomart(key: &PrivateKey) -> String {
    let fingerprint = key.fingerprint(HashAlg::Sha256);
    fingerprint.to_randomart(RANDOMART_HEADER)
}

pub fn write_private_key(profile: &Profile, key: &PrivateKey) -> Result<()> {
    let path = private_key_path(&profile.name);
    key.write_openssh_file(Path::new(&path), LineEnding::LF)
        .map_err(|_| SshError::from(format!("Error writing private key: {path}")))
}

pub fn write_public_key(profile: &Profile, key: &PublicKey) -> Result<()> {
    let path = public_key_path(&profile.name);
    key.write_openssh_file(Path::new(&path))
        .map_err(|_| SshError::from(format!("Error writing public key: {path}")))
}

fn private_key_path(key_file_name: &str) -> String {
    let keys_dir = shellexpand::tilde(&SSH_KEYS_PATH);
    format!("{keys_dir}/id_{key_file_name}")
}

fn public_key_path(key_file_name: &str) -> String {
    let keys_dir = shellexpand::tilde(&SSH_KEYS_PATH);
    format!("{keys_dir}/id_{key_file_name}.pub")
}
