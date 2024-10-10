#![allow(unused_imports)]
use std::io::{self};
use std::path::PathBuf;

use clap::Args;

use crate::Object;

#[derive(Args, Debug)]
pub struct Clone {
    url: String,
    path: Option<PathBuf>,
}

impl Clone {
    pub fn execute(&self) -> crate::Result<()> {
        let body = reqwest::blocking::get(&self.url)?.text()?;

        println!("body = {body:?}");

        Ok(())
    }
}
