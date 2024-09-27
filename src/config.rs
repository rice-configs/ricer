// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Format preserving configuration file handling.
//!
//! Ricer's standard configuration file handling module that provides efficient
//! ways to read, write, parse, serialize, and deserialize configuration file
//! data while preserving the original formatting.
//!
//! Ricer uses the [TOML file format][toml-spec] for configuration files.
//! Currently, Ricer splits configuration file data into two main categories:
//! repository configurations, and command hook configurations. Repository
//! configurations mainly deal with detailing _what_ kinds of repositories Ricer
//! needs to keep track of, and _how_ to manage them according to the user's
//! specifications. Command hook configurations specify what hook scripts need
//! to be executed for a given Ricer command, as well as _when_ and _where_
//! those hook scripts need to be executed.
//!
//! Repository configurations and command hook configurations recieve their own
//! separate configuration file. To reflect this design decision, this module
//! provides a repository configuration file handler, and a command hook
//! configuration file handler respectively.
//!
//! This module makes no assumptions about the locations of each configuration
//! file. All it cares about is that the caller provides valid configuration
//! file data to handle.
//!
//! [toml-spec]: https://toml.io/en/v1.0.0

use anyhow::{anyhow, Result};
use log::{debug, info, trace};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use toml_edit::visit::{visit_table_like_kv, visit_inline_table, Visit};
use toml_edit::{Array, InlineTable, DocumentMut, Item, Key, Table, Value};

/// TOML parser.
///
/// Parser only operates on string data, i.e., file I/O is left to the caller.
/// Parsed string data is referred to as a _document_ or _TOML document_.
///
/// Interface of this parser works through _sections_ and _entries_. A _section_
/// is the __topmost__ table housing a set of entries. An _entry_ is a key-value
/// pair in a section. Here is an example of sections and entries:
///
/// ```markdown
/// [repo.vim]
/// branch = "master"
/// remote = "origin"
/// workdir_home = true
///
/// [repo.vim.bootstrap]
/// os = any
///
/// [hooks]
/// commit = "hook.sh"
/// ```
///
/// From the example above, there are two sections: "repo" and "hooks", because
/// "repo" and "hooks" are the topmost tables in the example. Everything else
/// are considered entries to either the "repo" section or "hooks" section.
/// This terminology makes it easier and more predictable to locate
/// configuration data for serialization and deserialization.
///
/// Do note that Ricer does not use the root-table of a TOML document, hence
/// why it is not considered in the previous example.
///
/// # Invariants
///
/// Preserve original formatting for any modifications made to TOML document.
#[derive(Clone, Default, Debug)]
pub struct Toml {
    doc: DocumentMut,
}

impl Toml {
    /// Construct new TOML parser.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::Toml;
    ///
    /// let toml = Toml::new();
    /// ```
    pub fn new() -> Self {
        trace!("Construct new TOML parser");
        Self { doc: DocumentMut::new() }
    }

    /// Get entry data from TOML document.
    ///
    /// Returns key-value pair reference to target entry data.
    ///
    /// # Errors
    ///
    /// Function will fail if `section` does not exist, `section` is not defined
    /// as a table, or `key` does not exist in `section`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use pretty_assertions::assert_eq;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::Toml;
    ///
    /// let config: Toml = indoc! {r#"
    ///     [test]
    ///     foo = "some data"
    /// "#}.parse()?;
    /// let (key, value) = config.get("test", "foo")?;
    /// assert_eq!("foo", key.get());
    /// assert_eq!("some data", value.as_str().unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get<S>(&self, section: S, key: S) -> Result<(&Key, &Item)>
    where
        S: AsRef<str>,
    {
        info!("Get TOML entry '{}' from '{}' section", key.as_ref(), section.as_ref());
        let table = self.section(section.as_ref())?;
        let entry = table.get_key_value(key.as_ref()).ok_or(anyhow!(
            "Entry '{}' does not exist in '{}' section",
            key.as_ref(),
            section.as_ref()
        ))?;
        Ok(entry)
    }

