// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

mod error;
mod locator;

#[doc(inline)]
pub use error::*;
pub use locator::*;

use crate::config::{CommandHook, Repository, Toml, TomlError};

use log::trace;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Configuration file construct.
///
/// Manage configuration file data by selecting which configuration
/// startegy to use, i.e., which configuration category to handle.
/// Expected section to serialize and deserialize will depend on the
/// configuration strategy selected by caller.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Config`]
/// - [`Toml`]
#[derive(Clone, Debug, Default)]
pub struct ConfigManager<T, D>
where
    T: TomlManager,
    D: DirLocator,
{
    doc: Toml,
    locator: D,
    config: T,
}

impl<T, D> ConfigManager<T, D>
where
    T: TomlManager,
    D: DirLocator,
{
    pub fn new(config: T, locator: D) -> Self {
        trace!("Construct new configuration manager");
        Self { doc: Toml::new(), locator, config }
    }

    pub fn load(&mut self) -> Result<(), ConfigManagerError> {
        let path = self.location();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|err| ConfigManagerError::FileOpen { source: err, path: path.clone() })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| ConfigManagerError::FileRead { source: err, path: path.clone() })?;
        self.doc = buffer
            .parse()
            .map_err(|err| ConfigManagerError::Toml { source: err, path: path.clone() })?;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), ConfigManagerError> {
        let path = self.location();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|err| ConfigManagerError::FileOpen { source: err, path: path.clone() })?;
        let buffer = self.doc.to_string();
        file.write_all(buffer.as_bytes())
            .map_err(|err| ConfigManagerError::FileWrite { source: err, path: path.clone() })?;
        Ok(())
    }

    pub fn get(&self, key: impl AsRef<str>) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .get(&self.doc, key.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.location() })
    }

    pub fn add(&mut self, entry: T::Entry) -> Result<Option<T::Entry>, ConfigManagerError> {
        self.config
            .add(&mut self.doc, entry)
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.location() })
    }

    pub fn rename(
        &mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .rename(&mut self.doc, from.as_ref(), to.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.location() })
    }

    pub fn remove(&mut self, key: impl AsRef<str>) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .remove(&mut self.doc, key.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.location() })
    }

    pub fn location(&self) -> PathBuf {
        self.config.location(&self.locator)
    }
}

impl<T, D> fmt::Display for ConfigManager<T, D>
where
    T: TomlManager,
    D: DirLocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

/// TOML serialization and deserialization manager.
///
/// Interface to simplify serialization and deserialization of parsed TOML data.
///
/// # See also
///
/// - [`Toml`]
pub trait TomlManager {
    type Entry;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError>;
    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError>;
    fn location<D>(&self, locator: &D) -> PathBuf
    where
        D: DirLocator;
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
/// - [`Repository`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RepositoryData;

impl TomlManager for RepositoryData {
    type Entry = Repository;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.get("repos", key.as_ref())?;
        Ok(Repository::from(entry))
    }

    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError> {
        let entry = doc.add("repos", entry.to_toml())?.map(Repository::from);
        Ok(entry)
    }

    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.remove("repos", key.as_ref())?;
        Ok(Repository::from(entry))
    }

    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.rename("repos", from.as_ref(), to.as_ref())?;
        Ok(Repository::from(entry))
    }

    fn location<D>(&self, locator: &D) -> PathBuf
    where
        D: DirLocator,
    {
        locator.config_dir().join("repos.toml")
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
/// - [`CommandHook`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandHookData;

impl TomlManager for CommandHookData {
    type Entry = CommandHook;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.get("hooks", key.as_ref())?;
        Ok(CommandHook::from(entry))
    }

    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError> {
        let entry = doc.add("hooks", entry.to_toml())?.map(CommandHook::from);
        Ok(entry)
    }

    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.remove("hooks", key.as_ref())?;
        Ok(CommandHook::from(entry))
    }

    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.rename("hooks", from.as_ref(), to.as_ref())?;
        Ok(CommandHook::from(entry))
    }

    fn location<D>(&self, locator: &D) -> PathBuf
    where
        D: DirLocator,
    {
        locator.config_dir().join("hooks.toml")
    }
}
