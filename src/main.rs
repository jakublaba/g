use clap::Parser;
use serde::ser::Error;

use crate::profile::profile::Profile;
use crate::ssh::key::{generate_pair, randomart, write_private_key, write_public_key};

mod cli;
mod ssh;
mod git;
mod profile;

// TODO (globally) improve error handling to also pass the underlying error messages
fn main() {
    // let cli = Cli::parse();
    // println!("{:#?}", cli);
    let profile = Profile {
        name: String::from("johnsmith"),
        user_name: String::from("John Smith"),
        user_email: String::from("john.smith@example.com"),
    };
    let (priv_key, pub_key) = generate_pair(&profile.name);
    let randomart = randomart(&priv_key);
    println!("{randomart}");
    write_private_key(&profile.name, &priv_key).unwrap();
    write_public_key(&profile.name, &pub_key).unwrap();
}
