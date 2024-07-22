use std::env;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::{configure_user, who_am_i};
use crate::profile::{add_profile, edit_profile, profile_list, remove_profile};
use crate::profile::profile::Profile;

mod cli;
mod ssh;
mod git;
mod profile;
mod util;

// TODO fix error handling
fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Cmd::Su { profile, global } => {
                if let Err(e) = configure_user(&profile, global) { panic!("{}", e.to_string()) }
            }
            Cmd::WhoAmI { global } => {
                let profile = who_am_i(global).unwrap();
                println!("{profile}");
            }
            Cmd::Profile { command } => {
                if let Some(prof_cmd) = command {
                    match prof_cmd {
                        ProfileCmd::List => {
                            for profile in profile_list() {
                                println!("{profile}");
                            }
                        }
                        ProfileCmd::Show { profile } => {
                            println!("{profile:#?}")
                        }
                        ProfileCmd::Add { name, user_name, user_email, force } => {
                            let profile = Profile::new(name, user_name, user_email);
                            add_profile(profile, force);
                        }
                        ProfileCmd::Remove { profile } => {
                            remove_profile(&profile)
                        }
                        ProfileCmd::Edit { name, user_name, user_email } => {
                            edit_profile(name, user_name, user_email)
                        }
                    }
                }
            }
        }
    };
}

pub fn home() -> String {
    env::var("HOME").unwrap()
}
