use std::path::Path;

use rand::thread_rng;
use ssh_key::{HashAlg, LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::model::Profile;

const SSH_KEYS_PATH: &str = "~/.ssh";
const RANDOMART_HEADER: &str = "ED25519";

// TODO implement custom SshError struct and change method signatures to Result<_, SshError>

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

pub fn write_private_key(profile: &Profile, key: &PrivateKey) {
    let path = private_key_path(&profile.name);
    key.write_openssh_file(Path::new(&path), LineEnding::LF)
        .expect(&format!("Error writing private key: {path}"));
}

pub fn write_public_key(profile: &Profile, key: &PublicKey) {
    let path = public_key_path(&profile.name);
    key.write_openssh_file(Path::new(&path))
        .expect(&format!("Error writing public key: {path}"));
}

fn private_key_path(key_file_name: &str) -> String {
    let keys_dir = shellexpand::tilde(&SSH_KEYS_PATH);
    format!("{keys_dir}/id_{key_file_name}")
}

fn public_key_path(key_file_name: &str) -> String {
    let keys_dir = shellexpand::tilde(&SSH_KEYS_PATH);
    format!("{keys_dir}/id_{key_file_name}.pub")
}
