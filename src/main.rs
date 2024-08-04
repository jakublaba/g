use clap::Parser;

use crate::cli::Cli;
use crate::cli::pres::Presentation;

mod cli;
mod ssh;
mod git;
mod profile;
mod util;

const HOME: &str = env!("HOME");

fn main() {
    Cli::parse().present()
}
