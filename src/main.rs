mod cmd;
mod object;

use crate::cmd::Command;
use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(version, author, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.command.execute() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
