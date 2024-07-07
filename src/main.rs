use clap::Parser;

use crate::model::Profile;
use crate::ssh::{generate_keys, write_private_key, write_public_key};

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
    write_private_key(&profile, &priv_key);
    write_public_key(&profile, &pub_key);
}
