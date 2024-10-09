use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use clap::Args;

use crate::object::{Kind, Object};

#[derive(Args, Debug)]
pub struct WriteTree;

/// .gitignore
const IGNORE: [&'static str; 2] = ["./.git", "./target"];

fn write_tree<P: AsRef<Path>>(path: P) -> crate::Result<Option<[u8; 20]>> {
    let mut entries = fs::read_dir(&path)?
        .filter_map(|e| e.ok())
        .filter(|e| !IGNORE.iter().any(|pattern| e.path().starts_with(pattern)))
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.file_name());

    let mut buf = Vec::new();
    for entry in entries {
        let name = dbg!(entry.file_name());
        let meta = entry.metadata()?;
        let mode = if meta.is_dir() {
            "40000"
        } else if meta.is_symlink() {
            "120000"
        } else if meta.permissions().mode() & 0o111 != 0 {
            "100755"
        } else {
            "100644"
        };

        let path = entry.path();
        let hash = if meta.is_dir() {
            let Some(hash) = write_tree(&path)? else {
                continue;
            };
            hash
        } else {
            Object::new_blob(&path)?.write()?
        };

        buf.extend(mode.as_bytes());
        buf.push(b' ');
        buf.extend(name.as_encoded_bytes());
        buf.push(0);
        buf.extend(hash);
    }

    if buf.is_empty() {
        return Ok(None);
    }

    let hash = Object::from_bytes(Kind::Tree, &buf)?.write()?;
    Ok(Some(hash))
}

impl WriteTree {
    pub fn execute(&self) -> crate::Result<()> {
        let hash = write_tree(".")?.ok_or_else(|| "empty tree")?;
        println!("{}", hex::encode(hash));
        Ok(())
    }
}
