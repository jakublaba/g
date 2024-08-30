use std::env;

use clap::Parser;

use crate::cli::pres::Presentation;
use crate::cli::Cli;

mod cli;
mod git;
mod profile;
mod ssh;

fn home() -> String {
    env::var("HOME").unwrap()
}

fn main() {
    if let Err(err) = Cli::parse().present() {
        println!("{err}");
    }
}
