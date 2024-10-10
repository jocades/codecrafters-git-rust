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

mod commit_tree;
use commit_tree::CommitTree;

mod clone;
use clone::Clone;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(Init),
    CatFile(CatFile),
    HashObject(HashObject),
    LsTree(LsTree),
    WriteTree(WriteTree),
    CommitTree(CommitTree),
    Clone(Clone),
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
            CommitTree(cmd) => cmd.execute(),
            Clone(cmd) => cmd.execute(),
        }
    }
}
