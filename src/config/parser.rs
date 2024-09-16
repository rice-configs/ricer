// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use log::{info, trace};
use std::fmt;
use std::fs::{read_to_string, write};
use std::path::Path;
use toml_edit::{DocumentMut, Item, Key, Table};

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
    /// Will fail if path to file does not exist, or contains invalid TOML
    /// formatting.
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

    /// Rename entry in TOML file.
    ///
    /// Provides old entry before it was renamed.
    ///
    /// # Errors
    ///
    /// Will fail if target section does not exist, or is not defined as a
    /// table. Will also fail if target entry does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{TomlParser, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut toml = TomlParser::new();
    /// toml.add_entry("repos", repo.to_toml())?;
    /// toml.rename_entry("repos", "vim", "neovim")?;
    ///
    /// let expect = indoc! {r#"
    ///     [repos.neovim]
    ///     branch = "main"
    ///     remote = "origin"
    ///     workdir_home = true
    /// "#};
    /// let result = toml.to_string();
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename_entry<S>(&mut self, section: S, from: S, to: S) -> Result<(Key, Item)>
    where
        S: AsRef<str>,
    {
        let table = self
            .doc
            .get_mut(section.as_ref())
            .ok_or(anyhow!("Configuration file does not contain '{}' section", section.as_ref()))?;
        let table = table.as_table_mut().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            section.as_ref()
        ))?;
        let (old_key, old_item) = table.remove_entry(from.as_ref()).ok_or(anyhow!(
            "Configuration file does not define '{}' in '{}' section to remove",
            section.as_ref(),
            from.as_ref()
        ))?;
        let new_key = Key::new(to.as_ref()).with_leaf_decor(old_key.leaf_decor().clone());
        table.insert_formatted(&new_key, old_item.clone());
        Ok((old_key, old_item))
    }
}

impl fmt::Display for TomlParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}
