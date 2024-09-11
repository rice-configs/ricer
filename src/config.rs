// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration data management.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use anyhow::{anyhow, Result};
use log::{debug, info, trace, warn};
use std::fmt;
use std::fs::{read_to_string, write};
use std::path::Path;
use toml_edit::visit::{visit_table_like_kv, Visit};
use toml_edit::{Array, DocumentMut, InlineTable, Item, Key, Table, Value};

mod locator;

#[doc(inline)]
pub use locator::*;

/// Repository configuration file handler.
///
/// Handles the the parsing and manipulation of Ricer's repository configuration
/// file. Designed to serialize and deserialize repository configuration
/// information, while perserving the comments and formatting the user may
/// introduce into the configuration file.
///
/// # See also
///
/// - [`RepoEntry`]
/// - [`RepoBootstrapEntry`]
#[derive(Debug, Default, Clone)]
pub struct ReposConfig {
    doc: DocumentMut,
}

impl ReposConfig {
    /// Construct new repository configuration handler.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::ReposConfig;
    ///
    /// let config = ReposConfig::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Read from repository configuration file at provided path.
    ///
    /// # Errors
    ///
    /// Will fail if configuration file cannot be read, or contains invalid
    /// TOML formatting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::ReposConfig;
    ///
    /// let mut config = ReposConfig::new();
    /// config.read("/path/to/repos.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let buffer = read_to_string(path.as_ref())?;
        let doc: DocumentMut = buffer.parse()?;
        self.doc = doc;
        Ok(())
    }

    /// Write to repository configuraiton file at provided path.
    ///
    /// # Errors
    ///
    /// Will fail if writing to the configuration file fails for whatever
    /// reason.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::ReposConfig;
    ///
    /// let mut config = ReposConfig::new();
    /// config.write("/path/to/repos.toml");
    /// ```
    pub fn write(&mut self, path: impl AsRef<Path>) -> Result<()> {
        info!("Write to '{}'", path.as_ref().display());
        let buffer = self.doc.to_string();
        write(path.as_ref(), buffer)?;
        Ok(())
    }

    /// Serialize repository entry into parsed configuration file data.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section is not defined as a table.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{ReposConfig, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = ReposConfig::new();
    /// config.add_repo(&repo)?;
    /// let expect = indoc! {r#"
    /// [repos.vim]
    /// branch = "master"
    /// remote = "origin"
    /// workdir_home = true
    /// "#};
    /// let result = config.to_string();
    /// assert_eq!(result, expect);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_repo(&mut self, entry: &RepoEntry) -> Result<()> {
        info!("Add repository '{}' to configuration file", &entry.name);
        let (key, value) = entry.to_toml();
        if let Some(repos) = self.doc.get_mut("repos") {
            trace!("The 'repos' section exists, add to it");
            let repos = repos.as_table_mut().ok_or(anyhow!(
                "The 'repos' section in configuration file not defined as a table"
            ))?;
            repos.insert(key.get(), value);
        } else {
            trace!("The 'repos' section does not exist, set it up and add to it");
            let mut repos = Table::new();
            repos.insert(key.get(), value);
            repos.set_implicit(true);
            self.doc.insert("repos", Item::Table(repos));
        }
        Ok(())
    }

    /// Deserialize repository entry from parsed configuration file data.
    ///
    /// # Errors
    ///
    /// Will fail if there is not 'repos' section to obtain repository entries
    /// from. Will also fail if target repository does not exist in 'repos'
    /// section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use pretty_assertions::assert_eq;
    /// use indoc::indoc;
    /// use ricer::config::{ReposConfig, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = ReposConfig::new();
    /// config.add_repo(&repo)?;
    /// let result = config.get_repo("vim")?;
    /// assert_eq!(result, repo);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_repo(&self, name: impl AsRef<str>) -> Result<RepoEntry> {
        debug!("Get repository '{}' from configuration file", name.as_ref());
        let repos = self.get_section("repos")?;
        let repo = repos
            .get_key_value(name.as_ref())
            .ok_or(anyhow!("Repository '{}' does not exist in 'repos' section", name.as_ref()))?;
        Ok(RepoEntry::from(repo))
    }

    /// Delete repository entry definition from configuraiton file data.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, `repos` section is not
    /// defined as table, or target repository does not exist in the `repos`
    /// section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::{ReposConfig, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = ReposConfig::new();
    /// config.add_repo(&repo)?;
    /// let ret_result = config.delete_repo("vim")?;
    /// let str_result = config.to_string();
    /// let expect = "";
    /// assert_eq!(str_result, expect);
    /// assert_eq!(ret_result, repo);
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_repo(&mut self, name: impl AsRef<str>) -> Result<RepoEntry> {
        info!("Remove repository '{}' from configuration file", name.as_ref());
        let repos = self.get_section_mut("repos")?;
        match repos.remove_entry(name.as_ref()) {
            Some((key, value)) => Ok(RepoEntry::from((&key, &value))),
            None => Err(anyhow!("Repository '{}' does not exist in configuration file", name.as_ref())),
        }
    }

    /// Rename repository entry in configuration file data.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, `repos` section is not
    /// defined as a table, or target repository does not exist in the `repos`
    /// section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{ReposConfig, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = ReposConfig::new();
    /// config.add_repo(&repo)?;
    /// config.rename_repo("vim", "neovim")?;
    /// let result = config.to_string();
    /// let expect = indoc! {r#"
    /// [repos.neovim]
    /// branch = "master"
    /// remote = "origin"
    /// workdir_home = true
    /// "#};
    /// assert_eq!(result, expect);
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename_repo(&mut self, from: impl AsRef<str>, to: impl AsRef<str>) -> Result<()> {
        info!("Rename repository '{}' to '{}' in configuration file", from.as_ref(), to.as_ref());
        let repos = self.get_section_mut("repos")?;
        let (key, value) = repos.remove_entry(from.as_ref()).ok_or(anyhow!(
            "Repository '{}' does not exist in 'repos' section of configuratoin file",
            from.as_ref()
        ))?;

        // Preserve decor (comments and formatting) from original key...
        let key = Key::new(to.as_ref()).with_leaf_decor(key.leaf_decor().clone());
        repos.insert_formatted(&key, value);
        Ok(())
    }

    /// Get specific section of TOML document.
    ///
    /// # Errors
    ///
    /// Will fail if configuration file does not contain target section, or if
    /// target section is not defined as a table.
    fn get_section(&self, name: impl AsRef<str>) -> Result<&Table> {
        let repos = self
            .doc
            .get(name.as_ref())
            .ok_or(anyhow!("Configuration file does not define a '{}' section", name.as_ref()))?;
        let repos = repos.as_table().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            name.as_ref()
        ))?;

        Ok(repos)
    }

    /// Get specific mutable section of TOML document.
    ///
    /// # Errors
    ///
    /// Will fail if configuration file does not contain target section, or if
    /// target section is not defined as a table.
    fn get_section_mut(&mut self, name: impl AsRef<str>) -> Result<&mut Table> {
        let repos = self
            .doc
            .get_mut(name.as_ref())
            .ok_or(anyhow!("Configuration file does not define a '{}' section", name.as_ref()))?;
        let repos = repos.as_table_mut().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            name.as_ref()
        ))?;

        Ok(repos)
    }
}

