use std::env;
use std::path::Path;

use git2::Config;

use crate::git::error::Error;
use crate::home;
use crate::profile::profile::Profile;

type Result<T> = std::result::Result<T, error::Error>;
pub mod error;

pub fn configure_user(profile: &Profile, global: bool) -> Result<()> {
    let is_inside_repo = is_inside_repo();
    if !is_inside_repo && !global {
        println!("No git repository detected, setting profile in global config");
    };
    let global = global || !is_inside_repo;
    let mut config = config(global)?;
    // Can safely unwrap those because they throw only for invalid git config key
    config.set_str("user.name", &profile.username).unwrap();
    config.set_str("user.email", &profile.email).unwrap();
    config.set_str("core.sshCommand", &ssh_command(&profile.name)).unwrap();

    Ok(())
}

pub fn get_username_and_email(global: bool) -> Result<(String, String)> {
    let global = global || !is_inside_repo();
    let config = config(global)?;
    let username = config.get_string("user.name")
        .map_err(|_| Error::EmptyProperty("user.name".to_string()))?;
    let email = config.get_string("user.email")
        .map_err(|_| Error::EmptyProperty("user.email".to_string()))?;

    Ok((username, email))
}

pub(crate) fn is_inside_repo() -> bool {
    let current_dir = env::current_dir().unwrap();
    let path_str = format!("{}/.git", current_dir.to_str().unwrap());
    let path = Path::new(&path_str);

    path.exists() && path.is_dir()
}

pub(crate) fn config(global: bool) -> Result<Config> {
    let config = if global {
        Config::open_default()?
    } else {
        let current_dir = env::current_dir()?;
        Config::open(&current_dir)?
    };

    Ok(config)
}

pub(crate) fn ssh_command(profile_name: &str) -> String {
    let home = home();
    format!("ssh -i {home}/.ssh/id_{profile_name} -F /dev/null")
}
