use std::path::Path;

use rand::thread_rng;
use ssh_key::{HashAlg, LineEnding, PrivateKey, PublicKey};
use ssh_key::private::Ed25519Keypair;

use crate::home;

const SSH_DIR: &str = ".ssh";
pub const ED25519: &str = "ED25519";

pub fn generate_key_pair(profile_name: &str, user_email: &str, force: bool) {
    println!("Generating a new ssh-ed25519 key pair");
    let private_path = private_key_path(profile_name);
    let public_path = public_key_path(profile_name);
    if !force && Path::new(&private_path).exists() {
        if Path::new(&public_path).exists() {
            println!("Key pair already exists, re-run with --force if you want to re-generate it");
            return;
        }
        println!("Found private key, re-generating public key from it.");
        regenerate_public_from_private(profile_name, user_email);
        return;
    }
    let (private, public) = generate_pair_from_scratch(&user_email);
    write_private_key(profile_name, &private);
    write_public_key(profile_name, &public);
    println!("Key pair written");
    let fingerprint = private.fingerprint(HashAlg::Sha256);
    let randomart = fingerprint.to_randomart(ED25519);
    println!("The key fingerprint is:\n{fingerprint}");
    println!("The key's randomart image is:\n{randomart}");
}

// TODO add support for keys with passphrase
fn generate_pair_from_scratch(user_email: &str) -> (PrivateKey, PublicKey) {
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);
    let private = PrivateKey::from(pair);
    let mut public = PublicKey::from(&private);
    public.set_comment(user_email);

    (private, public)
}

fn regenerate_public_from_private(profile_name: &str, user_email: &str) {
    let private_key_path = private_key_path(profile_name);
    match PrivateKey::read_openssh_file(Path::new(&private_key_path)) {
        Ok(private) => {
            let mut public = PublicKey::from(&private);
            public.set_comment(user_email);
            write_public_key(profile_name, &mut public);
        }
        Err(_) => println!("Error reading private key: {private_key_path}")
    }
}

fn write_private_key(profile_name: &str, key: &PrivateKey) {
    let key_path = private_key_path(profile_name);
    if let Err(_) = key.write_openssh_file(Path::new(&key_path), LineEnding::LF) {
        println!("Error writing private key: {key_path}");
    }
}

fn write_public_key(profile_name: &str, key: &PublicKey) {
    let key_path = public_key_path(profile_name);
    if let Err(_) = key.write_openssh_file(Path::new(&key_path)) {
        eprintln!("Error writing public key: {key_path}");
    }
}

pub(crate) fn private_key_path(profile_name: &str) -> String {
    let home = home();
    let ssh_dir = format!("{home}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}")
}

pub(crate) fn public_key_path(profile_name: &str) -> String {
    let home = home();
    let ssh_dir = format!("{home}/{SSH_DIR}");
    format!("{ssh_dir}/id_{profile_name}.pub")
}
