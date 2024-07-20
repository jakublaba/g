use std::fs;
use std::path::Path;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::clone;
use crate::profile::profile::{Profile, profile_path};

mod cli;
mod ssh;
mod git;
mod profile;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("{:#?}", cli);

    if let Some(cmd) = cli.command {
        match cmd {
            Cmd::Su { profile } => {
                git::configure_user(&profile)?;
            }
            Cmd::Profile { command } => {
                if let Some(prof_cmd) = command {
                    match prof_cmd {
                        ProfileCmd::Add { name, user_name, user_email } => {
                            Profile::new(name, user_name, user_email).write_json()?;
                        }
                        ProfileCmd::Remove { profile } => {
                            let path = profile_path(&profile);
                            fs::remove_file(path)?;
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
