use std::env;
use std::path::Path;

use git2::{Config, Repository};

use crate::home;
use crate::profile::active::ActiveProfile;
use crate::profile::profile::Profile;

pub fn configure_user(profile: &Profile, global: bool) {
    let is_inside_repo = is_inside_repo();
    if !is_inside_repo && !global {
        println!("No git repository detected, setting profile in global config");
    };
    let global = global || !is_inside_repo;
    if let Some(mut config) = if global { global_config() } else { local_config() } {
        // Can safely unwrap those because they throw only for invalid git config key
        config.set_str("user.name", &profile.user_name).unwrap();
        config.set_str("user.email", &profile.user_email).unwrap();
        config.set_str("core.sshCommand", &ssh_command(&profile.name)).unwrap();
        let active_profile = ActiveProfile::new(
            &profile.name,
            &profile.user_name,
            &profile.user_email,
            env::current_dir().unwrap().to_str().unwrap(),
        );
        if global { active_profile.write_global() } else { active_profile.write_local() }.unwrap();
    } else {
        println!("Can't load git config");
    }
}

pub fn whoami(global: bool) -> Option<String> {
    if global {
        let profile = ActiveProfile::read_global()?;
        Some(profile.name)
    } else {
        let config = local_config()?.snapshot().ok()?;
        let username = config.get_str("user.name").ok()?;
        let email = config.get_str("user.email").ok()?;
        let profile = ActiveProfile::read_local(username, email)?;
        Some(profile.name)
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
