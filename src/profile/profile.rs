use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::home;
use crate::profile::{cache, Result};
use crate::profile::error::Error;

const PROFILES_DIR: &str = ".config/g-profiles";

pub fn profiles_dir() -> String {
    let home = home();
    format!("{home}/{PROFILES_DIR}")
}

pub fn profile_path(profile_name: &str) -> String {
    let home = home();
    let profiles_dir = format!("{home}/{PROFILES_DIR}");
    format!("{profiles_dir}/{profile_name}")
}

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
    pub fn new(name: &str, username: &str, email: &str) -> Result<Self> {
        if name.contains('.') {
            Err(Error::InvalidName)?
        }
        cache::get(&username, &email)
            .map_or_else(
                || {
                    let (name, username, email) = (
                        name.to_string(),
                        username.to_string(),
                        email.to_string()
                    );
                    let profile = Self { name, username, email };
                    cache::insert(&profile)?;
                    Ok(profile)
                },
                |existing| {
                    let (username, email) = (username.to_string(), email.to_string());
                    Err(Error::CombinationExists { username, email, existing })
                },
            )
    }

    // TODO maybe implement Read trait?
    pub fn read(profile_name: &str) -> Result<Self> {
        let path = profile_path(profile_name);
        let bytes = fs::read(path)?;
        let partial = bincode::deserialize(&bytes[..])?;

        Ok((profile_name, partial).into())
    }

    // TODO maybe implement Write trait?
    pub fn write(self) -> Result<()> {
        let (profile_name, partial) = self.into();
        let path = profile_path(&profile_name);
        if Path::new(&path).exists() {
            Err(Error::ProfileExists(profile_name))?
        }
        let bytes = bincode::serialize(&partial)?;
        fs::write(&path, &bytes[..])?;

        Ok(())
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let user_name = &self.username;
        let user_email = &self.email;
        let home = home();

        write!(f, r#"
Profile '{name}'
username:       {user_name}
email:          {user_email}
ssh key:        {home}/.ssh/id_{name}
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
