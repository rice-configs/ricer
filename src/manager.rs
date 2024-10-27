// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

mod error;
mod locator;

#[doc(inline)]
pub use error::*;
pub use locator::*;

use crate::config::{Toml, CommandHook, Repository, TomlError};

use log::trace;

#[cfg(test)]
use mockall::automock;

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
    toml: Toml,
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
        Self {
            toml: Toml::new(),
            locator,
            config,
        }
    }
}

/// TOML serialization and deserialization manager.
///
/// Interface to simplify serialization and deserialization of parsed TOML data.
///
/// # See also
///
/// - [`Toml`]
#[cfg_attr(test, automock(type Entry = Repository;))]
pub trait TomlManager {
    type Entry;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError>;
    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError>;
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
}
