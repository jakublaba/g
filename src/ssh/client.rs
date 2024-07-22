use std::env;
use std::path::Path;

use ssh_agent_client_rs::Client;
use ssh_key::PrivateKey;

use crate::ssh::error::Error;
use crate::ssh::Result;

pub fn add_identity(identity: &PrivateKey) -> Result<()> {
    let ssh_auth_sock = ssh_auth_sock()?;
    ssh_agent()?
        .add_identity(identity)
        .map_err(|cause| Error::Agent { ssh_auth_sock, cause })
}

pub fn remove_identity(identity: &PrivateKey) -> Result<()> {
    let ssh_auth_sock = ssh_auth_sock()?;
    ssh_agent()?
        .remove_identity(identity)
        .map_err(|cause| Error::Agent { ssh_auth_sock, cause })
}

fn ssh_agent() -> Result<Client> {
    let ssh_auth_sock = ssh_auth_sock()?;
    Client::connect(Path::new(&ssh_auth_sock))
        .map_err(|cause| Error::Agent { ssh_auth_sock, cause })
}

fn ssh_auth_sock() -> Result<String> {
    env::var("SSH_AUTH_SOCK").map_err(|_| Error::SshAuthSock)
}
