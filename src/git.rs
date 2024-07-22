use std::env;
use std::path::Path;

use git2::{Config, Repository};

use crate::home;
use crate::profile::profile::{PartialProfile, Profile};

pub fn configure_user(profile: &Profile, global: bool) {
    let is_inside_repo = is_inside_repo();
    if !is_inside_repo && !global {
        println!("No git repository detected, setting profile in global config");
    };
    if global || !is_inside_repo { set_config(profile, true) };
    if is_inside_repo { set_config(profile, false) };
}

pub fn who_am_i(global: bool) -> Option<PartialProfile> {
    let is_inside_repo = is_inside_repo();
    let config = if global || !is_inside_repo { global_config() } else { local_config() }?;
    
    PartialProfile::try_from(config).ok()
}

fn set_config(profile: &Profile, global: bool) {
    let config_result = if global { global_config() } else { local_config() };
    if let Some(mut config) = config_result {
        config.set_str("user.name", &profile.user_name).unwrap();
        config.set_str("user.email", &profile.user_email).unwrap();
        config.set_str("core.sshCommand", &ssh_command(&profile.name)).unwrap();
    } else {
        println!("Can't load {} git config", if global { "global" } else { "local" });
    }
}

fn is_inside_repo() -> bool {
    let current_dir = env::current_dir().unwrap();
    let path_str = format!("{}/.git", current_dir.to_str().unwrap());
    let path = Path::new(&path_str);

    path.exists() && path.is_dir()
}

fn global_config() -> Option<Config> {
    Config::open_default().ok()
}

fn local_config() -> Option<Config> {
    let current_dir = env::current_dir().ok()?;
    let repository = Repository::open(current_dir).ok()?;

    repository.config().ok()
}

fn ssh_command(profile_name: &str) -> String {
    let home = home();
    format!("ssh -i {home}/.ssh/id_{profile_name} -F /dev/null")
}
