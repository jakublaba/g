use ssh_key::HashAlg;

use crate::{git, profile, ssh};
use crate::cli::{Cli, Cmd, ProfileCmd};
use crate::cli::error::Error;
use crate::cli::Result;
use crate::profile::profile::Profile;
use crate::ssh::key::r#type::{KeyType, RandomArtHeader};

pub(crate) trait Presentation {
    fn present(self) -> Result<()>;
}

impl Presentation for Cli {
    fn present(self) -> Result<()> {
        self.command.present()
    }
}

impl Presentation for Cmd {
    fn present(self) -> Result<()> {
        match self {
            Cmd::Su { profile, global } => {
                git::configure_user(&profile, global)?;
            }
            Cmd::WhoAmI { global } => {
                let (username, email) = git::get_username_and_email(global)?;
                let profile = profile::cache::get(&username, &email)
                    .ok_or(Error::NoProfileSet)?;
                println!("{profile}");
            }
            Cmd::Profile { command } => {
                command.present()?;
            }
        }
        Ok(())
    }
}

impl Presentation for ProfileCmd {
    fn present(self) -> Result<()> {
        match self {
            ProfileCmd::List { cached } => {
                let list = if cached { profile::cache::get_all() } else { profile::list()? };
                list
                    .iter()
                    .for_each(|profile_name| {
                        println!("{profile_name}")
                    });
            }
            ProfileCmd::Show { name } => {
                println!("{}", Profile::load(&name)?);
            }
            ProfileCmd::Add { name, username, email, force, key_type } => {
                let profile = Profile::new(&name, &username, &email)?;
                println!("Writing profile...");
                profile.save(false).map_err(|err| {
                    let err = Box::new(err);
                    Error::WithTip { err, tip: "re-run with --force to overwrite" }
                })?;
                ssh::try_regenerate_pair(&name, &email, force).map_err(|err| {
                    let err = Box::new(err);
                    Error::WithTip { err, tip: "re-run with --force to re-generate" }
                })?;
                generate_ssh_keys(&name, &email, &key_type)?;
            }
            ProfileCmd::Remove { profiles } => {
                for name in &profiles {
                    profile::remove(name)?;
                }
            }
            ProfileCmd::Edit { name, username, email, regenerate, key_type } => {
                profile::edit(&name, username, email)?;
                if regenerate {
                    let profile = Profile::load(&name)?;
                    generate_ssh_keys(&profile.name, &profile.email, &key_type)?;
                }
            }
        }
        Ok(())
    }
}

fn generate_ssh_keys(profile_name: &str, email: &str, key_type: &KeyType) -> Result<()> {
    let (private, public) = ssh::key::pair(email, key_type)?;
    println!("Generating ssh-{key_type} key pair...");
    ssh::key::write_private(profile_name, &private)?;
    ssh::key::write_public(profile_name, &public)?;
    println!("Keys written");
    let fingerprint = private.fingerprint(HashAlg::Sha256);
    let random_art = fingerprint.to_randomart(&key_type.random_art_header());
    println!("Key fingerprint is: {fingerprint}");
    println!("The key's randomart image is:\n{random_art}");

    Ok(())
}
