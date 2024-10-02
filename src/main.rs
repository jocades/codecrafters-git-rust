use std::fs;

use bytes::{Buf, Bytes};
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use std::io::{self, BufReader};

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
        CatFile { object_hash, .. } => {
            let f = fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))?;

            let mut decoded = BufReader::new(ZlibDecoder::new(f));
            let mut buf = Vec::new();

            decoded.read_until(b'\0', &mut buf)?;

            let header = String::from_utf8_lossy(&buf[..buf.len() - 1]);
            let Some(size) = header.split_whitespace().nth(1) else {
                return Err("could not find size".into());
            };
            let size = size.parse::<usize>()?;
            buf.clear();
            buf.resize(size, 0);
            decoded.read_exact(&mut buf)?;

            print!("{}", String::from_utf8(buf)?);
        }
    }

    Ok(())
}
