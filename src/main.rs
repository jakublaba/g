use std::env;

use clap::Parser;

use crate::cli::Cli;
use crate::cli::pres::Execute;

mod cli;
mod ssh;
mod git;
mod profile;
mod util;

fn main() {
    Cli::parse()
        .command
        .execute()
}

// TODO could it actually be safe to just evaluate $HOME at compile time?
pub fn home() -> String {
    env::var("HOME").unwrap()
}
