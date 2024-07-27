use std::path::Path;

use rand::thread_rng;
use ssh_key::{LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::home;

pub(super) const SSH_DIR: &str = ".ssh";
pub(super) const DSA: &str = "DSA 1024";
pub(super) const RSA: &str = "RSA";
pub(super) const ED25519: &str = "ED25519";
const DEFAULT_RSA_SIZE: u32 = 3072;

#[derive(Debug)]
pub enum KeyType {
    Dsa,
    Rsa { size: u32 },
    Ecdsa { size: u32 },
    Ed25519,
}


// TODO add support for keys with passphrase
pub(super) fn pair(user_email: &str) -> (PrivateKey, PublicKey) {
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(user_email);

    (private, public)
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
