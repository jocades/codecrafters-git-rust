use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::Args;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

#[derive(Args, Debug)]
pub struct HashObject {
    file: PathBuf,
    #[arg(short, long)]
    write: bool,
}

impl HashObject {
    pub fn execute(self) -> crate::Result<()> {
        // TODO: Literally creating 4 Vecs here, better to stream the bytes from one buffer to
        // another directly instead of creating intermideate in memory buffers.
        let content = fs::read(self.file)?;
        let mut buf = format!("blob {}\0", content.len()).as_bytes().to_vec();
        buf.extend(content);

        let mut hasher = Sha1::new();
        hasher.update(&buf);

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&buf)?;

        let hash = format!("{:x}", hasher.finalize());
        let compressed = encoder.finish()?;
        println!("{hash}");

        if !self.write {
            return Ok(());
        }

        let dir = PathBuf::from(format!(".git/objects/{}", &hash[..2]));
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(&hash[2..]), compressed)?;
        Ok(())
    }
}
