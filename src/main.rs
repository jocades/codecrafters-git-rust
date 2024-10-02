use std::fs;

use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::io;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(version, author, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    CatFile {
        #[arg(short, long)]
        pretty_print: bool,

        object_hash: String,
    },
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    use Command::*;
    match cli.command {
        Init => {
            fs::create_dir(".git")?;
            fs::create_dir(".git/objects")?;
            fs::create_dir(".git/refs")?;
            fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
            println!("Initialized git directory")
        }
        CatFile {
            pretty_print,
            object_hash,
        } => {
            // let file = fs::OpenOptions::new()
            //     .read(true)
            //     .write(true)
            //     .create(true)
            //     .open("foo.txt");

            let mut f = fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))?;

            let mut z = ZlibDecoder::new(f);
            let mut buf = String::new();
            z.read_to_string(&mut buf)?;
        }
    }

    Ok(())
}
