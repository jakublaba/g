use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use anyhow::Result;
use git2::Config;

use crate::home;

pub fn insert(git_config: &Config, profile_name: &str) -> Result<()> {
    let mut cache = load_cache()?;
    let key = key(git_config);
    cache.insert(key, profile_name.to_string());
    let bytes = bincode::serialize(&cache)?;
    fs::write(&cache_path(), &bytes[..])?;

    Ok(())
}

pub fn get(git_config: &Config) -> Option<String> {
    let mut cache = load_cache().ok()?;
    let key = key(git_config);

    cache.remove(&key)
}

// TODO figure out if there's a way to check if config is a snapshot
fn key(git_config: &Config) -> u64 {
    let mut hasher = DefaultHasher::new();
    let fields = [
        git_config.get_str("user.name").unwrap(),
        git_config.get_str("user.email").unwrap(),
        git_config.get_str("core.sshCommand").unwrap()
    ];
    for f in fields {
        f.hash(&mut hasher);
    }

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

fn cache_path() -> String {
    format!("{}/.config/g-profiles/.cache", home())
}
