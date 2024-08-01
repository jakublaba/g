use std::env;
use std::fmt::Display;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::{configure_user, whoami};
use crate::profile::{add, edit, load_profile_list, remove};
use crate::profile::profile::Profile;
use crate::util::SafeUnwrap;

mod cli;
mod ssh;
mod git;
mod profile;
mod util;

// TODO fix error handling
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Su { profile, global } => {
            configure_user(&profile, global).safe_unwrap();
        }
        Cmd::WhoAmI { global } => {
            if let Some(profile) = whoami(global) {
                println!("{profile}");
            } else {
                println!("No profile set");
            }
        }
        Cmd::Profile { command } => {
            match command {
                ProfileCmd::List => {
                    for profile in load_profile_list() {
                        println!("{profile}");
                    }
                }
                ProfileCmd::Show { name } => {
                    match Profile::read(&name) {
                        Ok(p) => println!("{p}"),
                        Err(e) => println!("{e}"),
                    }
                }
                ProfileCmd::Add { name, username, email, key_type, force } => {
                    add(name, username, email, key_type, force).safe_unwrap();
                }
                ProfileCmd::Remove { profiles } => {
                    for p in profiles {
                        remove(&p).safe_unwrap();
                    }
                }
                ProfileCmd::Edit { name, username: user_name, email: user_email } => {
                    edit(name, user_name, user_email).safe_unwrap();
                }
                ProfileCmd::Regenerate { profile, key_type } => {
                    ssh::generate_key_pair(&profile.name, &profile.email, key_type, true).safe_unwrap();
                }
            }
        }
    }
}

// TODO could it actually be safe to just evaluate $HOME at compile time?
pub fn home() -> String {
    env::var("HOME").unwrap()
}
