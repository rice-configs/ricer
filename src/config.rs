// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration data management.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use std::fmt::{Display, Formatter, Result};
use toml_edit::visit::{visit_table_like_kv, Visit};
use toml_edit::{Array, DocumentMut, InlineTable, Item, Key, Table, Value};

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
    pub bootstrap: Option<RepoBootstrapEntry>,
}

impl RepoEntry {
    pub fn builder(name: impl Into<String>) -> RepoEntryBuilder {
        RepoEntryBuilder::new(name)
    }

    pub fn to_toml(&self) -> (Key, Item) {
        let mut repo_entry = Table::new();
        let mut bootstrap_entry = Table::new();

        repo_entry.insert("branch", Item::Value(Value::from(&self.branch)));
        repo_entry.insert("remote", Item::Value(Value::from(&self.remote)));
        repo_entry.insert("workdir_home", Item::Value(Value::from(self.workdir_home)));
        if let Some(bootstrap) = &self.bootstrap {
            bootstrap_entry.insert("clone", Item::Value(Value::from(&bootstrap.clone)));
            bootstrap_entry.insert("os", Item::Value(Value::from(bootstrap.os.to_string())));

            if let Some(users) = &bootstrap.users {
                bootstrap_entry.insert("users", Item::Value(Value::Array(Array::from_iter(users))));
            }

            if let Some(hosts) = &bootstrap.hosts {
                bootstrap_entry.insert("hosts", Item::Value(Value::Array(Array::from_iter(hosts))));
            }

            repo_entry.insert("bootstrap", Item::Table(bootstrap_entry));
        }

        let key = Key::new(&self.name);
        let value = Item::Table(repo_entry);
        (key, value)
    }
}

impl<'toml> From<(&'toml Key, &'toml Item)> for RepoEntry {
    fn from(toml_entry: (&'toml Key, &'toml Item)) -> Self {
        let (key, value) = toml_entry;
        let mut bootstrap = RepoBootstrapEntry::builder();
        let mut repo = RepoEntry::builder(key.get());
        bootstrap.visit_item(value);
        repo.visit_item(value);
        repo.bootstrap(bootstrap.build()).build()
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoEntryBuilder {
    name: String,
    branch: String,
    remote: String,
    workdir_home: bool,
    bootstrap: Option<RepoBootstrapEntry>,
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

impl<'toml> Visit<'toml> for RepoEntryBuilder {
    fn visit_table_like_kv(&mut self, key: &'toml str, node: &'toml Item) {
        match key {
            "branch" => self.branch = node.as_str().unwrap_or_default().to_string(),
            "remote" => self.remote = node.as_str().unwrap_or_default().to_string(),
            "workdir_home" => self.workdir_home = node.as_bool().unwrap_or_default(),
            &_ => visit_table_like_kv(self, key, node),
        }
        visit_table_like_kv(self, key, node);
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
        RepoBootstrapEntry { clone: self.clone, os: self.os, users: self.users, hosts: self.hosts }
    }
}

impl<'toml> Visit<'toml> for RepoBootstrapEntryBuilder {
    fn visit_table_like_kv(&mut self, key: &'toml str, node: &'toml Item) {
        match key {
            "clone" => self.clone = node.as_str().unwrap_or_default().to_string(),
            "os" => self.os = OsType::from(node.as_str().unwrap_or_default()),

            "hosts" => {
                if let Some(hosts) = node.as_array() {
                    self.hosts = Some(hosts.into_iter().map(|s| s.to_string()).collect())
                }
            }

            "users" => {
                if let Some(users) = node.as_array() {
                    self.users = Some(users.into_iter().map(|s| s.to_string()).collect())
                }
            }

            &_ => visit_table_like_kv(self, key, node),
        }
        visit_table_like_kv(self, key, node);
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

impl From<&str> for OsType {
    fn from(data: &str) -> Self {
        match data {
            "any" => Self::Any,
            "unix" => Self::Unix,
            "MacOs" => Self::MacOs,
            "Windows" => Self::Windows,
            &_ => Self::Any,
        }
    }
}

impl Display for OsType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            OsType::Any => write!(f, "any"),
            OsType::Unix => write!(f, "unix"),
            OsType::MacOs => write!(f, "macos"),
            OsType::Windows => write!(f, "windows"),
        }
    }
}
