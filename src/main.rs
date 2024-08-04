use clap::Parser;

use crate::cli::Cli;
use crate::cli::pres::Presentation;

mod cli;
mod ssh;
mod git;
mod profile;

const HOME: &str = env!("HOME");

fn main() {
    if let Err(err) = Cli::parse().present() {
        println!("{err}");
    }
}
