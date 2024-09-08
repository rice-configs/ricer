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

impl RepoEntry {
    pub fn builder(name: impl Into<String>) -> RepoEntryBuilder {
        RepoEntryBuilder::new(name)
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoEntryBuilder {
    name: String,
    branch: String,
    remote: String,
    workdir_home: bool,
    bootstrap: Option<RepoBootstrapEntry>
}

impl RepoEntryBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            branch: Default::default(),
            remote: Default::default(),
            workdir_home: Default::default(),
            bootstrap: Default::default(),
        }
    }

    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = branch.into();
        self
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.remote = remote.into();
        self
    }

    pub fn workdir_home(mut self, choice: bool) -> Self {
        self.workdir_home = choice;
        self
    }

    pub fn bootstrap(mut self, bootstrap: RepoBootstrapEntry) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }

    pub fn build(self) -> RepoEntry {
        RepoEntry {
            name: self.name,
            branch: self.branch,
            remote: self.remote,
            workdir_home: self.workdir_home,
            bootstrap: self.bootstrap,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrapEntry {
    pub clone: String,
    pub os: OsType,
    pub users: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
}

impl RepoBootstrapEntry {
    pub fn builder() -> RepoBootstrapEntryBuilder {
        RepoBootstrapEntryBuilder::new()
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrapEntryBuilder {
    clone: String,
    os: OsType,
    users: Option<Vec<String>>,
    hosts: Option<Vec<String>>,
}

impl RepoBootstrapEntryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clone(mut self, url: impl Into<String>) -> Self {
        self.clone = url.into();
        self
    }

    pub fn os(mut self, os: OsType) -> Self {
        self.os = os;
        self
    }

    pub fn users<I, S>(mut self, users: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.users = Some(Vec::new());
        for user in users {
            self.users.get_or_insert(vec![]).push(user.into());
        }
        self
    }

    pub fn hosts<I, S>(mut self, hosts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.hosts = Some(Vec::new());
        for host in hosts {
            self.users.get_or_insert(vec![]).push(host.into());
        }
        self
    }

    pub fn build(self) -> RepoBootstrapEntry {
        RepoBootstrapEntry {
            clone: self.clone,
            os: self.os,
            users: self.users,
            hosts: self.hosts,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum OsType {
    #[default]
    Any,

    Unix,

    MacOs,

    Windows,
}
