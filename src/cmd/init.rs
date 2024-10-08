use std::fs;

use clap::Args;

#[derive(Args, Debug)]
pub struct Init;

impl Init {
    pub fn execute(&self) -> crate::Result<()> {
        fs::create_dir(".git")?;
        fs::create_dir(".git/objects")?;
        fs::create_dir(".git/refs")?;
        fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
        println!("Initialized git directory");
        Ok(())
    }
}
