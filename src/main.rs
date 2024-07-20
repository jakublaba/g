use std::fs;
use std::path::Path;

use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::git::clone;
use crate::profile::profile::{Profile, profile_path};
use crate::ssh::key::{generate_pair, private_key_path, public_key_path, randomart, write_private_key, write_public_key};

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
                            println!("Generating a new ssh-ed25519 key pair");
                            let (private, public) = generate_pair(&profile.user_email);
                            write_private_key(&profile.name, &private)?;
                            write_public_key(&profile.name, &public)?;
                            let randomart = randomart(&private);
                            println!("Key pair written, private key fingerprint randomart:\n{randomart}");
                            profile.write_json()?;
                            println!("Profile written");
                        }
                        ProfileCmd::Remove { profile } => {
                            let path = profile_path(&profile);
                            fs::remove_file(path)?;
                            let private_key_path = private_key_path(&profile);
                            let public_key_path = public_key_path(&profile);
                            fs::remove_file(private_key_path)?;
                            fs::remove_file(public_key_path)?;
                            println!("Removed profile '{profile}' and associated ssh keys");
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