    /// Add entry data into TOML document.
    ///
    /// If entry data already exists in `section`, then it will be replaced by
    /// `entry` with old entry data being returned. Otherwise, `entry` will be
    /// added into `section` returning `None`. If `section` does not exist, then
    /// it will be added into document, and `entry` will be added into newly
    /// created `section`.
    ///
    /// # Errors
    ///
    /// Will fail if `section` was not defined as a table.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use pretty_assertions::assert_eq;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use toml_edit::{Key, Item, Value};
    ///
    /// use ricer::config::Toml;
    ///
    /// let mut config = Toml::new();
    /// let entry = (Key::new("foo"), Item::Value(Value::from("some data")));
    /// let old_entry = config.add("test", entry)?;
    /// let expect = indoc! {r#"
    ///     [test]
    ///     foo = "some data"
    /// "#};
    /// let result = config.to_string();
    /// assert_eq!(expect, result);
    /// assert!(matches!(old_entry, None));
    /// # Ok(())
    /// # }
    /// ```
    pub fn add(
        &mut self,
        section: impl AsRef<str>,
        entry: (Key, Item),
    ) -> Result<Option<(Key, Item)>> {
        let (key, value) = entry;
        info!("Add entry '{}' to '{}' section", key.get(), section.as_ref());
        let old_key = key.clone();
        let old_entry = if let Some(table) = self.doc.get_mut(section.as_ref()) {
            debug!("Section '{}' exists", section.as_ref());
            let table = table
                .as_table_mut()
                .ok_or(anyhow!("Section '{}' not defined as a table", section.as_ref()))?;
            table.insert(key.get(), value)
        } else {
            debug!("Create new '{}' section", section.as_ref());
            let mut table = Table::new();
            table.insert(key.get(), value);
            table.set_implicit(true);
            self.doc.insert(section.as_ref(), Item::Table(table))
        }
        .map(|old_item| (old_key, old_item));
        Ok(old_entry)
    }

    /// Remove entry from TOML document.
    ///
    /// Returns removed entry.
    ///
    /// # Errors
    ///
    /// Function will fail if `section` does not exist, `section` is not defined
    /// as a table, or `key` does not exist in `section`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use pretty_assertions::assert_eq;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::Toml;
    ///
    /// let mut config: Toml = indoc! {r#"
    ///     [test]
    ///     foo = "some data"
    /// "#}.parse()?;
    /// let (key, value) = config.remove("test", "foo")?;
    /// let expect = "[test]\n";
    /// let result = config.to_string();
    /// assert_eq!(expect, result);
    /// assert_eq!("foo", key.get());
    /// assert_eq!("some data", value.as_str().unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove<S>(&mut self, section: S, key: S) -> Result<(Key, Item)>
    where
        S: AsRef<str>,
    {
        info!("Remove entry '{}' from '{}' section", key.as_ref(), section.as_ref());
        let table = self.section_mut(section.as_ref())?;
        let entry = table.remove_entry(key.as_ref()).ok_or(anyhow!(
            "Entry '{}' does not exist in '{}' section",
            key.as_ref(),
            section.as_ref()
        ))?;
        Ok(entry)
    }

    /// Rename entry in TOML document.
    ///
    /// Returns old entry before it was renamed.
    ///
    /// # Errors
    ///
    /// Function will fail if `section` does not exist, `section` is not defined
    /// as a table, or `key` does not exist in `section`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use pretty_assertions::assert_eq;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::Toml;
    ///
    /// let mut config: Toml = indoc! {r#"
    ///     [test]
    ///     foo = "some data"
    /// "#}.parse()?;
    /// let (key, value) = config.rename("test", "foo", "lum")?;
    /// let expect = indoc! {r#"
    ///     [test]
    ///     lum = "some data"
    /// "#};
    /// let result = config.to_string();
    /// assert_eq!(expect, result);
    /// assert_eq!("foo", key.get());
    /// assert_eq!("some data", value.as_str().unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename<S>(&mut self, section: S, from: S, to: S) -> Result<(Key, Item)>
    where
        S: AsRef<str>,
    {
        let table = self.section_mut(section.as_ref())?;
        let (old_key, old_item) = table.remove_entry(from.as_ref()).ok_or(anyhow!(
            "Entry '{}' does not exist in '{}' section",
            from.as_ref(),
            section.as_ref()
        ))?;
        let new_key = Key::new(to.as_ref()).with_leaf_decor(old_key.leaf_decor().clone());
        table.insert_formatted(&new_key, old_item.clone());
        Ok((old_key, old_item))
    }

