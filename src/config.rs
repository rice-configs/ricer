// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration directory and file manager and implementation.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

pub mod dir;
pub mod file;
pub mod locator;

use crate::error::RicerResult;
use dir::ConfigDirManager;
use file::ConfigFileManager;

pub struct ConfigManager<D: ConfigDirManager, F: ConfigFileManager> {
    dir_mgr: D,
    file_mgr: F,
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
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_mgr = DefaultConfigFileManager::new();
    /// let config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn new(dir_mgr: D, file_mgr: F) -> Self {
        Self { dir_mgr, file_mgr }
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
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_mgr = DefaultConfigFileManager::new();
    /// let mut config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    /// config.read_config_file()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_config_file(&mut self) -> RicerResult<()> {
        let path = self.dir_mgr.config_file_path()?;
        self.file_mgr.read(path)?;
        Ok(())
    }
}
