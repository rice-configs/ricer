// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::data_xchg::TomlEntry;

use std::fmt;
use toml_edit::visit::{visit_table_like_kv, Visit};
use toml_edit::{Array, Item, Key, Table, Value};

/// Repository configuration settings.
///
/// Intermediary structure meant to help make it easier to deserialize and
/// serialize repository configuration file data.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Repository {
    /// Name of repository.
    pub name: String,

    /// Default branch.
    pub branch: String,

    /// Default remote.
    pub remote: String,

    /// Flag to determine if repository's working directory is the user's home
    /// directory through _fake bare_ technique.
    pub workdir_home: bool,

    /// Bootstrap configuration for repository.
    pub bootstrap: Option<Bootstrap>,
}

impl Repository {
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

    pub fn bootstrap(mut self, bootstrap: Bootstrap) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }
}

impl TomlEntry for Repository {
    fn to_toml(&self) -> (Key, Item) {
        let mut repo = Table::new();
        let mut repo_bootstrap = Table::new();

        repo.insert("branch", Item::Value(Value::from(&self.branch)));
        repo.insert("remote", Item::Value(Value::from(&self.remote)));
        repo.insert("workdir_home", Item::Value(Value::from(self.workdir_home)));
        if let Some(bootstrap) = &self.bootstrap {
            if let Some(clone) = &bootstrap.clone {
                repo_bootstrap.insert("clone", Item::Value(Value::from(clone)));
            }
            if let Some(os) = &bootstrap.os {
                repo_bootstrap.insert("os", Item::Value(Value::from(os.to_string())));
            }
            if let Some(users) = &bootstrap.users {
                repo_bootstrap.insert("users", Item::Value(Value::Array(Array::from_iter(users))));
            }
            if let Some(hosts) = &bootstrap.hosts {
                repo_bootstrap.insert("hosts", Item::Value(Value::Array(Array::from_iter(hosts))));
            }
            repo.insert("bootstrap", Item::Table(repo_bootstrap));
        }

        let key = Key::new(&self.name);
        let value = Item::Table(repo);
        (key, value)
    }
}

fn repo_toml<'toml>(entry: (&'toml Key, &'toml Item)) -> Repository {
    let (key, value) = entry;
    let mut bootstrap = Bootstrap::new();
    let mut repo = Repository::new(key.get());
    bootstrap.visit_item(value);
    repo.visit_item(value);

    if !bootstrap.is_empty() {
        repo = repo.bootstrap(bootstrap);
    }
    repo
}

impl<'toml> From<(&'toml Key, &'toml Item)> for Repository {
    fn from(entry: (&'toml Key, &'toml Item)) -> Repository {
        repo_toml(entry)
    }
}

impl From<(Key, Item)> for Repository {
    fn from(entry: (Key, Item)) -> Self {
        let (key, value) = entry;
        repo_toml((&key, &value))
    }
}

impl<'toml> Visit<'toml> for Repository {
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

/// Repository bootstrap configuration settings.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Bootstrap {
    /// URL to clone repository from.
    pub clone: Option<String>,

    /// Bootstrap repository if and only if user is using a specific OS.
    pub os: Option<OsType>,

    /// Bootstrap repository if and only if user is logged on to a specific
    /// set of user accounts.
    pub users: Option<Vec<String>>,

    /// Bootstrap repository if and only if user is logged on to a specific
    /// set of hosts.
    pub hosts: Option<Vec<String>>,
}

impl Bootstrap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clone(mut self, url: impl Into<String>) -> Self {
        self.clone = Some(url.into());
        self
    }

    pub fn os(mut self, os: OsType) -> Self {
        self.os = Some(os);
        self
    }

    pub fn users<I, S>(mut self, users: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut vec = Vec::new();
        vec.extend(users.into_iter().map(Into::into));
        self.users = Some(vec);
        self
    }

    pub fn hosts<I, S>(mut self, hosts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut vec = Vec::new();
        vec.extend(hosts.into_iter().map(Into::into));
        self.hosts = Some(vec);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.clone.is_none() && self.os.is_none() && self.users.is_none() && self.hosts.is_none()
    }
}

impl<'toml> Visit<'toml> for Bootstrap {
    fn visit_table_like_kv(&mut self, key: &'toml str, node: &'toml Item) {
        match key {
            "clone" => {
                if let Some(clone) = node.as_str() {
                    self.clone = Some(clone.to_string())
                }
            }
            "os" => {
                if let Some(os) = node.as_str() {
                    self.os = Some(OsType::from(os))
                }
            }
            "users" => {
                if let Some(users) = node.as_array() {
                    let data = users
                        .into_iter()
                        .map(|s| {
                            s.as_str().unwrap().trim_matches(|c| c == '\"' || c == '\'').to_string()
                        })
                        .collect();
                    self.users = Some(data)
                }
            }
            "hosts" => {
                if let Some(hosts) = node.as_array() {
                    let data = hosts
                        .into_iter()
                        .map(|s| {
                            s.as_str().unwrap().trim_matches(|c| c == '\"' || c == '\'').to_string()
                        })
                        .collect();
                    self.hosts = Some(data)
                }
            }
            &_ => visit_table_like_kv(self, key, node),
        }
        visit_table_like_kv(self, key, node);
    }
}

/// Operating System settings.
///
/// Simple enum used to determine the target OS user wants to bootstrap with.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum OsType {
    /// Bootstrap to any operating system.
    #[default]
    Any,

    /// Bootstrap to Unix-like systems only.
    Unix,

    /// Bootstrap to MacOS systems only.
    MacOs,

    /// Bootstrap to Windows system only.
    Windows,
}

impl From<&str> for OsType {
    fn from(data: &str) -> Self {
        match data {
            "any" => Self::Any,
            "unix" => Self::Unix,
            "macos" => Self::MacOs,
            "windows" => Self::Windows,
            &_ => Self::Any,
        }
    }
}

impl fmt::Display for OsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsType::Any => write!(f, "any"),
            OsType::Unix => write!(f, "unix"),
            OsType::MacOs => write!(f, "macos"),
            OsType::Windows => write!(f, "windows"),
        }
    }
}