    /// Get section of TOML document.
    ///
    /// # Errors
    ///
    /// Function will fail if section does not exist, or section is not
    /// defined as a table.
    fn section(&self, name: &str) -> Result<&Table> {
        let table =
            self.doc.get(name.as_ref()).ok_or(anyhow!("Section '{}' does not exist", name))?;
        let table = table.as_table().ok_or(anyhow!("Section '{}' not defined as table", name))?;
        Ok(table)
    }

    /// Get mutable section of TOML document.
    ///
    /// # Errors
    ///
    /// Function will fail if section does not exist, or section is not defined
    /// as a table.
    fn section_mut(&mut self, name: &str) -> Result<&mut Table> {
        let table =
            self.doc.get_mut(name.as_ref()).ok_or(anyhow!("Section '{}' does not exist", name))?;
        let table =
            table.as_table_mut().ok_or(anyhow!("Section '{}' not defined as table", name))?;
        Ok(table)
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

impl FromStr for Toml {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let doc: DocumentMut = data.parse()?;
        Ok(Self { doc })
    }
}

/// Repository configuration settings.
///
/// Intermediary structure meant to help make it easier to deserialize and
/// serialize repository configuration file data.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Repo {
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
    pub bootstrap: Option<RepoBootstrap>,
}

impl Repo {
    /// Build new repository settings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::Repo;
    ///
    /// let builder = Repo::builder("vim");
    /// ```
    pub fn builder(name: impl AsRef<str>) -> RepoBuilder {
        RepoBuilder::new(name.as_ref())
    }

    /// Serialize repository settings to TOML document entry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{Repo, RepoBootstrap};
    ///
    /// let bootstrap = RepoBootstrap::builder()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .build();
    /// let repo = Repo::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .bootstrap(bootstrap)
    ///     .build();
    /// let entry = repo.to_toml();
    /// ```
    pub fn to_toml(&self) -> (Key, Item) {
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

impl<'toml> From<(&'toml Key, &'toml Item)> for Repo {
    fn from(entry: (&'toml Key, &'toml Item)) -> Self {
        let (key, value) = entry;
        let mut bootstrap = RepoBootstrap::builder();
        let mut repo = Repo::builder(key.get());
        bootstrap.visit_item(value);
        repo.visit_item(value);

        let bootstrap = bootstrap.build();
        if !bootstrap.is_empty() {
            repo = repo.bootstrap(bootstrap);
        }
        repo.build()
    }
}

/// Builder for [`Repo`].
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBuilder {
    name: String,
    branch: String,
    remote: String,
    workdir_home: bool,
    bootstrap: Option<RepoBootstrap>,
}

impl RepoBuilder {
    /// Construct new repository setting builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBuilder;
    ///
    /// let builder = RepoBuilder::new("vim");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            branch: Default::default(),
            remote: Default::default(),
            workdir_home: Default::default(),
            bootstrap: Default::default(),
        }
    }

    /// Set default branch to use in repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBuilder;
    ///
    /// let builder = RepoBuilder::new("vim").branch("master");
    /// ```
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = branch.into();
        self
    }

    /// Set default remote to use in repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBuilder;
    ///
    /// let builder = RepoBuilder::new("vim").remote("origin");
    /// ```
    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.remote = remote.into();
        self
    }

    /// Set repository to use user's home as the main working directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBuilder;
    ///
    /// let builder = RepoBuilder::new("vim").workdir_home(true);
    /// ```
    pub fn workdir_home(mut self, choice: bool) -> Self {
        self.workdir_home = choice;
        self
    }

    /// Set bootstrapping options for repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBuilder, RepoBootstrap};
    ///
    /// let bootstrap = RepoBootstrap::builder()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .build();
    /// let builder = RepoBuilder::new("vim").bootstrap(bootstrap);
    /// ```
    pub fn bootstrap(mut self, bootstrap: RepoBootstrap) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }

    /// Build new [`Repo`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBuilder, RepoBootstrap};
    ///
    /// let bootstrap = RepoBootstrap::builder()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .build();
    /// let repo = RepoBuilder::new("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .bootstrap(bootstrap)
    ///     .build();
    /// ```
    pub fn build(self) -> Repo {
        Repo {
            name: self.name,
            branch: self.branch,
            remote: self.remote,
            workdir_home: self.workdir_home,
            bootstrap: self.bootstrap,
        }
    }
}

