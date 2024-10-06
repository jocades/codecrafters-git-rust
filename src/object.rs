use std::ffi::CStr;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;

#[derive(Debug)]
pub enum Kind {
    Blob,
    Tree,
    Commit,
}

#[derive(Debug)]
pub struct Object<R> {
    kind: Kind,
    size: usize,
    pub content: R,
}

fn hash_to_path(hash: &str) -> PathBuf {
    Path::new(".git/objects").join(&hash[..2]).join(&hash[2..])
}

impl<R: Read> Object<R> {
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Object<()> {
    pub fn new<P: AsRef<Path>>(kind: Kind, path: P) -> crate::Result<Object<impl Read>> {
        let file = File::open(path)?;
        let stat = file.metadata()?;
        Ok(Object {
            kind,
            size: stat.len() as usize,
            content: file,
        })
    }

    pub fn new_blob<P: AsRef<Path>>(path: P) -> crate::Result<Object<impl Read>> {
        Object::new(Kind::Blob, path)
    }

    pub fn from_hash(hash: &str) -> crate::Result<Object<impl BufRead>> {
        let file = File::open(hash_to_path(hash))?;
        let mut decoder = BufReader::new(ZlibDecoder::new(file));

        let mut buf = Vec::new();
        decoder.read_until(b'\0', &mut buf)?;

        let (kind, size) = CStr::from_bytes_with_nul(&buf)?
            .to_str()?
            .split_once(' ')
            .ok_or_else(|| "could not parse header")?;

        let size = size.parse::<usize>()?;
        Ok(Object {
            kind: kind.into(),
            size,
            content: decoder.take(size as u64),
        })
    }
}

impl<R: Read> Read for Object<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.content.read(buf)
    }
}

impl<R: BufRead> BufRead for Object<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.content.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.content.consume(amt)
    }
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        match s {
            "blob" => Kind::Blob,
            "tree" => Kind::Tree,
            "commit" => Kind::Commit,
            _ => unreachable!("{s}"),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Tree => write!(f, "tree"),
            Kind::Commit => write!(f, "commit"),
        }
    }
}
