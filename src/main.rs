use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::clone;
use crate::profile::{edit_profile, generate_profile, remove_profile};
use crate::profile::profile::Profile;

mod cli;
mod ssh;
mod git;
mod profile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        match cmd {
            Cmd::Su { profile } => {
                git::configure_user(&profile)?;
            }
            Cmd::Profile { command } => {
                if let Some(prof_cmd) = command {
                    match prof_cmd {
                        ProfileCmd::Add { name, user_name, user_email } => {
                            let profile = Profile::new(name, user_name, user_email);
                            generate_profile(profile);
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
            Cmd::Clone { profile, url } => {
                clone(&profile, &url)?;
            }
        }
    };

    Ok(())
}
