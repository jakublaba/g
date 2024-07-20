use clap::Parser;
use serde::ser::Error;

use crate::cli::Cli;

mod cli;
mod ssh;
mod git;
mod profile;

fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
}
