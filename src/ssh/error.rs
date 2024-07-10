use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can't open ssh config: {0:?}")]
    OpenConfig(io::Error),
    #[error("Can't write profile {0} to ssh config: {1:?}")]
    WriteConfig(String, io::Error),
    #[error("Can't write private key: {0}")]
    WritePrivateKey(String),
    #[error("Can't write public key: {0}")]
    WritePublicKey(String),
    #[error("SSH_AUTH_SOCK env var is not set")]
    SshAuthSock,
    #[error("Can't add identity {1:?} to ssh-agent\n{0:?}")]
    AddIdentity(ssh_agent_client_rs::Error, String),
    #[error("Can't remove identity {1:?} from ssh-agent\n{0:?}")]
    RemoveIdentity(ssh_agent_client_rs::Error, String),
    #[error("Can't connect to ssh-agent at {0}")]
    ConnectToAgent(String),
}
