use std::env;
use std::fmt::Display;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::{configure_user, whoami};
use crate::profile::{add_profile, edit_profile, profile_list, remove_profile, show_profile};
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
                configure_user(&profile, global)
            }
            Cmd::WhoAmI { global } => {
                if let Some(profile) = whoami(global) {
                    println!("{profile}");
                } else {
                    println!("No profile set");
                }
            }
            Cmd::Profile { command } => {
                if let Some(prof_cmd) = command {
                    match prof_cmd {
                        ProfileCmd::List => {
                            for profile in profile_list() {
                                println!("{profile}");
                            }
                        }
                        ProfileCmd::Show { name } => {
                            show_profile(&name);
                        }
                        ProfileCmd::Add { name, user_name, user_email, force, key_type } => {
                            let profile = Profile::new(name, user_name, user_email);
                            add_profile(profile, key_type, force);
                        }
                        ProfileCmd::Remove { profiles } => {
                            for p in profiles {
                                remove_profile(&p);
                            }
                        }
                        ProfileCmd::Edit { name, user_name, user_email } => {
                            edit_profile(name, user_name, user_email)
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

trait SafeUnwrap {
    fn safe_unwrap(self);
}

impl<T, E: Display> SafeUnwrap for Result<T, E> {
    fn safe_unwrap(self) {
        if let Err(e) = self {
            println!("{e}")
        }
    }
}
