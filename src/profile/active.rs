use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::home;

fn active_global_path() -> String {
    format!("{}/.config/g-profiles/active_global", home())
}

fn active_local_path() -> String {
    format!("{}/.config/g-profiles/active_local", home())
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ActiveProfile {
    pub(crate) name: String,
    user_name: String,
    user_email: String,
    repository: String,
}

impl ActiveProfile {
    pub fn new(name: &str, user_name: &str, user_email: &str, repository: &str) -> Self {
        Self {
            name: name.to_string(),
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
            repository: repository.to_string(),
        }
    }


    pub fn read_global() -> Option<Self> {
        let path = active_global_path();
        let json = fs::read(Path::new(&path)).ok()?;

        serde_json::from_slice(json.as_slice()).ok()
    }

    pub fn read_local(username: &str, email: &str) -> Option<Self> {
        let mut active_profiles = Self::read_local_list().ok()?;

        active_profiles.remove(&Self::key(username, email))
    }

    pub fn write_global(self) -> Result<()> {
        let json = serde_json::to_vec(&self)?;
        let path = active_global_path();
        fs::write(Path::new(&path), json.as_slice())?;

        Ok(())
    }

    pub fn write_local(self) -> Result<()> {
        let mut active_profiles = Self::read_local_list()?;
        active_profiles.insert(Self::key(&self.user_name, &self.user_email), self);
        let path = active_local_path();
        let json = serde_json::to_vec(&active_profiles)?;
        fs::write(Path::new(&path), json.as_slice())?;

        Ok(())
    }

    fn read_local_list() -> Result<HashMap<u64, Self>> {
        let p = active_local_path();
        let path = Path::new(&p);
        if !path.exists() {
            return Ok(HashMap::new());
        }
        let json = fs::read(path)?;
        let active_profiles = serde_json::from_slice(json.as_slice())?;

        Ok(active_profiles)
    }

    fn key(username: &str, email: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        email.hash(&mut hasher);

        hasher.finish()
    }
}
