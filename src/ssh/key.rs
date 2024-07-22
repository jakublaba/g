use std::path::Path;

use rand::thread_rng;
use ssh_key::{LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;
use crate::home;
use crate::ssh::error::Error;
use crate::ssh::Result;

const SSH_DIR: &str = ".ssh";
pub const ED25519: &str = "ED25519";

// TODO add support for keys with passphrase
pub fn generate_pair(user_email: &str) -> (PrivateKey, PublicKey) {
    // TODO review whether thread_rng is cryptographically secure
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(user_email);

    (private, public)
}

pub fn write_private_key(profile_name: &str, key: &PrivateKey) -> Result<()> {
    let key_path = private_key_path(profile_name);
    key.write_openssh_file(Path::new(&key_path), LineEnding::LF)
        .map_err(|cause| Error::WriteKey { key_path, cause })
}

pub fn write_public_key(profile_name: &str, key: &PublicKey) -> Result<()> {
    let key_path = public_key_path(profile_name);
    key.write_openssh_file(Path::new(&key_path))
        .map_err(|cause| Error::WriteKey { key_path, cause })
}

pub fn private_key_path(profile_name: &str) -> String {
    let home = home();
    let ssh_dir = format!("{home}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}")
}

pub fn public_key_path(profile_name: &str) -> String {
    let home = home();
    let ssh_dir = format!("{home}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}.pub")
}
