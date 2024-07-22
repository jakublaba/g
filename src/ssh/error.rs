use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Can't read key: {key_path}")]
    ReadKey {
        key_path: String,
        cause: ssh_key::Error,
    },
    #[error("Can't write key: {key_path}")]
    WriteKey {
        key_path: String,
        cause: ssh_key::Error,
    },
    #[error("IO error: {path}")]
    Io {
        path: String,
        cause: io::Error,
    },
    #[error("Error interacting with ssh-agent at {ssh_auth_sock}")]
    Agent {
        ssh_auth_sock: String,
        cause: ssh_agent_client_rs::Error,
    },
    #[error("SSH_AUTH_SOCK environment variable is not set")]
    SshAuthSock,
}
