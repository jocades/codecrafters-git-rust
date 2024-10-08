mod init;
use init::Init;

mod cat_file;
use cat_file::CatFile;

mod hash_object;
use hash_object::HashObject;

mod ls_tree;
use ls_tree::LsTree;

mod write_tree;
use write_tree::WriteTree;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(Init),
    CatFile(CatFile),
    HashObject(HashObject),
    LsTree(LsTree),
    WriteTree(WriteTree),
}

impl Command {
    pub fn execute(&self) -> crate::Result<()> {
        use Command::*;
        match self {
            Init(cmd) => cmd.execute(),
            CatFile(cmd) => cmd.execute(),
            HashObject(cmd) => cmd.execute(),
            LsTree(cmd) => cmd.execute(),
            WriteTree(cmd) => cmd.execute(),
        }
    }
}
