use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use hex_literal::hex;
use sha1::{Digest, Sha1};

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
        object_hash: String,
        #[arg(short, long)]
        pretty_print: bool,
    },
    HashObject {
        file: PathBuf,
        #[arg(short, long)]
        write: bool,
    },
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

            let mut docoder = BufReader::new(ZlibDecoder::new(f));
            let mut buf = Vec::new();

            let _ = docoder.read_until(b'\0', &mut buf)?;
            let header = String::from_utf8_lossy(&buf[..buf.len() - 1]);
            let Some(size) = header.split_whitespace().nth(1) else {
                return Err("could not find size".into());
            };
            let size = size.parse::<u64>()?;
            let mut content = docoder.take(size);
            let mut stdout = io::stdout().lock();
            let n = io::copy(&mut content, &mut stdout)?;
            if n != size {
                return Err(format!("unexpected object size, expected {size} got {n}").into());
            }
        }
        HashObject { file, write } => {
            // TODO: Literally creating 4 Vecs here, better to stream the bytes from one buffer to
            // another directly instead of creating intermideate in memory buffers.
            let content = fs::read(file)?;
            let mut buf = format!("blob {}\0", content.len()).as_bytes().to_vec();
            buf.extend(content);

            let mut hasher = Sha1::new();
            hasher.update(&buf);

            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&buf)?;

            let hash = format!("{:x}", hasher.finalize());
            let compressed = encoder.finish()?;
            println!("{hash}");

            if !write {
                return Ok(());
            }

            let dir = PathBuf::from(format!(".git/objects/{}", &hash[..2]));
            fs::create_dir_all(&dir)?;
            fs::write(dir.join(&hash[2..]), compressed)?;
        }
    }

    Ok(())
}
