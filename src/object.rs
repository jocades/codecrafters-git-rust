use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, fs};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

#[derive(Debug, PartialEq)]
pub enum Kind {
    Blob,
    Tree,
    Commit,
}

#[derive(Debug)]
pub struct Object<R> {
    kind: Kind,
    size: usize,
    data: R,
}

pub fn hash_to_path(hash: &str) -> PathBuf {
    Path::new(".git/objects").join(&hash[..2]).join(&hash[2..])
}

impl<R> Object<R> {
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Object<()> {
    pub fn new_blob<P: AsRef<Path>>(path: P) -> crate::Result<Object<impl BufRead>> {
        let file = File::open(path)?;
        let stat = file.metadata()?;
        Ok(Object {
            kind: Kind::Blob,
            size: stat.len() as usize,
            data: BufReader::new(file),
        })
    }

    pub fn from_bytes(kind: Kind, bytes: &[u8]) -> Object<impl Read + '_> {
        Object {
            kind,
            size: bytes.len(),
            data: bytes,
        }
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
            data: decoder.take(size as u64),
        })
    }
}

impl<R: Read> Object<R> {
    pub fn compress_and_hash<W>(&mut self, writer: &mut W) -> crate::Result<[u8; 20]>
    where
        W: Write,
    {
        let encoder = ZlibEncoder::new(writer, Compression::default());
        let mut hasher = Hasher::new(encoder);

        write!(hasher, "{} {}\0", self.kind(), self.size())?;
        io::copy(self, &mut hasher)?;

        hasher.writer.finish()?;
        Ok(hasher.inner.finalize().into())
    }

    pub fn write(&mut self) -> crate::Result<[u8; 20]> {
        let temp = std::env::temp_dir().join(format!(
            "{}_{}",
            self.kind(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        ));
        let hash = self.compress_and_hash(&mut File::create(&temp)?)?;
        let path = hash_to_path(&hex::encode(hash));
        fs::create_dir_all(&path.parent().ok_or("no parent path")?)?;
        fs::rename(temp, path)?;
        Ok(hash)
    }
}

#[derive(Debug)]
struct Hasher<W> {
    inner: Sha1,
    writer: W,
}

impl Hasher<()> {
    pub fn new<W: Write>(writer: W) -> Hasher<W> {
        Hasher {
            inner: Sha1::new(),
            writer,
        }
    }
}

impl<W: Write> Write for Hasher<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.inner.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<R: Read> Read for Object<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}

impl<R: BufRead> BufRead for Object<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.data.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.data.consume(amt)
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