impl<'toml> Visit<'toml> for RepoBuilder {
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

/// Repository bootstrap configuration settigns.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrap {
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

impl RepoBootstrap {
    /// Build new bootstrap settings for repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrap;
    ///
    /// let builder = RepoBootstrap::builder();
    /// ```
    pub fn builder() -> RepoBootstrapBuilder {
        RepoBootstrapBuilder::new()
    }

    /// Determine if repository has not bootstrap settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer::config::RepoBootstrap;
    ///
    /// let bootstrap = RepoBootstrap::default();
    /// assert!(bootstrap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.clone.is_none() && self.os.is_none() && self.users.is_none() && self.hosts.is_none()
    }
}

/// Repository bootstrap configuration settigns.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrapBuilder {
    clone: Option<String>,
    os: Option<OsType>,
    users: Option<Vec<String>>,
    hosts: Option<Vec<String>>,
}

impl RepoBootstrapBuilder {
    /// Construct new repository bootstrap settings builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapBuilder;
    ///
    /// let builder = RepoBootstrapBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set URL to clone repository from.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapBuilder;
    ///
    /// let builder = RepoBootstrapBuilder::new().clone("https://github.com/awkless/vim.git");
    /// ```
    pub fn clone(mut self, url: impl Into<String>) -> Self {
        self.clone = Some(url.into());
        self
    }

    /// Set target operating system to bootstrap repository too.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBootstrapBuilder, OsType};
    ///
    /// let builder = RepoBootstrapBuilder::new().os(OsType::Any);
    /// ```
    pub fn os(mut self, os: OsType) -> Self {
        self.os = Some(os);
        self
    }

    /// Set target users to bootsrap repository too.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapBuilder;
    ///
    /// let builder = RepoBootstrapBuilder::new().users(["awkless", "turing"]);
    /// ```
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

    /// Set target hosts to bootstrap repository too.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapBuilder;
    ///
    /// let builder = RepoBootstrapBuilder::new().hosts(["awkless", "turing"]);
    /// ```
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

    /// Build [`RepoBootstrap`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBootstrapBuilder, OsType};
    ///
    /// let bootstrap = RepoBootstrapBuilder::new()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .os(OsType::Unix)
    ///     .users(["awkless", "knuth", "goodwill"])
    ///     .hosts(["lovelace", "dijkstra", "turing"])
    ///     .build();
    /// ```
    pub fn build(self) -> RepoBootstrap {
        RepoBootstrap { clone: self.clone, os: self.os, users: self.users, hosts: self.hosts }
    }
}

impl<'toml> Visit<'toml> for RepoBootstrapBuilder {
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

/// Command hook settings.
///
/// An intermediary structure to help deserialize and serialize command hook
/// from Ricer's command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CmdHook {
    /// Name of command to bind hook definitions too.
    pub cmd: String,

    /// Array of hook definitions to execute.
    pub hooks: Vec<Hook>,
}

impl CmdHook {
    /// Construct new command hook definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::CmdHook;
    ///
    /// let cmd_hook = CmdHook::new("commit");
    /// ```
    pub fn new(cmd: impl Into<String>) -> Self {
        Self { cmd: cmd.into(), hooks: Default::default() }
    }

    /// Add hook definition into command hook list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{CmdHook, Hook};
    ///
    /// let mut cmd_hook = CmdHook::new("commit");
    /// let hook = Hook::builder()
    ///     .pre("hook.sh")
    ///     .post("hook.sh")
    ///     .workdir("/path/to/work/dir")
    ///     .build();
    /// cmd_hook.add_hook(hook);
    /// ```
    pub fn add_hook(&mut self, hook: Hook) {
        self.hooks.push(hook);
    }

