use std::fs;

use crate::{home, ssh};
use crate::profile::error::Error;
use crate::profile::model::{Profile, profile_path};

pub mod model;
pub mod cache;
pub mod error;

type Result<T> = std::result::Result<T, error::Error>;

const PROFILES_DIR: &str = ".config/g-profiles";

/// Loads list of profile names from [`PROFILES_DIR`].
///
/// ```
/// let names = list().expect("Can't load list of profile names");
/// for profile_name in names {
///     println!("{profile_name}");
/// }
/// ```
pub fn list() -> Result<Vec<String>> {
    let path = profiles_dir();
    let paths = fs::read_dir(&path)
        .map_err(|e| Error::Io(e, path.into()))?;
    let names = paths
        .map(|dir_entry| dir_entry.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.components()
            .last().unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string()
        )
        .filter(|name| !name.starts_with('.'))
        .collect();

    Ok(names)
}

/// Removes profile with chosen `name` from [`PROFILES_DIR`] and profile cache.
///
/// ```
/// let profile = "example";
/// remove(profile).expect(&format!("Can't remove {profile}"));
/// ```
pub fn remove(name: &str) -> Result<Vec<String>> {
    let mut info = Vec::<String>::new();
    [profile_path(name), ssh::key::path_private(name), ssh::key::path_public(name)]
        .iter()
        .for_each(|p| match fs::remove_file(p) {
            Ok(_) => info.push(format!("removed: {p}")),
            Err(_) => info.push(format!("skipped: {p}"))
        });
    cache::remove(name)?;

    Ok(info)
}

/// Changes `username` and/or `email` for profile with specified `name`
///
/// ```
/// let profile = "example";
/// edit(profile, None, Some("new@email.com".to_string()))).expect(&format!("Can't edit {profile}"));
/// ```
pub fn edit(name: &str, username: Option<String>, email: Option<String>) -> Result<()> {
    if username.is_none() && email.is_none() {
        return Ok(());
    }
    let mut profile = Profile::load(name)?;
    if let Some(usr_name) = username {
        profile.username = usr_name.to_string();
    };
    if let Some(usr_email) = email {
        profile.email = usr_email.to_string();
    };

    profile.save(true)
}

fn profiles_dir() -> String {
    format!("{}/{PROFILES_DIR}", home())
}

#[cfg(test)]
mod test {
    use std::{env, fs};

    use rstest::fixture;
    use rstest::rstest;
    use spectral::assert_that;
    use spectral::iter::ContainingIntoIterAssertions;
    use spectral::prelude::{OptionAssertions, PathAssertions, VecAssertions};
    use tempfile::{TempDir, tempdir};

    use super::*;

    #[fixture]
    fn fake_home() -> TempDir {
        let fake_home = tempdir().unwrap();
        fs::create_dir_all(fake_home.path().join(".config/g-profiles")).unwrap();
        fs::create_dir_all(fake_home.path().join(".ssh")).unwrap();
        env::set_var("HOME", fake_home.path().to_string_lossy().to_string());

        fake_home
    }

    mod list {
        use super::*;

        #[rstest]
        fn empty(fake_home: TempDir) {
            // create a hidden file to test it doesn't get picked up
            fs::write(&fake_home.path().join(PROFILES_DIR).join(".hidden"), "").unwrap();

            assert_that!(list().unwrap()).is_empty();
        }

        #[rstest]
        fn non_empty(fake_home: TempDir) {
            let profiles = vec!["normal".to_string(), "with.dots".to_string()];
            for p in &profiles {
                fs::write(fake_home.path().join(PROFILES_DIR).join(p), "").unwrap();
            }

            assert_that!(list().unwrap())
                .contains_all_of(&profiles.iter());
        }
    }

    mod remove {
        use super::*;

        #[rstest]
        #[case::only_profile(
            vec ! [".config/g-profiles/test"],
            vec ! [".ssh/id_test", ".ssh/id_test.pub"]
        )]
        #[case::one_key(
            vec ! [".config/g-profiles/test", ".ssh/id_test"],
            vec ! [".ssh/id_test.pub"]
        )]
        #[case::only_keys(
            vec ! [".ssh/id_test", ".ssh/id_test.pub"],
            vec ! [".config/g-profiles/test"]
        )]
        fn remove_stuff(#[case] removed: Vec<&str>, #[case] skipped: Vec<&str>, fake_home: TempDir) {
            let p = Profile::new("test", "", "").unwrap();
            cache::insert(&p).unwrap();
            removed.iter()
                .map(|p| fake_home.path().join(p))
                .for_each(|p| fs::write(p, "").unwrap());

            let info = remove("test").unwrap();

            assert_that!(cache::get(&p.username, &p.email)).is_none();
            removed.into_iter()
                .map(|p| fake_home.path().join(p))
                .for_each(|p| {
                    assert_that!(p).does_not_exist();
                    assert_that!(info).contains(format!("removed: {}", p.display()));
                });
            skipped.into_iter()
                .map(|p| fake_home.path().join(p))
                .for_each(|p| {
                    assert_that!(p).does_not_exist();
                    assert_that!(info).contains(format!("skipped: {}", p.display()));
                });
        }
    }

    mod edit {
        use super::*;

        #[fixture]
        fn profile() -> Profile {
            Profile::new("test", "Test Username", "test@email.com").unwrap()
        }

        #[rstest]
        fn nothing(profile: Profile, fake_home: TempDir) {
            let expected = profile.clone();
            profile.save(false).unwrap();

            edit(&expected.name, None, None).unwrap();

            assert_that!(Profile::load(&expected.name).unwrap()).is_equal_to(expected);
            let _ = fake_home;
        }

        #[rstest]
        #[case::only_username(Some("username".to_string()), None)]
        #[case::only_email(None, Some("em@i.l".to_string()))]
        #[case::both(Some("username".to_string()), Some("em@i.l".to_string()))]
        fn something(
            profile: Profile, _fake_home: TempDir,
            #[case] username: Option<String>, #[case] email: Option<String>,
        ) {
            let expected = profile.clone();
            profile.save(false).unwrap();

            edit(&expected.name, username.clone(), email.clone()).unwrap();

            let profile = Profile::load(&expected.name).unwrap();
            if let Some(username) = username {
                assert_that!(profile.username).is_equal_to(username);
            }
            if let Some(email) = email {
                assert_that!(profile.email).is_equal_to(email);
            }
        }
    }
}
