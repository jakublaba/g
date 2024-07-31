use crate::{git, profile};
use crate::cli::{Cmd, ProfileCmd};
use crate::profile::profile::Profile;
use crate::util::SafeUnwrap;

pub trait Execute {
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
                profile::load_profile_list()
                    .iter()
                    .for_each(|profile_name| {
                        println!("{profile_name}")
                    })
            }
            ProfileCmd::Show { name } => {
                match Profile::read_json(&name) {
                    Ok(profile) => println!("{profile}"),
                    Err(err) => println!("{err}")
                }
            }
            ProfileCmd::Add { name, user_name, user_email, force, key_type } => {
                let profile = Profile::new(name, user_name, user_email);
                todo!()
            }
            ProfileCmd::Remove { .. } => {}
            ProfileCmd::Edit { .. } => {}
            ProfileCmd::Regenerate { .. } => {}
        }
    }
}
