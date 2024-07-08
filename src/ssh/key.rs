use std::path::Path;

use rand::thread_rng;
use ssh_key::{HashAlg, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::ssh::{Result, SshError};

const SSH_DIR: &str = "~/.ssh";
const ED25519: &str = "ED25519";

pub fn generate_pair(user_email: &str) -> (PrivateKey, PublicKey) {
    // TODO review whether thread_rng is cryptographically secure
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(user_email);

    (private, public)
}

pub fn randomart(key: &PrivateKey) -> String {
    let fingerprint = key.fingerprint(HashAlg::Sha256);
    fingerprint.to_randomart(ED25519)
}

pub fn write_private_key(profile_name: &str, key: &PrivateKey) -> Result<()> {
    let path = private_key_path(profile_name);
    key.write_openssh_file(Path::new(&path))
        .map_err(|_| SshError::from(format!("Error writing private key: {path}")))
}

pub fn write_public_key(profile_name: &str, key: &PublicKey) -> Result<()> {
    let path = public_key_path(profile_name);
    key.write_openssh_file(Path::new(&path))
        .map_err(|_| SshError::from(format!("Error writing public key: {path}")))
}

fn private_key_path(profile_name: &str) -> String {
    let ssh_dir = shellexpand::tilde(SSH_DIR);
    format!("{ssh_dir}/id_{profile_name}")
}

fn public_key_path(profile_name: &str) -> String {
    let ssh_dir = shellexpand::tilde(SSH_DIR);
    format!("{ssh_dir}/id_{profile_name}.pub")
}
