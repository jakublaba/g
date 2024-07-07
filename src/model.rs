use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

const PROFILES_DIR: &str = "~/.config/git-multiaccount-profiles";

fn profile_path(profile: &str) -> String {
    let prof_dir = shellexpand::tilde(&PROFILES_DIR);
    format!("{prof_dir}/{profile}.json")
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
    pub fn from_json(profile_name: String) -> Result<Self, serde_json::Error> {
        let fname = profile_path(&profile_name);
        let file = File::open(&fname).expect(&format!("Error opening file: {fname}"));
        let reader = BufReader::new(file);

        let partial: PartialProfile = serde_json::from_reader(reader)?;

        Ok(Self::from_partial(profile_name, partial))
    }

    pub fn to_json(self) -> Result<(), serde_json::Error> {
        let (profile_name, partial) = self.to_partial();
        let fname = profile_path(&profile_name);
        let file = File::open(&fname).expect(&format!("Error writing to file: {fname}"));
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &partial)
    }

    fn from_partial(name: String, partial: PartialProfile) -> Self {
        Self {
            name,
            user_name: partial.user_name,
            user_email: partial.user_email,
        }
    }

    fn to_partial(self) -> (String, PartialProfile) {
        (
            self.name,
            PartialProfile {
                user_name: self.user_name,
                user_email: self.user_email,
            },
        )
    }
}
