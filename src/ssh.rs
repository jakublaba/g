// TODO improve error handling

use std::fs::File;
use std::io::{BufWriter, Write};

use rand::thread_rng;
use ssh_encoding::Encode;
use ssh_key::private::{Ed25519Keypair, Ed25519PrivateKey};
use ssh_key::public::Ed25519PublicKey;
use ssh_key::sha2::{Digest, Sha256};

use crate::model::Profile;

const SSH_KEYS_PATH: &str = "~/.ssh";

// TODO implement custom SshError struct and change method signatures to Result<_, SshError>

pub fn generate_keys() -> (Ed25519PrivateKey, Ed25519PublicKey) {
    let mut rng = thread_rng();
    let pair = Ed25519Keypair::random(&mut rng);

    (pair.private, pair.public)
}

pub fn write_private_key(profile: &Profile, key: &Ed25519PrivateKey) {
    let profile_name = &profile.name;
    let file_name = private_key_path(profile_name);
    let file = File::create(&file_name)
        .expect(&format!("Error creating file: {file_name}"));
    let mut buf_writer = BufWriter::new(file);
    // todo figure out the encoding skill issue
    let mut sha256_writer = Sha256::new();
    key.encode(&mut sha256_writer)
        .expect(&format!("Error encoding private key: {file_name}"));
    buf_writer.write_all(key.as_ref())
        .expect(&format!("Error writing private key: {file_name}"));
}

pub fn write_public_key(profile: &Profile, key: &Ed25519PublicKey) {
    let profile_name = &profile.name;
    let file_name = public_key_path(profile_name);
    let file = File::create(&file_name)
        .expect(&format!("Error creating file: {file_name}"));
    let mut buf_writer = BufWriter::new(file);
    let mut enc_writer = Sha256::new();
    // todo figure out the encoding skill issue
    key.encode(&mut enc_writer)
        .expect(&format!("Error encoding public key: {file_name}"));
    buf_writer.write_all(key.as_ref())
        .expect(&format!("Error writing public key: {file_name}"));
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