impl fmt::Display for ReposConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

/// Repository entry definition.
///
/// Intermediary structure meant to help make it easier to deserialize and
/// serialize repository configuration data to and from Ricer's repository
/// configuration file.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoEntry {
    /// Name of repository.
    pub name: String,

    /// Default branch.
    pub branch: String,

    /// Default remote.
    pub remote: String,

    /// Flag to determine if repository's working directory is the user's home
    /// through _fake bare_ technique.
    pub workdir_home: bool,

    /// Bootstrap configuration for repository.
    pub bootstrap: Option<RepoBootstrapEntry>,
}

impl RepoEntry {
    /// Build new repository entry definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoEntry;
    ///
    /// let builder = RepoEntry::builder("vim");
    /// ```
    ///
    /// # See also
    ///
    /// - [`RepoEntryBuilder`]
    /// - [`RepoBootstrapEntryBuilder`]
    pub fn builder(name: impl Into<String>) -> RepoEntryBuilder {
        RepoEntryBuilder::new(name)
    }

    /// Serialize repository entry definition into a TOML item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{RepoEntry, RepoBootstrapEntry, OsType};
    /// use toml_edit::DocumentMut;
    ///
    /// let bootstrap = RepoBootstrapEntry::builder()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .os(OsType::Unix)
    ///     .users(["awkless", "sedgwick"])
    ///     .hosts(["lovelace", "turing"])
    ///     .build();
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .bootstrap(bootstrap)
    ///     .build();
    /// let (key, value) = repo.to_toml();
    ///
    /// let mut toml_doc: DocumentMut = "[repos]".parse()?;
    /// let repos_table = toml_doc.get_mut("repos").unwrap();
    /// let repos_table = repos_table.as_table_mut().unwrap();
    /// repos_table.insert(&key, value);
    /// repos_table.set_implicit(true);
    /// let expect = indoc! {r#"
    ///     [repos.vim]
    ///     branch = "master"
    ///     remote = "origin"
    ///     workdir_home = true
    ///
    ///     [repos.vim.bootstrap]
    ///     clone = "https://github.com/awkless/vim.git"
    ///     os = "unix"
    ///     users = ["awkless", "sedgwick"]
    ///     hosts = ["lovelace", "turing"]
    /// "#};
    /// let result = toml_doc.to_string();
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_toml(&self) -> (Key, Item) {
        let mut repo_entry = Table::new();
        let mut bootstrap_entry = Table::new();

        repo_entry.insert("branch", Item::Value(Value::from(&self.branch)));
        repo_entry.insert("remote", Item::Value(Value::from(&self.remote)));
        repo_entry.insert("workdir_home", Item::Value(Value::from(self.workdir_home)));
        if let Some(bootstrap) = &self.bootstrap {
            if let Some(clone) = &bootstrap.clone {
                bootstrap_entry.insert("clone", Item::Value(Value::from(clone)));
            }
            if let Some(os) = &bootstrap.os {
                bootstrap_entry.insert("os", Item::Value(Value::from(os.to_string())));
            }
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

        let bootstrap = bootstrap.build();
        if !bootstrap.is_empty() {
            repo = repo.bootstrap(bootstrap);
        }
        repo.build()
    }
}

/// Builder for [`RepoEntry`].
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoEntryBuilder {
    name: String,
    branch: String,
    remote: String,
    workdir_home: bool,
    bootstrap: Option<RepoBootstrapEntry>,
}

impl RepoEntryBuilder {
    /// Construct new repository entry definition builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoEntryBuilder;
    ///
    /// let builder = RepoEntryBuilder::new("vim");
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
    /// use ricer::config::RepoEntryBuilder;
    ///
    /// let builder = RepoEntryBuilder::new("vim").branch("master");
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
    /// use ricer::config::RepoEntryBuilder;
    ///
    /// let builder = RepoEntryBuilder::new("vim").remote("origin");
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
    /// use ricer::config::RepoEntryBuilder;
    ///
    /// let builder = RepoEntryBuilder::new("vim").workdir_home(true);
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
    /// use ricer::config::{RepoEntryBuilder, RepoBootstrapEntry, OsType};
    ///
    /// let bootstrap = RepoBootstrapEntry::builder()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .os(OsType::Unix)
    ///     .users(["awkless", "sedgwick"])
    ///     .hosts(["lovelace", "turing"])
    ///     .build();
    /// let builder = RepoEntryBuilder::new("vim").bootstrap(bootstrap);
    /// ```
    pub fn bootstrap(mut self, bootstrap: RepoBootstrapEntry) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }

