// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration data management.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use anyhow::{anyhow, Result};
use log::{info, trace};
use std::fmt;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Key, Table};

mod hook;
mod repo;

#[doc(inline)]
pub use hook::*;
pub use repo::*;

/// Format preserving TOML parser.
#[derive(Clone, Debug, Default)]
pub struct TomlParser {
    doc: DocumentMut,
}

impl TomlParser {
    /// Construct new TOML parser.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::TomlParser;
    ///
    /// let toml = TomlParser::new();
    /// ```
    pub fn new() -> Self {
        trace!("Construct new TOML file parser");
        Default::default()
    }

    /// Read a TOML file.
    ///
    /// # Errors
    ///
    /// Will fail if path to TOML file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::TomlParser;
    ///
    /// let mut toml = TomlParser::new();
    /// toml.read("/path/to/file.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&mut self, path: impl AsRef<Path>) -> Result<()> {
        info!("Read configuration file '{}'", path.as_ref().display());
        let buffer = read_to_string(path.as_ref())?;
        let doc: DocumentMut = buffer.parse()?;
        self.doc = doc;
        Ok(())
    }

    /// Write to a TOML file.
    ///
    /// # Errors
    ///
    /// Will fail if path to TOML file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::TomlParser;
    ///
    /// let mut toml = TomlParser::new();
    /// toml.write("/path/to/file.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write(&mut self, path: impl AsRef<Path>) -> Result<()> {
        info!("Write configuration file '{}'", path.as_ref().display());
        let buffer = self.doc.to_string();
        write(path.as_ref(), buffer)?;
        Ok(())
    }

    /// Get entry from TOML file.
    ///
    /// # Errors
    ///
    /// Will fail if target section does not exist, or was not defined as a
    /// table. Will also fail if target entry does not exist in target section.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::TomlParser;
    ///
    /// let mut toml = TomlParser::new();
    /// toml.read("/path/to/file.toml")?;
    /// let entry = toml.get_entry("foo", "bar")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_entry(
        &self,
        section: impl AsRef<str>,
        key: impl AsRef<str>,
    ) -> Result<(&Key, &Item)> {
        info!("Get entry '{}' from '{}' section", key.as_ref(), section.as_ref());
        let table = self
            .doc
            .get(section.as_ref())
            .ok_or(anyhow!("Configuration file does not contain '{}' section", section.as_ref()))?;
        let table = table.as_table().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            section.as_ref()
        ))?;
        let entry = table.get_key_value(key.as_ref()).ok_or(anyhow!(
            "Configuration file does not have '{}' in '{}' section",
            section.as_ref(),
            key.as_ref()
        ))?;
        Ok(entry)
    }

    /// Add entry into TOML file.
    ///
    /// Inserts target section if it does not exist in TOML file, and inserts
    /// the target entry in that new section. Otherwise, will just insert the
    /// target entry if target section already exists.
    ///
    /// Will return previous entry data if provided entry replaces an existing
    /// entry in the TOML file. Otherwise, will return `None` if provided entry
    /// was a new addition to the TOML file.
    ///
    /// # Errors
    ///
    /// Will fail if existing section is not defined as a table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::{TomlParser, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut toml = TomlParser::new();
    /// toml.add_entry("repos", repo.to_toml())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_entry(
        &mut self,
        section: impl AsRef<str>,
        entry: (Key, Item),
    ) -> Result<Option<(Key, Item)>> {
        let (key, value) = entry;
        info!("Add entry '{}' to '{}' section", key.get(), section.as_ref());
        let old_key = key.clone();
        let old_entry = if let Some(table) = self.doc.get_mut(section.as_ref()) {
            let table = table.as_table_mut().ok_or(anyhow!(
                "Configuruation file does not define section '{}' as a table",
                section.as_ref()
            ))?;
            table.insert(key.get(), value)
        } else {
            let mut table = Table::new();
            table.insert(key.get(), value);
            table.set_implicit(true);
            self.doc.insert(section.as_ref(), Item::Table(table))
        }
        .map(|old_item| (old_key, old_item));
        Ok(old_entry)
    }

    /// Remove entry from TOML file.
    ///
    /// # Errors
    ///
    /// Will fail if target section does not exist, or is not defined as a
    /// table. Will also fail if target entry does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::TomlParser;
    ///
    /// let mut toml = TomlParser::new();
    /// toml.read("/path/to/file.toml")?;
    /// let entry = toml.remove_entry("foo", "bar")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_entry(
        &mut self,
        section: impl AsRef<str>,
        key: impl AsRef<str>,
    ) -> Result<(Key, Item)> {
        info!("Remove entry '{}' from '{}' section", key.as_ref(), section.as_ref());
        let table = self
            .doc
            .get_mut(section.as_ref())
            .ok_or(anyhow!("Configuration file does not contain '{}' section", section.as_ref()))?;
        let table = table.as_table_mut().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            section.as_ref()
        ))?;
        let entry = table.remove_entry(key.as_ref()).ok_or(anyhow!(
            "Configuration file does not define '{}' in '{}' section to remove",
            section.as_ref(),
            key.as_ref()
        ))?;
        Ok(entry)
    }
}

