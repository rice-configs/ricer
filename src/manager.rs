// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Data manager implementations.
//!
//! This module is responsible for providing the logic needed for Ricer to
//! manage configuration, hook, and repository data provided by the user.

mod error;
mod locator;
mod toml;

#[doc(inline)]
pub use error::*;
pub use locator::*;
pub use toml::*;

use crate::config::Toml;
use crate::wizard::PagerPrompt;
use crate::context::Context;

use log::debug;
use mkdirp::mkdirp;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

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
/// - [`Toml`]
#[derive(Clone, Default, Debug)]
pub struct ConfigManager<'cfg, L, T>
where
    L: Locator,
    &'cfg L: Default,
    T: TomlManager,
{
    doc: Toml,
    config: T,
    locator: &'cfg L,
}

impl<'cfg, L, T> ConfigManager<'cfg, L, T>
where
    L: Locator,
    &'cfg L: Default,
    T: TomlManager,
{
    /// Load new configuration manager.
    ///
    /// If path to configuration file does not exist, then it will be created at
    /// target location. Otherwise, configuration file will be read and parsed
    /// like normal.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileRead`] if target configuration file
    ///    could not be read.
    /// 1. Return [`ConfigManagerError::Toml`] if target configuration file
    ///    could not be parsed into TOML format.
    pub fn load(config: T, locator: &'cfg L) -> Result<Self, ConfigManagerError> {
        let path = config.location(locator);
        debug!("Load new configuration manager from '{}'", path.display());
        let root = path.parent().unwrap();
        mkdirp(root)
            .map_err(|err| ConfigManagerError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|err| ConfigManagerError::FileOpen { source: err, path: path.into() })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| ConfigManagerError::FileRead { source: err, path: path.into() })?;
        let doc: Toml = buffer
            .parse()
            .map_err(|err| ConfigManagerError::Toml { source: err, path: path.into() })?;

        Ok(Self { doc, config, locator })
    }

    /// Save configuration data at expected location.
    ///
    /// If expected configuration file does not exist at location, then it will
    /// be created and written into automatically.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileWrite`] if target configuration file
    ///    cannot be written into.
    pub fn save(&mut self) -> Result<(), ConfigManagerError> {
        debug!("Save configuration manager data to '{}'", self.as_path().display());
        let root = self.as_path().parent().unwrap();
        mkdirp(root)
            .map_err(|err| ConfigManagerError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(self.as_path())
            .map_err(|err| ConfigManagerError::FileOpen {
                source: err,
                path: self.as_path().into(),
            })?;
        let buffer = self.doc.to_string();
        file.write_all(buffer.as_bytes()).map_err(|err| ConfigManagerError::FileWrite {
            source: err,
            path: self.as_path().into(),
        })?;

        Ok(())
    }

    /// Get configuration entry in deserialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be deserialized.
    pub fn get(&self, key: impl AsRef<str>) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .get(&self.doc, key.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    /// Add new configuration entry in serialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be serialized.
    pub fn add(&mut self, entry: T::Entry) -> Result<Option<T::Entry>, ConfigManagerError> {
        self.config
            .add(&mut self.doc, entry)
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    /// Rename configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be renamed.
    pub fn rename(
        &mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .rename(&mut self.doc, from.as_ref(), to.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    /// Remove configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be removed.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .remove(&mut self.doc, key.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    pub fn as_path(&self) -> &Path {
        self.config.location(self.locator)
    }
}

impl<'cfg, L, T> fmt::Display for ConfigManager<'cfg, L, T>
where
    L: Locator,
    &'cfg L: Default,
    T: TomlManager,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

#[derive(Debug)]
pub struct CommandHookManager<'cfg, L, P>
where
    L: Locator,
    &'cfg L: Default,
    P: PagerPrompt,
{
    context: &'cfg Context,
    locator: &'cfg L,
    config: ConfigManager<'cfg, L, CommandHookData>,
    pager: P,
}

impl<'cfg, L, P> CommandHookManager<'cfg, L, P>
where
    L: Locator,
    &'cfg L: Default,
    P: PagerPrompt,
{
    // TODO: implement stuffs...
}
