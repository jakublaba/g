use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use rand::thread_rng;
use ssh_key::{HashAlg, LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::model::Profile;

const SSH_KEYS_PATH: &str = "~/.ssh";
const SSH_CONFIG_PATH: &str = "~/.ssh/config";
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

impl From<&str> for SshError {
    fn from(msg: &str) -> Self {
        Self { msg: String::from(msg) }
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

pub fn add_config_entry(profile: &Profile) -> Result<()> {
    let path = ssh_config_path();
    let content = filtered_ssh_config(&profile.name)? + &config_entry(&profile.name);
    fs::write(&path, content)
        .map_err(|_| SshError::from(format!("Error appending entry for profile {} to ssh config", &profile.name)))
}

pub fn remove_config_entry(profile: &Profile) -> Result<()> {
    let content = filtered_ssh_config(&profile.name)?;
    fs::write(&ssh_config_path(), content)
        .map_err(|_| SshError::from(format!("Error removing entry for profile {} from ssh config", &profile.name)))
}

fn private_key_path(key_file_name: &str) -> String {
    let keys_dir = shellexpand::tilde(&SSH_KEYS_PATH);
    format!("{keys_dir}/id_{key_file_name}")
}

fn public_key_path(key_file_name: &str) -> String {
    let keys_dir = shellexpand::tilde(&SSH_KEYS_PATH);
    format!("{keys_dir}/id_{key_file_name}.pub")
}

fn ssh_config_path() -> String {
    String::from(shellexpand::tilde(SSH_CONFIG_PATH))
}

fn filtered_ssh_config(excluded_profile_name: &str) -> Result<String> {
    let path = ssh_config_path();
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|_| SshError::from(format!("Error opening ssh config: {path}")))?;
    let reader = BufReader::new(file);
    let config_entry_lines = config_entry(excluded_profile_name)
        .lines()
        .map(String::from)
        .collect::<HashSet<_>>();
    let content = reader.lines()
        .map(|r| r.unwrap())
        .filter(|line| !config_entry_lines.contains(line))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(content)
}

fn config_entry(profile_name: &str) -> String {
    format!(r#"
Host github.com-{profile_name}
    HostName        github.com
    User            git
    IdentityFile    ~/.ssh/id_{profile_name}
"#)
}
