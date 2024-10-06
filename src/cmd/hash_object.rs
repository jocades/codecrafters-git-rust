use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use clap::Args;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::Object;

#[derive(Args, Debug)]
pub struct HashObject {
    file: PathBuf,
    #[arg(short, long)]
    write: bool,
}

struct HashWriter<W: Write> {
    inner: W,
    hasher: Sha1,
}

impl<W: Write> HashWriter<W> {
    pub fn new(writer: W) -> HashWriter<W> {
        HashWriter {
            inner: writer,
            hasher: Sha1::new(),
        }
    }
}

impl<W: Write> Write for HashWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn hash_and_compress<R: Read, W: Write>(mut obj: Object<R>, writer: W) -> crate::Result<String> {
    let encoder = ZlibEncoder::new(writer, Compression::default());
    let mut writer = HashWriter::new(encoder);

    write!(writer, "{} {}\0", obj.kind(), obj.size())?;
    io::copy(&mut obj, &mut writer)?;

    writer.inner.finish()?;
    Ok(hex::encode(writer.hasher.finalize()))
}

impl HashObject {
    pub fn execute(self) -> crate::Result<()> {
        let obj = Object::new_blob(&self.file)?;

        let hash = if !self.write {
            hash_and_compress(obj, io::sink())?
        } else {
            let tmp = "__tmp__";
            let hash = hash_and_compress(obj, File::create(tmp)?)?;
            let dir = Path::new(".git/objects").join(&hash[..2]);
            fs::create_dir_all(&dir)?;
            fs::rename(tmp, dir.join(&hash[2..]))?;
            hash
        };

        println!("{hash}");
        Ok(())
    }
}
