use std::io;

use clap::Args;

use crate::object::{self, Object};

// BLOB -> 08909652063260f9df101353883a4f68edbeed56
// TREE -> 6a7b3b87b66664eb367297e05d7df885e898c5a5

#[derive(Args, Debug)]
pub struct CatFile {
    object_hash: String,

    #[arg(short, long)]
    pretty_print: bool,
}

impl CatFile {
    pub fn execute(self) -> crate::Result<()> {
        let mut obj = Object::from_hash(&self.object_hash)?;

        use object::Kind::*;
        match obj.kind() {
            Blob => {
                let mut stdout = io::stdout().lock();
                let n = io::copy(&mut obj, &mut stdout)?;

                if n as usize != obj.size() {
                    Err(format!("unexpected object size, got {} want {}", n, obj.size()).into())
                } else {
                    Ok(())
                }
            }
            _ => unimplemented!(),
        }
    }
}
