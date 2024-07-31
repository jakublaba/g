use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use crate::home;
use crate::profile::profile::Profile;
use crate::profile::Result;

pub fn insert(profile: &Profile) -> Result<()> {
    let mut cache = load_cache()?;
    let key = key(&profile.user_name, &profile.user_email);
    cache.insert(key, (&profile.name).to_string());
    save_cache(cache)?;

    Ok(())
}

pub fn get(username: &str, email: &str) -> Option<String> {
    let mut cache = load_cache().ok()?;
    let key = key(username, email);

    cache.remove(&key)
}

pub fn remove(profile_name: &str) -> Result<()> {
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
    let cache_path = cache_path();
    let path = Path::new(&cache_path);
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let bytes = fs::read(path)?;
    let cache = bincode::deserialize(&bytes[..])?;

    Ok(cache)
}

fn save_cache(cache: HashMap<u64, String>) -> Result<()> {
    let bytes = bincode::serialize(&cache)?;
    fs::write(&cache_path(), &bytes[..])?;

    Ok(())
}

fn cache_path() -> String {
    format!("{}/.config/g-profiles/.cache", home())
}