impl fmt::Display for TomlParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

/// Manage repository configuration file.
#[derive(Clone, Debug, Default)]
pub struct RepoConfig {
    path: PathBuf,
    toml: TomlParser,
}

impl RepoConfig {
    /// Construct new repository configuration file manager
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoConfig;
    ///
    /// let config = RepoConfig::new("/path/to/repos.toml");
    /// ```
    pub fn new(path: impl Into<PathBuf>) -> Self {
        trace!("Construct new repository configuration file manager");
        Self { path: path.into(), toml: TomlParser::new() }
    }

    /// Read repository configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if path to repository configuration file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::RepoConfig;
    ///
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.read()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&mut self) -> Result<()> {
        self.toml.read(&self.path)?;
        Ok(())
    }

    /// Write repository configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if path to repository configuration file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::RepoConfig;
    ///
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.write()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write(&mut self) -> Result<()> {
        self.toml.write(&self.path)?;
        Ok(())
    }

    /// Add repository definition entry into configuration file.
    ///
    /// Inserts repository entry into `repos` section. If the `repos` section
    /// does not exist, then a new `repos` section will be created with the
    /// repository entry inserted.
    ///
    /// Will return previous repository entry if target repository entry
    /// replaced it in the configuration file. Otherwise, will return `None`
    /// if provided repository entry was a new addition to configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if existing `repos` section was not defined as a table.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(repo)?;
    /// let expect = indoc! {r#"
    /// [repos.vim]
    /// branch = "main"
    /// remote = "origin"
    /// workdir_home = true
    /// "#};
    /// let result = config.to_string();
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_repo(&mut self, entry: RepoEntry) -> Result<Option<RepoEntry>> {
        let (key, item) = entry.to_toml();
        let old_entry = self
            .toml
            .add_entry("repos", (key, item))?
            .map(|(key, item)| RepoEntry::from((&key, &item)));
        Ok(old_entry)
    }

    /// Get repository entry from configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, or `repos` section was not
    /// defined as a table. Will also fail if repository entry does not exist
    /// in `repos` section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let expect = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(expect.clone())?;
    /// let result = config.get_repo("vim")?;
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_repo(&self, name: impl AsRef<str>) -> Result<RepoEntry> {
        let entry = self.toml.get_entry("repos", name.as_ref())?;
        Ok(RepoEntry::from(entry))
    }

    /// Remove repository entry from configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, or `repos` section was not
    /// defined as a table. Will also fail if repository entry does not exist
    /// in `repos` section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let expect = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(expect.clone())?;
    /// let result = config.remove_repo("vim")?;
    /// assert_eq!(expect, result);
    /// assert_eq!("", config.to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_repo(&mut self, name: impl AsRef<str>) -> Result<RepoEntry> {
        let (key, item) = self.toml.remove_entry("repos", name.as_ref())?;
        Ok(RepoEntry::from((&key, &item)))
    }

    /// Rename repository entry in configuration file.
    ///
    /// # Examples
    ///
    /// TODO
    pub fn rename_repo<S>(&mut self, from: S, to: S) -> Result<RepoEntry>
    where
        S: AsRef<str>,
    {
        let (key, item) = self.toml.remove_entry("repos", from.as_ref())?;
        let mut repo = RepoEntry::from((&key, &item));
        repo.name = to.as_ref().into();
        self.toml.add_entry("repos", repo.to_toml())?;
        repo.name = from.as_ref().into();
        Ok(repo)
    }
}

impl fmt::Display for RepoConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toml)
    }
}
