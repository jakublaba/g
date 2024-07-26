use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use git2::Config;
use serde::{Deserialize, Serialize};

use crate::home;

const PROFILES_DIR: &str = ".config/g-profiles";

pub fn profiles_dir() -> String {
    let home = home();
    format!("{home}/{PROFILES_DIR}")
}

pub fn profile_path(profile_name: &str) -> String {
    let home = home();
    let profiles_dir = format!("{home}/{PROFILES_DIR}");
    format!("{profiles_dir}/{profile_name}.json")
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub user_name: String,
    pub user_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartialProfile {
    #[serde(rename = "name")]
    user_name: String,
    #[serde(rename = "email")]
    user_email: String,
}

impl TryFrom<Config> for PartialProfile {
    type Error = git2::Error;

    fn try_from(config: Config) -> std::result::Result<Self, Self::Error> {
        let user_name = config.get_string("user.name")?;
        let user_email = config.get_string("user.email")?;

        Ok(Self { user_name, user_email })
    }
}

impl Profile {
    pub fn new(name: String, user_name: String, user_email: String) -> Self {
        Self { name, user_name, user_email }
    }

    pub fn read_json(profile_name: &str) -> Result<Self> {
        let path = profile_path(profile_name);
        let json = fs::read(path)?;
        let partial = serde_json::from_slice(json.as_slice())?;

        Ok((profile_name, partial).into())
    }

    pub fn get_active(global: bool) -> Option<String> {
        let active_path = Self::active_path(global);
        fs::read_to_string(Path::new(&active_path)).ok()
    }

    pub fn set_active(profile_name: &str, global: bool) -> Result<()> {
        let active_path = Self::active_path(global);
        let mut file = File::create(Path::new(&active_path))?;
        file.write(profile_name.as_bytes())?;

        Ok(())
    }

    pub fn write_json(self) -> Result<()> {
        let (profile_name, partial) = self.into();
        let path = profile_path(&profile_name);
        let json = serde_json::to_vec(&partial)?;
        fs::write(&path, json)?;

        Ok(())
    }

    fn active_path(global: bool) -> String {
        if global {
            format!("{}/active_global", profiles_dir())
        } else {
            format!("{}/active", profiles_dir())
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let user_name = &self.user_name;
        let user_email = &self.user_email;
        let home = home();

        write!(f, r#"
profile name:   {name}
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
            user_name: partial.user_name,
            user_email: partial.user_email,
        }
    }
}

impl Into<(String, PartialProfile)> for Profile {
    fn into(self) -> (String, PartialProfile) {
        let partial = PartialProfile {
            user_name: self.user_name,
            user_email: self.user_email,
        };

        (self.name, partial)
    }
}
