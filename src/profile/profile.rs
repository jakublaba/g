use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

// TODO make this into `profile` module and define custom Error + Result
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
    pub fn read_json(profile_name: &str) -> Result<Self, serde_json::Error> {
        let fname = profile_path(profile_name);
        let file = File::open(&fname).expect(&format!("Error opening file: {fname}"));
        let reader = BufReader::new(file);

        let partial: PartialProfile = serde_json::from_reader(reader)?;

        Ok((profile_name, partial).into())
    }

    pub fn write_json(self) -> Result<(), serde_json::Error> {
        let (profile_name, partial) = self.into();
        let fname = profile_path(&profile_name);
        let file = File::open(&fname).expect(&format!("Error writing to file: {fname}"));
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &partial)
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
