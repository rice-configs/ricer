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
use std::str::FromStr;
use toml_edit::visit::{visit_table_like_kv, Visit};
use toml_edit::{DocumentMut, Item, Key, Table};

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
    // TODO: Implement this...
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
    // TODO: Implement this...
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
