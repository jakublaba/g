use std::{env, io};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use git2::Config;

use crate::git::error::Error;
use crate::home;
use crate::profile::model::Profile;

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

const TIMEOUT: Duration = Duration::from_millis(1);
fn config(global: bool) -> Result<Config> {
    let config_path = if global {
        format!("{}/.gitconfig", home())
    } else {
        format!("{}/.git/config", env::current_dir()?.display())
    };
    let lock_path = PathBuf::from(format!("{config_path}.lock"));
    let start = Instant::now();
    while lock_path.exists() {
        if start.elapsed() >= TIMEOUT {
            Err(io::Error::new(ErrorKind::TimedOut, &*format!("Timed out waiting for {}", lock_path.display())))?;
        }
    }
    let config = Config::open(Path::new(&config_path))?;

    Ok(config)
}

fn ssh_command(profile_name: &str) -> String {
    format!("ssh -i {}/.ssh/id_{profile_name} -F /dev/null", home())
}

#[cfg(test)]
mod test {
    use std::{env, fs};

    use git2::{Config, Repository};
    use rstest::{fixture, rstest};
    use spectral::assert_that;
    use tempfile::{tempdir, TempDir};

    use super::*;

    mod configure_user {
        use super::*;

        #[fixture]
        #[once]
        fn profile() -> Profile {
            Profile::new("test", "Test Profile", "em@i.l").unwrap()
        }

        #[fixture]
        fn fake_home() -> TempDir {
            let fake_home = tempdir().unwrap();
            env::set_var("HOME", fake_home.path().to_string_lossy().to_string());

            fake_home
        }

        #[fixture]
        fn fake_repo() -> TempDir {
            let fake_repo = tempdir().unwrap();
            Repository::init(fake_repo.path()).unwrap();

            fake_repo
        }

        #[rstest]
        fn set_local_config_in_repo(profile: &Profile, fake_repo: TempDir) {
            env::set_current_dir(fake_repo.path()).unwrap();

            configure_user(profile, false).unwrap();
            let config = Config::open(&fake_repo.path().join(".git/config")).unwrap()
                .snapshot().unwrap();

            assert_that!(config.get_str("user.name").unwrap())
                .is_equal_to(&*profile.username);
            assert_that!(config.get_str("user.email").unwrap())
                .is_equal_to(&*profile.email);
            assert_that!(config.get_str("core.sshCommand").unwrap())
                .is_equal_to(&*ssh_command(&profile.name));
        }

        #[rstest]
        fn set_global_config_in_repo(profile: &Profile, fake_repo: TempDir, fake_home: TempDir) {
            env::set_current_dir(fake_repo.path()).unwrap();

            env::set_var("HOME", fake_home.path().to_string_lossy().to_string());
            fs::write(fake_home.path().join(".gitconfig"), "").unwrap();

            configure_user(profile, true).unwrap();
            let config = Config::open(&fake_home.path().join(".gitconfig")).unwrap()
                .snapshot().unwrap();

            assert_that!(config.get_str("user.name").unwrap())
                .is_equal_to(&*profile.username);
            assert_that!(config.get_str("user.email").unwrap())
                .is_equal_to(&*profile.email);
            assert_that!(config.get_str("core.sshCommand").unwrap())
                .is_equal_to(&*ssh_command(&profile.name));
        }

        #[rstest]
        fn set_global_config_outside_repo(profile: &Profile, fake_repo: TempDir, fake_home: TempDir) {
            env::set_current_dir(fake_repo.path()).unwrap();

            fs::write(fake_home.path().join(".gitconfig"), "").unwrap();

            configure_user(profile, true).unwrap();
            let config = Config::open(&fake_home.path().join(".gitconfig")).unwrap()
                .snapshot().unwrap();

            assert_that!(config.get_str("user.name").unwrap())
                .is_equal_to(&*profile.username);
            assert_that!(config.get_str("user.email").unwrap())
                .is_equal_to(&*profile.email);
            assert_that!(config.get_str("core.sshCommand").unwrap())
                .is_equal_to(&*ssh_command(&profile.name));
        }
    }
}
