// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration directory and file manager and implementation.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use log::warn;

pub mod dir;
pub mod file;
pub mod locator;

use crate::error::{RicerError, RicerResult};
use dir::ConfigDirManager;
use file::ConfigFileManager;

pub struct ConfigManager<D: ConfigDirManager, F: ConfigFileManager> {
    dir_manager: D,
    file_manager: F,
}

impl<D: ConfigDirManager, F: ConfigFileManager> ConfigManager<D, F> {
    /// Construct new configuration manager.
    ///
    /// # Preconditions
    ///
    /// 1. Valid [`ConfigDirManager`] instance.
    /// 2. Valid [`ConfigFileManager`] instance.
    ///
    /// # Postconditions
    ///
    /// 1. Return new configuration manager instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    /// use ricer::config::file::DefaultConfigFileManager;
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::ConfigManager;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_manager = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_manager = DefaultConfigFileManager::new();
    /// let config = ConfigManager::new(cfg_dir_manager, cfg_file_manager);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn new(dir_manager: D, file_manager: F) -> Self {
        Self { dir_manager, file_manager }
    }

    /// Get current configuration directory manager.
    ///
    /// # Postconditions
    ///
    /// 1. Immutable reference to [`ConfigDirManager`] instance.
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    pub fn dir_manager(&self) -> &D {
        &self.dir_manager
    }

    /// Get current mutable configuration directory manager.
    ///
    /// # Postconditions
    ///
    /// 1. Mutable reference to [`ConfigDirManager`] instance.
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    pub fn dir_manager_mut(&mut self) -> &mut D {
        &mut self.dir_manager
    }

    /// Get current configuration file manager.
    ///
    /// # Postconditions
    ///
    /// 1. Immutable reference to [`ConfigFileManager`].
    ///
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn file_manager(&self) -> &F {
        &self.file_manager
    }

    /// Get current mutable configuration file manager.
    ///
    /// # Postconditions
    ///
    /// 1. Mutable reference to [`ConfigFileManager`].
    ///
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn file_manager_mut(&mut self) -> &mut F {
        &mut self.file_manager
    }

    /// Read configuration file.
    ///
    /// # Preconditions
    ///
    /// 1. Configuration file exists at `$XDG_CONFIG_HOME/ricer/config.toml`.
    /// 2. Configuration file contains valid TOML formatting.
    ///
    /// # Postconditions
    ///
    /// 1. Read and parse configuration file for later manipulation.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::NoConfigFile`] if configuration file does not
    ///    exist.
    /// 2. Return [`RicerError::Unrecoverable`] if configuration file contains
    ///    invalid TOML formatting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    /// use ricer::config::file::DefaultConfigFileManager;
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::ConfigManager;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_manager = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_manager = DefaultConfigFileManager::new();
    /// let mut config = ConfigManager::new(cfg_dir_manager, cfg_file_manager);
    /// config.read_config_file()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::NoConfigFile`]: crate::error::RicerError::NoConfigFile
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    pub fn read_config_file(&mut self) -> RicerResult<()> {
        let path = self.dir_manager.config_file_path()?;
        self.file_manager.read(path)?;
        Ok(())
    }

    /// Write configuration file.
    ///
    /// # Preconditions
    ///
    /// 1. Full path to configuration file exists, i.e., no sub-directories are
    ///    _not_ missing.
    ///
    /// # Postconditions
    ///
    /// 1. If file does not exist, but all sub-directories do exist, then create
    ///    it and write to it.
    /// 2. Preserve original formatting and comments that existed before
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if sub-directories in provided
    ///    path do not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    /// use ricer::config::file::DefaultConfigFileManager;
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::ConfigManager;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_manager = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_manager = DefaultConfigFileManager::new();
    /// let mut config = ConfigManager::new(cfg_dir_manager, cfg_file_manager);
    /// config.write_config_file()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    pub fn write_config_file(&mut self) -> RicerResult<()> {
        let path = match self.dir_manager.config_file_path() {
            Ok(path) => path,
            Err(RicerError::NoConfigFile { path }) => {
                warn!("Create non-existant configuration file at '{}'", path.display());
                path
            }
            Err(err) => return Err(err),
        };

        self.file_manager.write(path)?;
        Ok(())
    }
}
