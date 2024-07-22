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
}
