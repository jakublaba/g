use clap::Parser;
use serde::ser::Error;

use crate::git::clone;
use crate::profile::profile::Profile;

mod cli;
mod ssh;
mod git;
mod profile;

// TODO (globally) improve error handling to also pass the underlying error messages
fn main() {
    // let cli = Cli::parse();
    // println!("{:#?}", cli);
    let profile = Profile {
        name: String::from("jakublaba"),
        user_name: String::from("jakublaba"),
        user_email: String::from("jakub.maciej.laba@gmail.com"),
    };
    // let (priv_key, pub_key) = generate_pair(&profile.name);
    // let randomart = randomart(&priv_key);
    // println!("{randomart}");
    // write_private_key(&profile.name, &priv_key).unwrap();
    // write_public_key(&profile.name, &pub_key).unwrap();
    clone(&profile.name, "git@github.com:jakublaba/git-multiaccount.git").unwrap();
}
