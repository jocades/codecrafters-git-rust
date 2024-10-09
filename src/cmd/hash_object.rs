use std::io::{self};
use std::path::PathBuf;

use clap::Args;

use crate::Object;

#[derive(Args, Debug)]
pub struct HashObject {
    file: PathBuf,
    #[arg(short, long)]
    write: bool,
}

impl HashObject {
    pub fn execute(&self) -> crate::Result<()> {
        let mut obj = Object::new_blob(&self.file)?;

        let hash = if !self.write {
            obj.compress_and_hash(&mut io::sink())?
        } else {
            obj.write()?
        };

        println!("{}", hex::encode(hash));
        Ok(())
    }
}