    /// Serialize command hook into TOML document entry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{CmdHook, Hook};
    ///
    /// let mut cmd_hook = CmdHook::new("commit");
    /// let hook = Hook::builder()
    ///     .pre("hook.sh")
    ///     .post("hook.sh")
    ///     .workdir("/path/to/work/dir")
    ///     .build();
    /// cmd_hook.add_hook(hook);
    /// let entry = cmd_hook.to_toml();
    /// ```
    pub fn to_toml(&self) -> (Key, Item) {
        let mut tables = Array::new();
        let mut iter = self.hooks.iter().enumerate().peekable();
        while let Some((_, hook)) = iter.next() {
            let mut inline = InlineTable::new();
            let decor = inline.decor_mut();
            decor.set_prefix("\n    ");

            if iter.peek().is_none() {
                decor.set_prefix("\n");
            }

            if let Some(pre) = &hook.pre {
                inline.insert("pre", Value::from(pre));
            }

            if let Some(post) = &hook.post {
                inline.insert("post", Value::from(post));
            }

            if let Some(workdir) = &hook.workdir {
                inline.insert("workdir", Value::from(String::from(workdir.to_string_lossy())));
            }

            tables.push_formatted(Value::from(inline));
        }

        let key = Key::new(&self.cmd);
        let value = Item::Value(Value::from(tables));
        (key, value)
    }
}

impl<'toml> From<(&'toml Key, &'toml Item)> for CmdHook {
    fn from(entry: (&'toml Key, &'toml Item)) -> Self {
        let (key, value) = entry;
        let mut cmd_hook = CmdHook::new(key.get());
        cmd_hook.visit_item(value);
        cmd_hook
    }
}

impl<'toml> Visit<'toml> for CmdHook {
    fn visit_inline_table(&mut self, node: &'toml InlineTable) {
        let pre = if let Some(pre) = node.get("pre") { pre.as_str() } else { None };
        let post = if let Some(post) = node.get("post") { post.as_str() } else { None };
        let workdir = if let Some(workdir) = node.get("workdir") { workdir.as_str() } else { None };

        let hook = Hook::builder();
        let hook = if let Some(pre) = pre { hook.pre(pre) } else { hook };
        let hook = if let Some(post) = post { hook.post(post) } else { hook };
        let hook = if let Some(workdir) = workdir { hook.workdir(workdir) } else { hook };
        self.add_hook(hook.build());

        visit_inline_table(self, node);
    }
}

/// Hook definition settings.
///
/// An intermediary structure to help deserialize and serialize hook entries
/// for command hook settings in command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Hook {
    /// Execute hook script _before_ command itself.
    pub pre: Option<String>,

    /// Execute hook script _after_ command itself.
    pub post: Option<String>,

    /// Set working directory of hook script.
    pub workdir: Option<PathBuf>,
}

impl Hook {
    /// Build new hook definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::Hook;
    ///
    /// let hook = Hook::builder()
    ///     .pre("hook.sh")
    ///     .pre("hook.sh")
    ///     .workdir("/path/to/work/dir/")
    ///     .build();
    /// ```
    pub fn builder() -> HookBuilder {
        HookBuilder::new()
    }
}

/// Builder for [`Hook`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HookBuilder {
    pre: Option<String>,
    post: Option<String>,
    workdir: Option<PathBuf>,
}

impl HookBuilder {
    /// Construct new hook builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookBuilder;
    ///
    /// let builder = HookBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set hook to run _before_ command.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookBuilder;
    ///
    /// let builder = HookBuilder::new().pre("hook.sh");
    /// ```
    pub fn pre(mut self, script: impl Into<String>) -> Self {
        self.pre = Some(script.into());
        self
    }

    /// Set hook to run _after_ command.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookBuilder;
    ///
    /// let builder = HookBuilder::new().post("hook.sh");
    /// ```
    pub fn post(mut self, script: impl Into<String>) -> Self {
        self.post = Some(script.into());
        self
    }

    /// Set working directory of hook script.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookBuilder;
    ///
    /// let builder = HookBuilder::new().workdir("/path/to/work/dir");
    /// ```
    pub fn workdir(mut self, path: impl Into<PathBuf>) -> Self {
        self.workdir = Some(path.into());
        self
    }

    /// Build new [`Hook`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookBuilder;
    ///
    /// let hook = HookBuilder::new()
    ///     .pre("hook.sh")
    ///     .pre("hook.sh")
    ///     .workdir("/path/to/work/dir/")
    ///     .build();
    /// ```
    pub fn build(self) -> Hook {
        Hook { pre: self.pre, post: self.post, workdir: self.workdir }
    }
}
