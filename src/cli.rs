use clap::{Parser, Subcommand};

use crate::profile::profile::Profile;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Switch profiles
    Su {
        /// Name of the profile
        #[arg(value_parser = | name: & str | Profile::read_json(name))]
        profile: Profile,
    },
    /// Manage profiles
    Profile {
        #[clap(subcommand)]
        command: Option<ProfileCmd>,
    },
    /// Clone a git repository
    Clone {
        /// Name of profile to use for authorization for cloning
        #[arg(value_parser)]
        profile: String,
        /// Url of the remote repository
        #[arg(value_parser)]
        url: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfileCmd {
    /// Add a new profile
    Add {
        /// Name of the profile
        #[arg(short, long)]
        name: String,
        /// Git username (user.name in gitconfig)
        #[arg(short, long = "username")]
        user_name: String,
        /// Git user email (user.email in gitconfig)
        #[arg(short = 'e', long = "email")]
        user_email: String,
    },
    /// Remove an existing profile
    Remove {
        /// Name of the profile
        #[arg()]
        profile: String,
    },
    /// Edit an existing profile
    Edit {
        /// Name of the profile
        #[arg(short, long)]
        name: String,
        /// Git username (user.name in gitconfig)
        #[arg(short, long = "username")]
        user_name: Option<String>,
        /// Git user email (user.email in gitconfig)
        #[arg(short = 'e', long = "email")]
        user_email: Option<String>,
    },
}
