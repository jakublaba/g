use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use crate::profile::error::Error;
use crate::profile::Result;

const HOME: &str = env!("HOME");
const PROFILES_DIR: &str = ".config/git-multiaccount-profiles";

fn profile_path(profile: &str) -> String {
    let profiles_dir = format!("{HOME}/{PROFILES_DIR}");
    format!("{profiles_dir}/{profile}.json")
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub user_name: String,
    pub user_email: String,
}

#[derive(Serialize, Deserialize)]
struct PartialProfile {
    #[serde(rename = "name")]
    user_name: String,
    #[serde(rename = "email")]
    user_email: String,
}

// TODO - handle overriding existing profiles
impl Profile {
    pub fn read_json(profile_name: &str) -> Result<Self> {
        let path = profile_path(profile_name);
        let file = File::open(&path)
            .map_err(|cause| Error::Io { path, cause })?;
        let reader = BufReader::new(file);
        let partial = serde_json::from_reader(reader)
            .map_err(Error::Serde)?;

        Ok((profile_name, partial).into())
    }

    pub fn write_json(self) -> Result<()> {
        let (profile_name, partial) = self.into();
        let path = profile_path(&profile_name);
        let file = File::open(&path)
            .map_err(|cause| Error::Io { path, cause })?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &partial)
            .map_err(Error::Serde)
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
