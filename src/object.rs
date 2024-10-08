use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::{fmt, fs};

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
    pub fn new<R: Read>(kind: Kind, size: usize, data: R) -> crate::Result<Object<impl Read>> {
        Ok(Object { kind, size, data })
    }

    pub fn new_blob<P: AsRef<Path>>(path: P) -> crate::Result<Object<impl BufRead>> {
        let file = File::open(path)?;
        let stat = file.metadata()?;
        Ok(Object {
            kind: Kind::Blob,
            size: stat.len() as usize,
            data: BufReader::new(file),
        })
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
        let mut writer = HashWriter::new(encoder);

        write!(writer, "{} {}\0", self.kind(), self.size())?;
        io::copy(self, &mut writer)?;

        writer.inner.finish()?;
        Ok(writer.hasher.finalize().into())
    }

    pub fn encode_and_write(&mut self) -> crate::Result<[u8; 20]> {
        let tmp = "__tmp";
        let hash = self.compress_and_hash(&mut File::create(tmp)?)?;
        let path = hash_to_path(&hex::encode(hash));
        fs::create_dir_all(&path.parent().ok_or("no parent path")?)?;
        fs::rename(tmp, path)?;
        Ok(hash)
    }
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
