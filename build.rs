use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};

const HOME: &str = env!("HOME");
const PROFILES_DIR: &str = ".config/g-profiles";

fn main() {
    let profiles = list_profiles()
        .into_iter()
        .map(|profile_name| Profile::read_json(&profile_name))
        .collect::<Vec<_>>();
    // todo load the cache to avoid destroying it
    let mut cache = HashMap::new();
    profiles.iter().for_each(|p| {
        let key = key(&p.username, &p.email);
        cache.insert(key, p.name.to_string());
    });
    let cache_path = format!("{HOME}/{PROFILES_DIR}/.cache");
    let cache_bytes = bincode::serialize(&cache).unwrap();
    // fs::write(cache_path, &cache_bytes[..]).unwrap();
    // todo for some reason this doesn't work
    profiles.into_iter().for_each(Profile::save_binary);
}

fn list_profiles() -> Vec<String> {
    fs::read_dir(format!("{HOME}/{PROFILES_DIR}"))
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| path.ends_with(".json"))
        .map(|path| path.file_stem().unwrap().to_string_lossy().to_string())
        .filter(|name| !name.starts_with('.'))
        .collect()
}

#[derive(Clone)]
struct Profile {
    name: String,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct PartialProfile {
    username: String,
    email: String,
}

impl From<(&str, PartialProfile)> for Profile {
    fn from(args: (&str, PartialProfile)) -> Self {
        let (name, partial) = args;
        Self {
            name: name.to_string(),
            username: partial.username,
            email: partial.email,
        }
    }
}

impl From<Profile> for (String, PartialProfile) {
    fn from(profile: Profile) -> Self {
        let partial = PartialProfile {
            username: profile.username,
            email: profile.email,
        };

        (profile.name, partial)
    }
}

impl Profile {
    fn read_json(profile_name: &str) -> Self {
        let path = format!("{HOME}/{PROFILES_DIR}/{profile_name}.json");
        let bytes = fs::read(&path).unwrap_or_else(|_| panic!("Can't open: {path}"));
        let partial = serde_json::from_slice(&bytes[..]).unwrap();

        (profile_name, partial).into()
    }

    fn save_binary(self) {
        let (profile_name, partial) = self.clone().into();
        let path = format!("{HOME}/{PROFILES_DIR}/{profile_name}");
        let bytes = serde_json::to_vec(&partial).unwrap();
        fs::write(&path, &bytes[..]).unwrap();
    }
}

fn key(username: &str, email: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);
    email.hash(&mut hasher);

    hasher.finish()
}
