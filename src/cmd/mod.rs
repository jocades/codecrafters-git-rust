mod init;
pub use init::Init;

mod cat_file;
pub use cat_file::CatFile;

mod hash_object;
pub use hash_object::HashObject;

mod ls_tree;
pub use ls_tree::LsTree;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(Init),
    CatFile(CatFile),
    HashObject(HashObject),
    LsTree(LsTree),
}

impl Command {
    pub fn execute(self) -> crate::Result<()> {
        use Command::*;
        match self {
            Init(cmd) => cmd.execute(),
            CatFile(cmd) => cmd.execute(),
            HashObject(cmd) => cmd.execute(),
            LsTree(cmd) => cmd.execute(),
        }
    }
}
