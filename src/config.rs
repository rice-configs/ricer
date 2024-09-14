// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration data management.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use anyhow::{anyhow, Result};
use log::{debug, info, trace};
use std::fmt;
use std::fs::{read_to_string, write};
use std::path::Path;
use toml_edit::{DocumentMut, Item, Key, Table};

mod hook;
mod repo;

#[doc(inline)]
pub use hook::*;
pub use repo::*;


pub enum Entry {
    Repo(RepoEntry),
    CmdHook(CmdHookEntry),
}

pub trait FileParser {
    fn read(&mut self, path: impl AsRef<Path>) -> Result<()>;

    fn write(&mut self, path: impl AsRef<Path>) -> Result<()>;

    fn get_entry(&self, section: impl AsRef<str>, key: impl AsRef<str>) -> Result<Entry>;

    fn add_entry(&mut self, section: impl AsRef<str>, entry: Entry) -> Result<Entry>;
}

