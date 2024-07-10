use std::env;
use std::path::Path;

use ssh_agent_client_rs::Client;
use ssh_key::{PrivateKey, PublicKey};

use crate::ssh::error::Error;
use crate::ssh::Result;

pub fn add_identity(key: &PrivateKey) -> Result<()> {
    let public = PublicKey::from(key).to_string();
    ssh_agent()?
        .add_identity(key)
        .map_err(|e| Error::AddIdentity(e, public))
}

pub fn remove_identity(key: &PrivateKey) -> Result<()> {
    let public = PublicKey::from(key).to_string();
    ssh_agent()?
        .remove_identity(key)
        .map_err(|e| Error::RemoveIdentity(e, public))
}

fn ssh_agent() -> Result<Client> {
    let ssh_auth_sock = ssh_auth_sock()?;
    let ssh_auth_sock_path = Path::new(&ssh_auth_sock);
    Client::connect(ssh_auth_sock_path)
        .map_err(|_| Error::ConnectToAgent(ssh_auth_sock))
}

fn ssh_auth_sock() -> Result<String> {
    env::var("SSH_AUTH_SOCK").map_err(|_| Error::SshAuthSock)
}
