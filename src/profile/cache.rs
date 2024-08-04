use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use const_format::formatcp;

use crate::profile::profile::Profile;
use crate::profile::PROFILES_DIR;
use crate::profile::Result;

const CACHE_PATH: &str = formatcp!("{PROFILES_DIR}/.cache");

pub(crate) fn get(username: &str, email: &str) -> Option<String> {
    let mut cache = load_cache().ok()?;
    let key = key(username, email);

    cache.remove(&key)
}

pub(crate) fn get_all() -> Vec<String> {
    match load_cache() {
        Ok(cache) => cache
            .into_values()
            .collect(),
        Err(_) => vec![]
    }
}

pub(super) fn insert(profile: &Profile) -> Result<()> {
    let mut cache = load_cache()?;
    let key = key(&profile.username, &profile.email);
    cache.insert(key, (&profile.name).to_string());
    save_cache(cache)?;

    Ok(())
}

pub(super) fn remove(profile_name: &str) -> Result<()> {
    let cache = load_cache()?
        .into_iter()
        .filter(|(_, v)| v != profile_name)
        .collect::<HashMap<u64, String>>();
    save_cache(cache)?;

    Ok(())
}

fn key(username: &str, email: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);
    email.hash(&mut hasher);

    hasher.finish()
}

fn load_cache() -> Result<HashMap<u64, String>> {
    if !Path::new(CACHE_PATH).exists() {
        return Ok(HashMap::new());
    }
    let bytes = fs::read(CACHE_PATH)?;
    let cache = bincode::deserialize(&bytes[..])?;

    Ok(cache)
}

fn save_cache(cache: HashMap<u64, String>) -> Result<()> {
    let bytes = bincode::serialize(&cache)?;
    fs::write(CACHE_PATH, &bytes[..])?;

    Ok(())
}
