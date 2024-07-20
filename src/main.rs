use std::path::Path;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::clone;
use crate::profile::{generate_profile, remove_profile};
use crate::profile::profile::{Profile, profile_path};

mod cli;
mod ssh;
mod git;
mod profile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // TODO code inside this match is too big, extract things to functions
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
                            remove_profile(&profile);
                        }
                        ProfileCmd::Edit { name, user_name, user_email } => {
                            let path = profile_path(&name);
                            if !Path::new(&path).exists() {
                                panic!("Can't open profile: {path}")
                            }
                            let mut profile = Profile::read_json(&name)?;
                            profile.name = name;
                            if let Some(usr_name) = user_name { profile.user_name = usr_name };
                            if let Some(usr_email) = user_email { profile.user_email = usr_email };
                            profile.write_json()?;
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
