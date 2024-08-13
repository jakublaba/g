use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use crate::profile::error::Error;
use crate::profile::model::Profile;
use crate::profile::{profiles_dir, Result};

pub(crate) fn get(username: &str, email: &str) -> Option<String> {
    let mut cache = load().ok()?;
    let key = key(username, email);

    cache.remove(&key)
}

pub(crate) fn get_all() -> Vec<String> {
    match load() {
        Ok(cache) => cache
            .into_values()
            .collect(),
        Err(_) => vec![]
    }
}

pub(super) fn insert(profile: &Profile) -> Result<()> {
    let mut cache = load()?;
    let key = key(&profile.username, &profile.email);
    cache.insert(key, profile.name.to_string());
    save(cache)?;

    Ok(())
}

pub(super) fn remove(profile_name: &str) -> Result<()> {
    let cache = load()?
        .into_iter()
        .filter(|(_, v)| v != profile_name)
        .collect::<HashMap<u64, String>>();
    save(cache)?;

    Ok(())
}

fn key(username: &str, email: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);
    email.hash(&mut hasher);

    hasher.finish()
}

fn load() -> Result<HashMap<u64, String>> {
    let cache_path = cache_path();
    if !Path::new(&cache_path).exists() {
        return Ok(HashMap::new());
    }
    let bytes = fs::read(&cache_path)
        .map_err(|e| Error::Io(e, cache_path.into()))?;
    let cache = bincode::deserialize(&bytes[..])?;

    Ok(cache)
}

fn save(cache: HashMap<u64, String>) -> Result<()> {
    let cache_path = cache_path();
    let bytes = bincode::serialize(&cache)?;
    fs::write(&cache_path, &bytes[..])
        .map_err(|e| Error::Io(e, cache_path.into()))?;

    Ok(())
}

fn cache_path() -> String {
    format!("{}/.cache", profiles_dir())
}
