// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration file management.
//!
//! Manage Ricer's special configuration files by providing ways to perform
//! parsing, serialization, and deserialization, while preserving the original
//! formatting of said configuration files. Ricer uses the [TOML file
//! format][toml-spec] as the main data exchange format for configuration file
//! data. Thus, all logic in this module is centered around TOML.
//!
//! Ricer currently is expected to manage two types of configuration file:
//! repository, and hook configurations. These configuration files are mainly
//! located at whatever path is expected from any [`Locator`] implementation.
//! Currently, expected location for these configuration files is in the
//! `$XDG_CONFIG_HOME/ricer` directory.
//!
//! [toml-spec]: https://toml.io/en/v1.0.0
//!
//! # See also
//!
//! - [`XdgDirLayout`]
//! - [`DefaultLocator`]
//!
//! [`XdgDirLayout`]: crate::locate::XdgDirLayout
//! [`DefaultLocator`]: crate::locate::DefaultLocator

mod settings;

#[doc(inline)]
pub use settings::*;

use crate::locate::Locator;

use log::{debug, info, trace};
use mkdirp::mkdirp;
use std::{
    fmt,
    fs::OpenOptions,
    io,
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use toml_edit::{DocumentMut, Item, Key, Table};

/// Error types for [`ConfigFile`].
#[derive(Debug, thiserror::Error)]
pub enum ConfigFileError {
    #[error("Failed to make parent directory '{path}'")]
    MakeDirP { source: io::Error, path: PathBuf },

    #[error("Failed to open '{path}'")]
    FileOpen { source: io::Error, path: PathBuf },

    #[error("Failed to read '{path}'")]
    FileRead { source: io::Error, path: PathBuf },

    #[error("Failed to write '{path}'")]
    FileWrite { source: io::Error, path: PathBuf },

    #[error("Failed to parse '{path}'")]
    Toml { source: TomlError, path: PathBuf },
}

/// Error types for [`Toml`].
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum TomlError {
    #[error("Failed to parse TOML data")]
    BadParse { source: toml_edit::TomlError },

    #[error("TOML table '{table}' not found")]
    TableNotFound { table: String },

    #[error("TOML table '{table}' not defined as a table")]
    NotTable { table: String },

    #[error("TOML entry '{key}' not found in table '{table}'")]
    EntryNotFound { table: String, key: String },
}

/// Format preserving configuration file handler.
///
/// Manage configuration file data by selecting which configuration startegy to
/// use, i.e., which configuration file type to handle. Currently, there exists
/// two configuration file types: repository, and command hook. Once caller has
/// selected configuration file type to use, the [`Locator`] they pass in will
/// determine the expected path of the configuration file.
///
/// The configuration file will be opened if it exists at the expected path
/// assigned by the [`Locator`]. However, if the configuration file does not
/// exist, then it will be created at the expected path instead. This includes
/// the parent directory if needed.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
/// - [`RepoConfig`]
/// - [`CmdHookConfig`]
/// - [`DefaultLocator`]
///
/// [`DefaultLocator`]: crate::locate::DefaultLocator
#[derive(Clone, Debug)]
pub struct ConfigFile<'cfg, C, L>
where
    C: Config,
    L: Locator,
{
    doc: Toml,
    config: C,
    locator: &'cfg L,
}

impl<'cfg, C, L> ConfigFile<'cfg, C, L>
where
    C: Config,
    L: Locator,
{
    /// Load new configuration manager.
    ///
    /// If path to configuration file does not exist, then it will be created at
    /// target location. Otherwise, configuration file will be read and parsed
    /// like normal.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigFileError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigFileError::FileRead`] if target configuration file
    ///    could not be read.
    /// 1. Return [`ConfigFileError::Toml`] if target configuration file
    ///    could not be parsed into TOML format.
    pub fn load(config: C, locator: &'cfg L) -> Result<Self, ConfigFileError> {
        let path = config.location(locator);
        debug!("Load new configuration manager from '{}'", path.display());
        let root = path.parent().unwrap();
        mkdirp(root).map_err(|err| ConfigFileError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|err| ConfigFileError::FileOpen { source: err, path: path.into() })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| ConfigFileError::FileRead { source: err, path: path.into() })?;
        let doc: Toml = buffer
            .parse()
            .map_err(|err| ConfigFileError::Toml { source: err, path: path.into() })?;

        Ok(Self { doc, config, locator })
    }

    /// Save configuration data at expected location.
    ///
    /// If expected configuration file does not exist at location, then it will
    /// be created and written into automatically.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigFileError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigFileError::FileWrite`] if target configuration file
    ///    cannot be written into.
    pub fn save(&mut self) -> Result<(), ConfigFileError> {
        debug!("Save configuration manager data to '{}'", self.as_path().display());
        let root = self.as_path().parent().unwrap();
        mkdirp(root).map_err(|err| ConfigFileError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(self.as_path())
            .map_err(|err| ConfigFileError::FileOpen {
                source: err,
                path: self.as_path().into(),
            })?;
        let buffer = self.doc.to_string();
        file.write_all(buffer.as_bytes()).map_err(|err| ConfigFileError::FileWrite {
            source: err,
            path: self.as_path().into(),
        })?;

        Ok(())
    }

    /// Get configuration entry in deserialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be deserialized.
    pub fn get(&self, key: impl AsRef<str>) -> Result<C::Entry, ConfigFileError> {
        self.config
            .get(&self.doc, key.as_ref())
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    /// Add new configuration entry in serialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be serialized.
    pub fn add(&mut self, entry: C::Entry) -> Result<Option<C::Entry>, ConfigFileError> {
        self.config
            .add(&mut self.doc, entry)
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    /// Rename configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be renamed.
    pub fn rename(
        &mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<C::Entry, ConfigFileError> {
        self.config
            .rename(&mut self.doc, from.as_ref(), to.as_ref())
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    /// Remove configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be removed.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Result<C::Entry, ConfigFileError> {
        self.config
            .remove(&mut self.doc, key.as_ref())
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    pub fn as_path(&self) -> &Path {
        self.config.location(self.locator)
    }
}

impl<'cfg, C, L> fmt::Display for ConfigFile<'cfg, C, L>
where
    C: Config,
    L: Locator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

/// TOML parser.
///
/// Offers basic CRUD interface for TOML parsing. Expects TOML data in string
/// form. Leaves file handling to caller. Mainly operates on whole tables for
/// key-value pair manipulation. Note, that `document` is terminology used to
/// refer to parsed TOML data.
///
/// # Invariants
///
/// 1. Preserve original formatting of document.
///
/// # See also
///
/// - [`ConfigFile`]
#[derive(Clone, Default, Debug)]
pub struct Toml {
    doc: DocumentMut,
}

impl Toml {
    pub fn new() -> Self {
        trace!("Construct new TOML parser");
        Self { doc: DocumentMut::new() }
    }

    /// Add TOML entry into document.
    ///
    /// Will add given `entry` into target `table`. If `table` does not exist, then it
    /// will be created and `entry` will be inserted into it.
    ///
    /// Will replace any entries that match the key in `entry`, returning the
    /// old entry that was replaced. If no replacement took place, then `None`
    /// is returned instead.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    pub fn add(
        &mut self,
        table: impl AsRef<str>,
        entry: (Key, Item),
    ) -> Result<Option<(Key, Item)>, TomlError> {
        let (key, value) = entry;
        info!("Add TOML entry '{}' to '{}' table", key.get(), table.as_ref());
        let entry = match self.get_table_mut(table.as_ref()) {
            Ok(table) => table,
            Err(TomlError::TableNotFound { .. }) => {
                let mut new_table = Table::new();
                new_table.set_implicit(true);
                self.doc.insert(table.as_ref(), Item::Table(new_table));
                self.doc[table.as_ref()].as_table_mut().unwrap()
            }
            Err(err) => return Err(err),
        };
        let entry = entry.insert(key.get(), value).map(|old| (key, old));
        Ok(entry)
    }

    /// Get entry from target table in document.
    ///
    /// Return reference to full key-value pair in document.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    /// - Return [`TomlError::EntryNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    /// [`TomlError::EntryNotFound`]: crate::config::TomlError::EntryNotFound
    pub fn get<S>(&self, table: S, key: S) -> Result<(&Key, &Item), TomlError>
    where
        S: AsRef<str>,
    {
        info!("Get TOML entry '{}' from '{}' table", key.as_ref(), table.as_ref());
        let entry = self.get_table(table.as_ref())?;
        let entry = entry.get_key_value(key.as_ref()).ok_or_else(|| TomlError::EntryNotFound {
            table: table.as_ref().into(),
            key: key.as_ref().into(),
        })?;
        Ok(entry)
    }

    /// Rename TOML entry from document.
    ///
    /// Rename entry from target `table`. Returns old unrenamed entry.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    /// - Return [`TomlError::EntryNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    /// [`TomlError::EntryNotFound`]: crate::config::TomlError::EntryNotFound
    pub fn rename<S>(&mut self, table: S, from: S, to: S) -> Result<(Key, Item), TomlError>
    where
        S: AsRef<str>,
    {
        let entry = self.get_table_mut(table.as_ref())?;
        let (old_key, old_item) = entry.remove_entry(from.as_ref()).ok_or_else(|| {
            TomlError::EntryNotFound { table: table.as_ref().into(), key: from.as_ref().into() }
        })?;

        // INVARIANT: preserve original formatting that existed beforehand.
        let new_key = Key::new(to.as_ref()).with_leaf_decor(old_key.leaf_decor().clone());
        entry.insert_formatted(&new_key, old_item.clone());

        Ok((old_key, old_item))
    }

    /// Remove TOML entry from document.
    ///
    /// Remove `key` from target `table`. Returns removed entry.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    /// - Return [`TomlError::EntryNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    /// [`TomlError::EntryNotFound`]: crate::config::TomlError::EntryNotFound
    pub fn remove<S>(&mut self, table: S, key: S) -> Result<(Key, Item), TomlError>
    where
        S: AsRef<str>,
    {
        let entry = self.get_table_mut(table.as_ref())?;
        let entry = entry.remove_entry(key.as_ref()).ok_or_else(|| TomlError::EntryNotFound {
            table: table.as_ref().into(),
            key: key.as_ref().into(),
        })?;
        Ok(entry)
    }

    /// Get target table in document.
    ///
    /// Return reference to target table in document.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    pub(crate) fn get_table(&self, key: &str) -> Result<&Table, TomlError> {
        debug!("Get TOML table '{key}'");
        let table =
            self.doc.get(key).ok_or_else(|| TomlError::TableNotFound { table: key.into() })?;
        let table = table.as_table().ok_or_else(|| TomlError::NotTable { table: key.into() })?;
        Ok(table)
    }

    /// Get mutable target table in document.
    ///
    /// Return mutable reference to target table in document.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    pub(crate) fn get_table_mut(&mut self, key: &str) -> Result<&mut Table, TomlError> {
        debug!("Get mutable TOML table '{key}'");
        let table =
            self.doc.get_mut(key).ok_or_else(|| TomlError::TableNotFound { table: key.into() })?;
        let table =
            table.as_table_mut().ok_or_else(|| TomlError::NotTable { table: key.into() })?;
        Ok(table)
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

impl FromStr for Toml {
    type Err = TomlError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let doc: DocumentMut = data.parse().map_err(|err| TomlError::BadParse { source: err })?;
        Ok(Self { doc })
    }
}

/// TOML serialization and deserialization configuration.
///
/// Interface to simplify serialization and deserialization of parsed TOML data.
///
/// # See also
///
/// - [`Toml`]
pub trait Config: fmt::Debug {
    type Entry: Settings;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError>;
    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError>;
    fn location<'cfg>(&self, locator: &'cfg impl Locator) -> &'cfg Path;
}

/// Repository data configuration management.
///
/// Handles serialization and deserialization of repository settings.
/// Repository settings are held within the "repos" section of a
/// configuration file.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
/// - [`RepoSettings`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RepoConfig;

impl Config for RepoConfig {
    type Entry = RepoSettings;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.get("repos", key.as_ref())?;
        Ok(RepoSettings::from(entry))
    }

    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError> {
        let entry = doc.add("repos", entry.to_toml())?.map(RepoSettings::from);
        Ok(entry)
    }

    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.remove("repos", key.as_ref())?;
        Ok(RepoSettings::from(entry))
    }

    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.rename("repos", from.as_ref(), to.as_ref())?;
        Ok(RepoSettings::from(entry))
    }

    fn location<'cfg>(&self, locator: &'cfg impl Locator) -> &'cfg Path {
        locator.repos_config()
    }
}

/// Command hook configuration management.
///
/// Handles serialization and deserialization of command hook settings.
/// Command hook settings are held within the "hooks" section of a
/// configuration file.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
/// - [`CmdHookSettings`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CmdHookConfig;

impl Config for CmdHookConfig {
    type Entry = CmdHookSettings;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.get("hooks", key.as_ref())?;
        Ok(CmdHookSettings::from(entry))
    }

    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError> {
        let entry = doc.add("hooks", entry.to_toml())?.map(CmdHookSettings::from);
        Ok(entry)
    }

    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.remove("hooks", key.as_ref())?;
        Ok(CmdHookSettings::from(entry))
    }

    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.rename("hooks", from.as_ref(), to.as_ref())?;
        Ok(CmdHookSettings::from(entry))
    }

    fn location<'cfg>(&self, locator: &'cfg impl Locator) -> &'cfg Path {
        locator.hooks_config()
    }
}

#[cfg(test)]
mod tests {}
