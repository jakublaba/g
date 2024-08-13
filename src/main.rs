use std::env;

use clap::Parser;

use crate::cli::Cli;
use crate::cli::pres::Presentation;

mod cli;
mod ssh;
mod git;
mod profile;

fn home() -> String {
    env::var("HOME").unwrap()
}

fn main() {
    if let Err(err) = Cli::parse().present() {
        println!("{err}");
    }
}
