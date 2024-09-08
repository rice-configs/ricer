// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration data management.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use toml_edit::DocumentMut;

mod locator;

#[doc(inline)]
pub use locator::*;

#[derive(Debug, Default)]
pub struct ReposConfig {
    doc: DocumentMut,
}

impl ReposConfig {
    // TODO: Implement this...
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoEntry {
    pub name: String,
    pub branch: String,
    pub remote: String,
    pub workdir_home: bool,
    pub bootstrap: Option<RepoBootstrapEntry>
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrapEntry {
    pub clone: String,
    pub os: OsType,
    pub users: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum OsType {
    #[default]
    Any,

    Unix,

    MacOs,

    Windows,
}