    /// Build new [`RepoEntry`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoEntry, RepoBootstrapEntry, OsType};
    ///
    /// let bootstrap = RepoBootstrapEntry::builder()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .os(OsType::Unix)
    ///     .users(["awkless", "sedgwick"])
    ///     .hosts(["lovelace", "turing"])
    ///     .build();
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .bootstrap(bootstrap)
    ///     .build();
    /// ```
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

/// Repository bootstrap entry definition.
///
/// Specify bootstrapping options for a given repository entry definition. This
/// structure serves the same purpose as [`RepoEntry`], i.e., help make it easy
/// to serialize and deserialize repository configuraiton information to and
/// from Ricer's repository configuration file.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrapEntry {
    pub clone: Option<String>,
    pub os: Option<OsType>,
    pub users: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
}

impl RepoBootstrapEntry {
    /// Build a new repository bootstrap definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapEntry;
    ///
    /// let builder = RepoBootstrapEntry::builder();
    /// ```
    pub fn builder() -> RepoBootstrapEntryBuilder {
        RepoBootstrapEntryBuilder::new()
    }

    pub fn is_empty(&self) -> bool {
        self.clone.is_none() && self.os.is_none() && self.users.is_none() && self.hosts.is_none()
    }
}

/// Builder for [`RepoBootstrapEntry`].
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoBootstrapEntryBuilder {
    clone: Option<String>,
    os: Option<OsType>,
    users: Option<Vec<String>>,
    hosts: Option<Vec<String>>,
}

