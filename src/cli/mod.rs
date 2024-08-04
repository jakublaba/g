use clap::{Parser, Subcommand};

use crate::profile::profile::Profile;
use crate::ssh::key::r#type::KeyType;

mod error;
pub mod pres;

type Result<T> = std::result::Result<T, error::Error>;

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Cmd,
}

// TODO don't expose error when reading profile on cli level fails
#[derive(Subcommand, Debug)]
pub(super) enum Cmd {
    /// Switch profiles
    Su {
        /// Name of the profile
        #[arg(
            value_parser = | name: & str | Profile::load(name)
            .map_err(| e | format ! ("Can't read profile '{name}', cause:\n{e}"))
        )]
        profile: Profile,
        /// Set the profile for global git config
        #[arg(short, long)]
        global: bool,
    },
    /// Show currently set profile
    #[clap(name = "whoami")]
    WhoAmI {
        /// Look up the profile in global config instead
        #[arg(short, long)]
        global: bool
    },
    /// Manage profiles
    Profile {
        #[clap(subcommand)]
        command: ProfileCmd,
    },
}

#[derive(Subcommand, Debug)]
pub(super) enum ProfileCmd {
    /// List existing profiles
    List {
        /// List profiles from cache instead of disk
        #[arg(short, long)]
        cached: bool,
    },
    /// Inspect a profile
    Show {
        /// Name of the profile
        name: String,
    },
    /// Add a new profile
    Add {
        /// Name of the profile
        name: String,
        /// Git username (user.name in gitconfig)
        #[arg(short, long)]
        username: String,
        /// Git user email (user.email in gitconfig)
        #[arg(short, long)]
        email: String,
        /// Override profile if exists
        #[arg(short, long)]
        force: bool,
        /// Type of ssh key: dsa, rsa or ed255119 (default)
        /// To generate rsa key with specific size, use rsa<size>, e.g. rsa4096
        #[arg(
            short, long, value_parser = KeyType::parse, default_value = "ed25519", verbatim_doc_comment
        )]
        key_type: KeyType,
    },
    /// Remove an existing profile
    Remove {
        /// Name of the profile(s)
        profiles: Vec<String>,
    },
    /// Edit an existing profile
    Edit {
        /// Name of the profile
        name: String,
        /// Git username (user.name in gitconfig)
        #[arg(short, long)]
        username: Option<String>,
        /// Git user email (user.email in gitconfig)
        #[arg(short, long)]
        email: Option<String>,
        /// Re-generate ssh keys
        #[arg(short, long)]
        regenerate: bool,
        /// Applicable only if --regenerate is used
        /// Type of ssh key: dsa, rsa or ed25519 (default)
        /// To generate rsa key with specific size, use rsa<size>, e.g. rsa2048
        #[arg(
            short, long, value_parser = KeyType::parse, default_value = "ed25519",
            verbatim_doc_comment, requires = "regenerate"
        )]
        key_type: KeyType,
    },
}
