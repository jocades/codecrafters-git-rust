use clap::Parser;
use codecrafters_git::Command;

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
