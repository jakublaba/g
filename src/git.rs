use std::env;
use std::path::Path;

use anyhow::Result;
use git2::{Config, Repository};

use crate::home;
use crate::profile::profile::{PartialProfile, Profile};

pub fn configure_user(profile: &Profile, global: bool) -> Result<()> {
    let is_inside_repo = is_inside_repo();
    if !is_inside_repo && !global {
        println!("No git repository detected, setting profile in global config");
    };
    if global || !is_inside_repo { set_config(profile, true)? };
    if is_inside_repo { set_config(profile, false)? };

    Ok(())
}

pub fn who_am_i(global: bool) -> Result<PartialProfile> {
    let is_inside_repo = is_inside_repo();
    let config = if global || !is_inside_repo { open_global_config() } else { open_local_config() }?;
    let profile = PartialProfile::try_from(config)?;

    Ok(profile)
}

fn set_config(profile: &Profile, global: bool) -> Result<()> {
    let mut config = if global { open_global_config() } else { open_local_config() }?;
    config.set_str("user.name", &profile.user_name)?;
    config.set_str("user.email", &profile.user_email)?;
    config.set_str("core.sshCommand", &ssh_command(&profile.name))?;

    Ok(())
}

fn is_inside_repo() -> bool {
    let current_dir = env::current_dir().unwrap();
    let path_str = format!("{}/.git", current_dir.to_str().unwrap());
    let path = Path::new(&path_str);

    path.exists() && path.is_dir()
}

fn open_global_config() -> Result<Config> {
    Ok(Config::open_default()?)
}

fn open_local_config() -> Result<Config> {
    let current_dir = env::current_dir()?;
    let config = Repository::open(current_dir)?
        .config()?;

    Ok(config)
}

fn ssh_command(profile_name: &str) -> String {
    let home = home();
    format!("ssh -i {home}/.ssh/id_{profile_name} -F /dev/null")
}
