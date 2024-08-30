use serde::{Deserialize, Serialize};
use std::fs;

const HOME: &str = env!("HOME");
const PROFILES_DIR: &str = ".config/g-profiles";

fn main() {
    let profile_names = list_profiles();
    if !profile_names.is_empty() {
        println!("cargo::warning=Detected json profiles");
        profile_names
            .iter()
            .for_each(|n| println!("cargo::warning={n}"));
        println!("cargo::warning=They've been migrated to binary format to ensure compatibility with v2.x.x")
    };
    let profiles = profile_names
        .into_iter()
        .map(|profile_name| Profile::read_json(&profile_name))
        .collect::<Vec<_>>();
    let mut cache = cache::load();
    profiles.iter().for_each(|p| {
        let key = cache::key(&p.username, &p.email);
        cache.insert(key, p.name.to_string());
    });
    cache::save(cache);
    profiles.into_iter().for_each(|profile| {
        let name = profile.name.clone();
        profile.save_binary();
        fs::remove_file(format!("{HOME}/{PROFILES_DIR}/{name}.json")).unwrap()
    });
}

fn list_profiles() -> Vec<String> {
    fs::read_dir(format!("{HOME}/{PROFILES_DIR}"))
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().is_some())
        .filter(|path| path.extension().unwrap() == "json")
        .map(|path| path.file_stem().unwrap().to_string_lossy().to_string())
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
        let bytes = bincode::serialize(&partial)
            .unwrap_or_else(|err| panic!("Can't save profile {}, {err}", self.name));
        fs::write(&path, &bytes[..]).unwrap();
    }
}

mod cache {
    use crate::{HOME, PROFILES_DIR};
    use std::collections::HashMap;
    use std::fs;
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::path::Path;

    pub(super) fn load() -> HashMap<u64, String> {
        let path = format!("{HOME}/{PROFILES_DIR}/.cache");
        if !Path::new(&path).exists() {
            return HashMap::new();
        }
        let bytes = fs::read(&path).unwrap_or_else(|err| panic!("Can't load cache: {err}"));

        bincode::deserialize(&bytes[..]).expect("Can't deserialize cache")
    }

    pub(super) fn save(cache: HashMap<u64, String>) {
        let path = format!("{HOME}/{PROFILES_DIR}/.cache");
        let bytes = bincode::serialize(&cache).expect("Can't serialize cache");
        fs::write(path, &bytes[..]).unwrap_or_else(|err| panic!("Can't save cache: {err}"));
    }

    pub(super) fn key(username: &str, email: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        email.hash(&mut hasher);

        hasher.finish()
    }
}
