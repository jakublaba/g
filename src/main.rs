use clap::Parser;

use crate::cli::{Cli, Cmd, ProfileCmd};

mod cli;
mod ssh;
mod git;
mod profile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("{:#?}", cli);

    if let Some(cmd) = cli.command {
        match cmd {
            Cmd::Su { profile } => {
                git::configure_user(&profile)?;
            }
            Cmd::Profile { command } => {
                if let Some(prof_cmd) = command {
                    match prof_cmd {
                        ProfileCmd::Add { name, user_name, user_email } => {}
                        ProfileCmd::Remove { profile } => {}
                        ProfileCmd::Edit { name, user_name, user_email } => {}
                    }
                }
            }
            Cmd::Clone { url } => {}
        }
    };

    Ok(())
}
