use clap::Parser;
use cli::Cli;

mod cli;
mod model;

fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
}
