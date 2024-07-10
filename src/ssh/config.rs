use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ssh::error::Error;
use crate::ssh::Result;

const HOME: &str = env!("HOME");
const SSH_CONFIG_DIR: &str = ".ssh/config";

pub fn add_config_entry(profile_name: &str) -> Result<()> {
    let path = ssh_config_dir();
    let content = filtered_ssh_config(profile_name)? + &config_entry(profile_name);
    fs::write(&path, content)
        .map_err(|cause| Error::Io { path, cause })
}

fn filtered_ssh_config(excluded_profile_name: &str) -> Result<String> {
    let path = ssh_config_dir();
    let file = File::open(&path)
        .map_err(|cause| Error::Io { path, cause })?;
    let reader = BufReader::new(file);
    let config_entry_lines = config_entry(excluded_profile_name)
        .lines()
        .map(String::from)
        .collect::<HashSet<_>>();
    let content = reader.lines()
        .map(|r| r.unwrap())
        .filter(|line| !config_entry_lines.contains(line))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(content)
}

fn ssh_config_dir() -> String {
    format!("{HOME}/{SSH_CONFIG_DIR}")
}

fn config_entry(profile_name: &str) -> String {
    format!(r#"
Host github.com-{profile_name}
    HostName        github.com
    User            git
    IdentityFile    ~/.ssh/id_{profile_name}
"#)
}
