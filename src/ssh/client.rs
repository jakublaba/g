use std::path::Path;

use ssh_agent_client_rs::Client;
use ssh_key::{PrivateKey, PublicKey};

use crate::ssh::{Result, SshError};

const SSH_AUTH_SOCK: &str = env!("SSH_AUTH_SOCK");

pub fn add_identity(key: &PrivateKey) -> Result<()> {
    let public = PublicKey::from(key).to_string();
    ssh_agent()?
        .add_identity(key)
        .map_err(|_| SshError::from(format!("Error adding new identity to ssh-agent\n{public}")))
}

pub fn remove_identity(key: &PrivateKey) -> Result<()> {
    let public = PublicKey::from(key).to_string();
    ssh_agent()?
        .remove_identity(key)
        .map_err(|_| SshError::from(format!("Error removing identity from ssh-agent\n{public}")))
}

fn ssh_agent() -> Result<Client> {
    let ssh_auth_sock = Path::new(SSH_AUTH_SOCK);
    Client::connect(ssh_auth_sock)
        .map_err(|_| SshError::from(format!("Can't connect to ssh-agent at {SSH_AUTH_SOCK}")))
}
