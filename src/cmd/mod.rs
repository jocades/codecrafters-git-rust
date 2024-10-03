mod init;
pub use init::Init;

mod cat_file;
pub use cat_file::CatFile;

mod hash_object;
pub use hash_object::HashObject;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(Init),
    CatFile(CatFile),
    HashObject(HashObject),
}

impl Command {
    pub fn execute(self) -> crate::Result<()> {
        use Command::*;
        match self {
            Init(cmd) => cmd.execute(),
            CatFile(cmd) => cmd.execute(),
            HashObject(cmd) => cmd.execute(),
        }
    }
}
