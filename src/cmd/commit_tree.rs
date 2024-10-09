use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Args;

use crate::object::{Kind, Object};

#[derive(Args, Debug)]
pub struct CommitTree {
    tree_sha: String,
    #[arg(short)]
    parent_commit: String,
    #[arg(short)]
    message: String,
}

// tree {tree_sha}
// {parents}
// author {author_name} <{author_email}> {author_date_seconds} {author_date_timezone}
// committer {committer_name} <{committer_email}> {committer_date_seconds} {committer_date_timezone}
//
// {commit message}

impl CommitTree {
    pub fn execute(&self) -> crate::Result<()> {
        let obj = Object::from_hash(&self.tree_sha)?;
        if *obj.kind() != Kind::Tree {
            return Err("cannot commit an object other than a tree".into());
        }

        let date_secs = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut buf = Vec::new();
        writeln!(buf, "tree {}", self.tree_sha)?;
        writeln!(buf, "parent {}", self.parent_commit)?;
        let author = format!("Jordi Calafat <me@mail.com> {date_secs} +0200");
        writeln!(buf, "author {author}")?;
        writeln!(buf, "commiter {author}\n")?; // double nl
        writeln!(buf, "{}", self.message)?;

        let hash = Object::new(Kind::Commit, buf.len(), buf.as_slice())?.write()?;
        println!("{}", hex::encode(hash));

        Ok(())
    }
}
