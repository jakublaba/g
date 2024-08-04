use std::env;
use std::path::Path;

use git2::Config;

use crate::git::error::Error;
use crate::HOME;
use crate::profile::profile::Profile;

type Result<T> = std::result::Result<T, error::Error>;
pub mod error;

/// Configures `profile` for git: `user.name`, `user.email` and `core.sshCommand`.
/// Local git config is used if current working directory is a git repository and `global` is set to `false`.
/// Otherwise, global config is used.
///
/// ```
/// let profile = Profile::new("example", "Example profile", "user@example.com");
/// configure_user(&profile, false);
/// ```
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

/// Gets `user.name` and `user.email` from git config.
/// Local git config is used if current working directory is a git repository and `global` is set to `false`.
/// Otherwise, global config is used.
///
/// Will return [`Error::EmptyProperty`] if either `user.name` or `user.email` is not set.
/// Other errors are forwarded from [`git2`].
///
/// ```
/// let (username, email) = get_username_and_email(true).expect("Can't open ~/.gitconfig");
/// ```
pub fn get_username_and_email(global: bool) -> Result<(String, String)> {
    let global = global || !is_inside_repo();
    let config = config(global)?;
    let username = config.get_string("user.name")
        .map_err(|_| Error::EmptyProperty("user.name".to_string()))?;
    let email = config.get_string("user.email")
        .map_err(|_| Error::EmptyProperty("user.email".to_string()))?;

    Ok((username, email))
}

fn is_inside_repo() -> bool {
    let current_dir = env::current_dir().unwrap();
    let path_str = format!("{}/.git", current_dir.to_str().unwrap());
    let path = Path::new(&path_str);

    path.exists() && path.is_dir()
}

fn config(global: bool) -> Result<Config> {
    let config = if global {
        Config::open_default()?
    } else {
        let config_path = format!("{}/.git/config", env::current_dir()?.display());
        Config::open(Path::new(&config_path))?
    };

    Ok(config)
}

fn ssh_command(profile_name: &str) -> String {
    format!("ssh -i {HOME}/.ssh/id_{profile_name} -F /dev/null")
}
