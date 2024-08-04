use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::HOME;
use crate::profile::{cache, Result};
use crate::profile::error::Error;
use crate::profile::PROFILES_DIR;

pub(super) fn profile_path(profile_name: &str) -> String {
    format!("{PROFILES_DIR}/{profile_name}")
}

/// Represents a g profile
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct PartialProfile {
    #[serde(rename = "name")]
    username: String,
    email: String,
}

impl Profile {
    /// Validates `name` and constructs [`Profile`]
    ///
    /// [`Error::InvalidName`] is returned if `name` starts with the `.` character
    ///
    /// ```
    /// let profile = Profile::new("example", "An example profile", "profile@example.com").unwrap();
    /// ```
    pub fn new(name: &str, username: &str, email: &str) -> Result<Self> {
        if name.starts_with('.') {
            Err(Error::InvalidName)?
        }

        Ok(Self {
            name: name.to_string(),
            username: username.to_string(),
            email: email.to_string(),
        })
    }

    /// Reads and deserializes [`Profile`] from [`PROFILES_DIR`]
    ///
    /// Doesn't return any specific errors, just forwards the ones related to io and deserialization
    ///
    /// ```
    /// let name = "example";
    /// let profile = Profile::load(name).expect("Can't load profile '{name}'");
    /// ```
    pub fn load(profile_name: &str) -> Result<Self> {
        let path = profile_path(profile_name);
        let bytes = fs::read(path)?;
        let partial = bincode::deserialize(&bytes[..])?;

        Ok((profile_name, partial).into())
    }

    /// Serializes and saves [`Profile`] to [`PROFILES_DIR`] and caches its name.
    ///
    /// # Errors
    /// - [`Error::ProfileExists`] if profile with the same name is already saved to [`PROFILES_DIR`]
    /// - [`Error::CombinationExists`] if username/email combination is already in use by another profile
    /// (either username or email can overlap, but not both at the same time)
    pub fn save(self, overwrite: bool) -> Result<()> {
        let (profile_name, partial) = self.clone().into();
        let path = profile_path(&profile_name);
        if Path::new(&path).exists() && !overwrite {
            Err(Error::ProfileExists(profile_name))?
        }
        let bytes = bincode::serialize(&partial)?;
        fs::write(&path, &bytes[..])?;

        cache::get(&self.username, &self.username)
            .map_or_else(
                || cache::insert(&self),
                |existing| {
                    Err(Error::CombinationExists {
                        username: (&self.username).to_string(),
                        email: (&self.email).to_string(),
                        existing,
                    })
                },
            )?;

        Ok(())
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let user_name = &self.username;
        let user_email = &self.email;

        write!(f, r#"
Profile '{name}'
username:       {user_name}
email:          {user_email}
ssh key:        {HOME}/.ssh/id_{name}
        "#)
    }
}

impl From<(&str, PartialProfile)> for Profile {
    fn from(args: (&str, PartialProfile)) -> Self {
        let (name, partial) = args;
        Self {
            name: String::from(name),
            username: partial.username,
            email: partial.email,
        }
    }
}

impl Into<(String, PartialProfile)> for Profile {
    fn into(self) -> (String, PartialProfile) {
        let partial = PartialProfile {
            username: self.username,
            email: self.email,
        };

        (self.name, partial)
    }
}
