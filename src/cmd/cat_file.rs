use std::fs;
use std::io::{self, BufRead, BufReader, Read};

use clap::Args;
use flate2::read::ZlibDecoder;

#[derive(Args, Debug)]
pub struct CatFile {
    object_hash: String,

    #[arg(short, long)]
    pretty_print: bool,
}

impl CatFile {
    pub fn execute(self) -> crate::Result<()> {
        let f = fs::File::open(format!(
            ".git/objects/{}/{}",
            &self.object_hash[..2],
            &self.object_hash[2..]
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
            Err(format!("unexpected object size, expected {size} got {n}").into())
        } else {
            Ok(())
        }
    }
}
