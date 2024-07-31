use std::env;
use std::fmt::Display;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::{configure_user, whoami};
use crate::profile::{add_profile, edit_profile, load_profile_list, remove_profile};
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
                    match Profile::read_json(&name) {
                        Ok(p) => println!("{p}"),
                        Err(e) => println!("{e}"),
                    }
                }
                ProfileCmd::Add { name, user_name, user_email, force, key_type } => {
                    add_profile(name, user_name, user_email, key_type, force).safe_unwrap();
                }
                ProfileCmd::Remove { profiles } => {
                    for p in profiles {
                        remove_profile(&p).safe_unwrap();
                    }
                }
                ProfileCmd::Edit { name, user_name, user_email } => {
                    edit_profile(name, user_name, user_email).safe_unwrap();
                }
                ProfileCmd::Regenerate { profile, key_type } => {
                    ssh::generate_key_pair(&profile.name, &profile.user_email, key_type, true).safe_unwrap();
                }
            }
        }
    }
}

// TODO could it actually be safe to just evaluate $HOME at compile time?
pub fn home() -> String {
    env::var("HOME").unwrap()
}