impl RepoBootstrapEntryBuilder {
    /// Construct new repository bootstrap definition builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapEntryBuilder;
    ///
    /// let builder = RepoBootstrapEntryBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set URL to clone repository from.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoBootstrapEntryBuilder;
    ///
    /// let builder = RepoBootstrapEntryBuilder::new().clone("url here");
    /// ```
    pub fn clone(mut self, url: impl Into<String>) -> Self {
        self.clone = Some(url.into());
        self
    }

    /// Set target OS to bootstrap repository for.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBootstrapEntryBuilder, OsType};
    ///
    /// let builder = RepoBootstrapEntryBuilder::new().os(OsType::Unix);
    /// ```
    ///
    /// # See also
    ///
    /// - [`OsType`]
    pub fn os(mut self, os: OsType) -> Self {
        self.os = Some(os);
        self
    }

    /// Set target users to bootstrap repository to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBootstrapEntryBuilder, OsType};
    ///
    /// let builder = RepoBootstrapEntryBuilder::new().users(["awkless", "knuth", "bob"]);
    /// ```
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

    /// Set target hosts to bootstrap repository for.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBootstrapEntryBuilder, OsType};
    ///
    /// let builder = RepoBootstrapEntryBuilder::new().hosts(["awkless", "knuth", "bob"]);
    /// ```
    pub fn hosts<I, S>(mut self, hosts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.hosts = Some(Vec::new());
        for host in hosts {
            self.hosts.get_or_insert(vec![]).push(host.into());
        }
        self
    }

    /// Build [`RepoBootstrapEntry`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{RepoBootstrapEntryBuilder, OsType};
    ///
    /// let bootstrap = RepoBootstrapEntryBuilder::new()
    ///     .clone("https://github.com/awkless/vim.git")
    ///     .os(OsType::Unix)
    ///     .users(["awkless", "knuth", "goodwill"])
    ///     .hosts(["lovelace", "dijkstra", "turing"])
    ///     .build();
    /// ```
    pub fn build(self) -> RepoBootstrapEntry {
        RepoBootstrapEntry { clone: self.clone, os: self.os, users: self.users, hosts: self.hosts }
    }
}

impl<'toml> Visit<'toml> for RepoBootstrapEntryBuilder {
    fn visit_table_like_kv(&mut self, key: &'toml str, node: &'toml Item) {
        match key {
            "clone" => self.clone = Some(node.as_str().unwrap_or_default().to_string()),
            "os" => self.os = Some(OsType::from(node.as_str().unwrap_or_default())),
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

/// Supported operating systems for bootstrapping.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum OsType {
    /// Bootstrap to any operating system.
    #[default]
    Any,

    /// Bootstrap to Unix-like system only.
    Unix,

    /// Bootstrap to MacOS system only.
    MacOs,

    /// Boostrap to Windows system only.
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
