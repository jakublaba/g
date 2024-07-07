use std::any::Any;

use clap::Parser;
use serde::ser::Error;

use crate::model::Profile;
use crate::ssh::{generate_keys, generate_randomart, write_private_key, write_public_key};

mod cli;
mod model;
mod ssh;

fn main() {
    // let cli = Cli::parse();
    // println!("{:#?}", cli);
    let profile = Profile {
        name: String::from("johnsmith"),
        user_name: String::from("John Smith"),
        user_email: String::from("john.smith@example.com"),
    };
    let (priv_key, pub_key) = generate_keys(&profile);
    let randomart = generate_randomart(&priv_key);
    println!("{randomart}");
    write_private_key(&profile, &priv_key).unwrap();
    write_public_key(&profile, &pub_key).unwrap();
}
