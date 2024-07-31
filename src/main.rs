use std::env;
use std::fmt::Display;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::{configure_user, whoami};
use crate::profile::{add_profile, edit_profile, profile_list, remove_profile};
use crate::profile::profile::Profile;

mod cli;
mod ssh;
mod git;
mod profile;

// TODO fix error handling
fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
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
            Cmd::Profile { command: prof_cmd } => {
                if let Some(prof_cmd) = prof_cmd {
                    match prof_cmd {
                        ProfileCmd::List => {
                            for profile in profile_list() {
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
                            let profile = Profile::new(name, user_name, user_email);
                            add_profile(profile, key_type, force).safe_unwrap();
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
    };
}

// TODO could it actually be safe to just evaluate $HOME at compile time?
pub fn home() -> String {
    env::var("HOME").unwrap()
}

// TODO should this stay in main?
trait SafeUnwrap {
    fn safe_unwrap(self);
}

impl<E: Display> SafeUnwrap for Result<(), E> {
    fn safe_unwrap(self) {
        if let Err(e) = self {
            println!("{e}")
        }
    }
}
