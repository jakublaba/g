use clap::{Parser, Subcommand};

use crate::model::Profile;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    Su {
        #[arg(value_parser = |name: &str| Profile::from_json(String::from(name)))]
        profile: Profile,
    },
    Profile {
        #[clap(subcommand)]
        command: Option<ProfileCmd>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfileCmd {
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        email: String,
    },
    Remove {
        #[arg()]
        profile: String,
    },
    Edit {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
}
