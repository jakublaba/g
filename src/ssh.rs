use std::path::Path;

use rand::thread_rng;
use ssh_key::{LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::model::Profile;

const SSH_KEYS_PATH: &str = "~/.ssh";

// TODO implement custom SshError struct and change method signatures to Result<_, SshError>

pub fn generate_keys(profile: &Profile) -> (PrivateKey, PublicKey) {
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(&profile.user_email);

    (private, public)
}

pub fn write_private_key(profile_name: &str, key: &PrivateKey) {
    let path = private_key_path(profile_name);
    key.write_openssh_file(Path::new(&path), LineEnding::LF)
        .expect(&format!("Error writing private key: {path}"));
}

pub fn write_public_key(profile: &Profile, key: &PublicKey) {
    let profile_name = &profile.name;
    let path = public_key_path(profile_name);
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

// TODO implement showing private key randomart
