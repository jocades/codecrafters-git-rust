#![allow(unused_imports)]

use std::ffi::CStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

use bytes::BytesMut;
use clap::Args;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::object::{self, Object};

#[derive(Args, Debug)]
pub struct LsTree {
    object_hash: String,

    #[arg(long)]
    name_only: bool,
}

// -> IN
// tree <size>\0
// <mode> <name>\0<20_byte_sha>
// <mode> <name>\0<20_byte_sha>

// <- OUT
// 040000 <kind> <tree_sha_1> <name>
// 100644 <kind> <blob_sha_1> <name>
// 100644 <kind> <blob_sha_2> <name>

impl LsTree {
    pub fn execute(self) -> crate::Result<()> {
        let mut obj = Object::from_hash(&self.object_hash)?;

        use object::Kind::*;
        match obj.kind() {
            Tree => {
                let mut buf = Vec::new();
                let mut hashbuf = [0; 20];
                let mut stdout = io::stdout().lock();
                loop {
                    buf.clear();
                    let n = obj.read_until(b'\0', &mut buf)?;
                    if n == 0 {
                        break; // EOF
                    }
                    obj.read_exact(&mut hashbuf)?;

                    let (mode, name) = CStr::from_bytes_with_nul(&buf)?
                        .to_str()?
                        .split_once(' ')
                        .ok_or_else(|| "could not parse header")?;

                    if self.name_only {
                        writeln!(stdout, "{name}")?;
                    } else {
                        let hash = hex::encode(&hashbuf);
                        let obj = Object::from_hash(&hash)?;
                        writeln!(stdout, "{:0>6} {} {:<44} {}", mode, obj.kind(), hash, name)?;
                    }
                }
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}
