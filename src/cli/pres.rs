use ssh_key::HashAlg;

use crate::{git, profile, ssh};
use crate::cli::{Cmd, ProfileCmd};
use crate::profile::profile::Profile;
use crate::ssh::key::r#type::{KeyType, RandomArtHeader};
use crate::util::{SafeUnwrap, UnwrapWithTip};

pub(crate) trait Execute {
    fn execute(self);
}

impl Execute for Cmd {
    fn execute(self) {
        match self {
            Cmd::Su { profile, global } => {
                git::configure_user(&profile, global).safe_unwrap();
            }
            Cmd::WhoAmI { global } => {
                match git::get_username_and_email(global) {
                    Ok((username, email)) => {
                        match profile::cache::get(&username, &email) {
                            Some(active_profile) => println!("{active_profile}"),
                            None => println!("No profile set"),
                        }
                    }
                    Err(e) => println!("{e}")
                }
            }
            Cmd::Profile { command } => {
                command.execute()
            }
        }
    }
}

impl Execute for ProfileCmd {
    fn execute(self) {
        match self {
            ProfileCmd::List => {
                profile::list()
                    .iter()
                    .for_each(|profile_name| {
                        println!("{profile_name}")
                    })
            }
            ProfileCmd::Show { name } => {
                match Profile::load(&name) {
                    Ok(profile) => println!("{profile}"),
                    Err(err) => println!("{err}")
                }
            }
            ProfileCmd::Add { name, username, email, force, key_type } => {
                match Profile::new(&name, &username, &email) {
                    Ok(profile) => {
                        let name = profile.name.clone();
                        let email = profile.email.clone();
                        println!("Writing profile...");
                        profile.save(false)
                            .unwrap_with_tip("re-run with --force to overwrite");
                        let result = ssh::try_regenerate_pair(&name, &email, force);
                        let is_err = result.is_err();
                        result
                            .unwrap_with_tip("re-run with --force to re-generate");
                        if is_err { return; }
                        generate_ssh_keys(&name, &email, &key_type);
                    }
                    Err(err) => println!("{err}")
                }
            }
            ProfileCmd::Remove { profiles } => {
                for p in &profiles {
                    profile::remove(p).safe_unwrap()
                }
            }
            ProfileCmd::Edit { name, username, email, regenerate, key_type } => {
                profile::edit(&name, username, email).safe_unwrap();
                if regenerate {
                    match Profile::load(&name) {
                        Err(err) => println!("{err}"),
                        Ok(profile) => {
                            generate_ssh_keys(&profile.name, &profile.email, &key_type);
                        }
                    }
                }
            }
        }
    }
}

fn generate_ssh_keys(profile_name: &str, email: &str, key_type: &KeyType) {
    match ssh::key::pair(email, key_type) {
        Err(err) => println!("{err}"),
        Ok((private, public)) => {
            println!("Generating ssh-{key_type} key pair...");
            ssh::key::write_private(profile_name, &private).safe_unwrap();
            ssh::key::write_public(profile_name, &public).safe_unwrap();
            println!("Keys written");
            let fingerprint = private.fingerprint(HashAlg::Sha256);
            let random_art = fingerprint.to_randomart(&key_type.header());
            println!("Key fingerprint is: {fingerprint}");
            println!("The key's randomart image is:\n{random_art}");
        }
    }
}
