use std::path::Path;

use crate::ssh::error::Error;

pub mod error;
pub mod key;

type Result<T> = std::result::Result<T, error::Error>;

pub(crate) fn try_regenerate_pair(profile_name: &str, email: &str, force: bool) -> Result<()> {
    if !force && Path::new(&key::path_private(profile_name)).exists() {
        if Path::new(&key::path_public(profile_name)).exists() {
            Err(Error::KeyPairExists)?
        }
        println!("Private key found, attempting to re-generate public key");
        let pub_key = key::public_from_private(profile_name, email)?;
        return key::write_public(profile_name, &pub_key);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::thread_rng;
    use rstest::{fixture, rstest};
    use spectral::assert_that;
    use spectral::prelude::PathAssertions;
    use ssh_key::{Algorithm, LineEnding, PrivateKey};
    use std::path::PathBuf;
    use std::{env, fs};
    use tempfile::{tempdir, TempDir};

    mod try_regenerate_pair {
        use super::*;

        const PROFILE_NAME: &str = "test";
        const EMAIL: &str = "test@email.com";

        #[fixture]
        fn fake_home() -> TempDir {
            let fake_home = tempdir().unwrap();
            env::set_var("HOME", fake_home.path().to_string_lossy().to_string());
            fs::create_dir_all(fake_home.path().join(".ssh")).unwrap();

            fake_home
        }

        fn priv_path<P: AsRef<Path>>(home: P) -> PathBuf {
            home.as_ref().join(&format!(".ssh/id_{PROFILE_NAME}"))
        }

        fn pub_path<P: AsRef<Path>>(home: P) -> PathBuf {
            home.as_ref().join(&format!(".ssh/id_{PROFILE_NAME}.pub"))
        }

        fn file_hash<P: AsRef<Path>>(path: P) -> String {
            let bytes = fs::read(path).unwrap();
            sha256::digest(&bytes[..])
        }

        #[rstest]
        #[case::none(vec![])]
        #[case::both(vec![format!(".ssh/id_{PROFILE_NAME}"), format!(".ssh/id_{PROFILE_NAME}.pub")])]
        #[case::private(vec![format!(".ssh/id_{PROFILE_NAME}")])]
        #[case::public(vec![format!(".ssh/id_{PROFILE_NAME}.pub")])]
        fn skip_on_force(fake_home: TempDir, #[case] existing: Vec<String>) {
            let home_path = fake_home.path();
            existing
                .iter()
                .map(|s| home_path.join(s))
                .for_each(|p| fs::write(p, "").unwrap());

            try_regenerate_pair(PROFILE_NAME, EMAIL, true).unwrap();

            existing
                .iter()
                .map(|s| home_path.join(s))
                .for_each(|p| assert_that!(p).exists());
        }

        #[rstest]
        #[case::force(true)]
        #[case::no_force(false)]
        fn none_exist(fake_home: TempDir, #[case] force: bool) {
            let home_path = fake_home.path();

            try_regenerate_pair(PROFILE_NAME, EMAIL, force).unwrap();

            assert_that!(priv_path(home_path)).does_not_exist();
            assert_that!(pub_path(home_path)).does_not_exist();
        }

        #[rstest]
        fn both_exist(fake_home: TempDir) {
            let home_path = fake_home.path();
            fs::write(priv_path(home_path), "").unwrap();
            fs::write(pub_path(home_path), "").unwrap();

            let err_msg = try_regenerate_pair(PROFILE_NAME, EMAIL, false)
                .unwrap_err()
                .to_string();

            assert_that!(err_msg).is_equal_to(Error::KeyPairExists.to_string());
        }

        #[rstest]
        fn private_exists(fake_home: TempDir) {
            let home_path = fake_home.path();
            let priv_key = PrivateKey::random(&mut thread_rng(), Algorithm::Ed25519).unwrap();
            priv_key
                .write_openssh_file(&priv_path(home_path), LineEnding::LF)
                .unwrap();
            let old_hash = file_hash(priv_path(home_path));

            try_regenerate_pair(PROFILE_NAME, EMAIL, false).unwrap();

            let new_hash = file_hash(priv_path(home_path));
            assert_that!(pub_path(home_path)).exists();
            assert_that!(new_hash).is_equal_to(old_hash);
        }
    }
}
