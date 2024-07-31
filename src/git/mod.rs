use std::env;
use std::path::Path;

use git2::Config;

use crate::{home, profile};
use crate::profile::profile::Profile;

type Result<T> = std::result::Result<T, error::Error>;
mod error;

pub fn configure_user(profile: &Profile, global: bool) -> Result<()> {
    let is_inside_repo = is_inside_repo()?;
    if !is_inside_repo && !global {
        println!("No git repository detected, setting profile in global config");
    };
    let global = global || !is_inside_repo;
    let mut config = config(global)?;
    // Can safely unwrap those because they throw only for invalid git config key
    config.set_str("user.name", &profile.user_name).unwrap();
    config.set_str("user.email", &profile.user_email).unwrap();
    config.set_str("core.sshCommand", &ssh_command(&profile.name)).unwrap();

    Ok(())
}

pub fn whoami(global: bool) -> Option<String> {
    let is_inside_repo = is_inside_repo().ok()?;
    let global = global || !is_inside_repo;
    let config = config(global).ok()?;
    let username = config.get_str("user.name").ok()?;
    let email = config.get_str("user.email").ok()?;

    profile::cache::get(username, email)
}

fn is_inside_repo() -> Result<bool> {
    let current_dir = env::current_dir()?;
    let path_str = format!("{}/.git", current_dir.to_str().unwrap());
    let path = Path::new(&path_str);

    Ok(path.exists() && path.is_dir())
}

fn config(global: bool) -> Result<Config> {
    let config = if global {
        Config::open_default()?
    } else {
        let current_dir = env::current_dir()?;
        Config::open(&current_dir)?
    };

    Ok(config)
}

fn ssh_command(profile_name: &str) -> String {
    let home = home();
    format!("ssh -i {home}/.ssh/id_{profile_name} -F /dev/null")
}
