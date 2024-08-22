// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration directory and file manager and implementation.
//!
//! This module is responsible for providing a reliable way to manipulate
//! configuration data housed in Ricer's configuration directory. This includes
//! tracked repositories, hook scripts, ignore files, and configuration files.

use std::path::PathBuf;

pub mod dir;
pub mod file;
pub mod locator;

use crate::error::RicerResult;
use dir::ConfigDirManager;
use file::hooks_section::CommandHookEntry;
use file::repos_section::RepoEntry;
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

    /// Read configuration file.
    ///
    /// # Preconditions
    ///
    /// 1. Configuration file contains valid TOML formatting.
    ///
    /// # Postconditions
    ///
    /// 1. Read and parse configuration file for later manipulation.
    ///     - Will create empty configuration file if it does not exist.
    ///
    /// # Errors
    ///
    /// 1. Will fail if configuration file contains invalid TOML formatting.
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
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn read_config_file(&mut self) -> RicerResult<()> {
        let path = self.dir_manager.setup_config_file()?;
        self.file_manager.read(path)?;
        Ok(())
    }

    /// Write configuration file.
    ///
    /// # Postconditions
    ///
    /// 1. If file does not exist, but all sub-directories do exist, then create
    ///    it and write to it.
    /// 2. Preserve original formatting and comments that existed before
    ///
    /// # Errors
    ///
    /// 1. May fail if it cannot write and/or create the configuration file
    ///    for whatever reason.
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
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn write_config_file(&mut self) -> RicerResult<()> {
        let path = self.dir_manager.setup_config_file()?;
        self.file_manager.write(path)?;
        Ok(())
    }

    /// Get config file manager data in string form.
    ///
    /// # Postconditions
    ///
    /// 1. Get valid string representation of parsed configuration file data
    ///    provided by  current configuraiton file manager.
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
    /// println!("{}", config.file_manager_to_string());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn file_manager_to_string(&self) -> String {
        self.file_manager.to_string()
    }

    /// Add new repository into configuration data.
    ///
    /// # Postconditions
    ///
    /// 1. Create new repository directory in `$XDG_CONFIG_HOME/ricer/repos`.
    ///     - Create sub-directories in path if needed.
    /// 2. Add repository entry data into configuration file.
    ///     - Preserve original formatting of configuration file that existed
    ///       beforehand.
    ///
    /// # Errors
    ///
    /// 1. Will fail if `repos` section was not defined as a table.
    /// 2. Will fail if it cannot create the repository in
    ///    `$XDG_CONFIG_HOME/ricer/repos` for whatever reason.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    /// use ricer::config::file::repos_section::RepoEntry;
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
    /// let new_repo = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("master")
    ///     .url("https://github.com/awkless/vim.git")
    ///     .build();
    /// config.add_repo(&new_repo)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn add_repo(&mut self, repo_entry: &RepoEntry) -> RicerResult<()> {
        self.dir_manager.add_repo(&repo_entry.name)?;
        self.file_manager.add_repo(repo_entry)?;
        Ok(())
    }

    /// Get repository data from configuration file and `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Preconditions
    ///
    /// 1. Repository exists in configuration file.
    /// 2. Repository exists in `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to target repository.
    /// 2. Return configuration file entry data of repository.
    ///
    /// # Errors
    ///
    /// 1. Will fail if repository entry does not exist in configuration file.
    /// 2. Will fail if repository entry does not exist in
    ///    `$XDG_CONFIG_HOME/ricer/repos`.
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
    /// let (path, repo) = config.get_repo("vim")?;
    /// println!("{} - {:#?}", path.display(), repo);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn get_repo(&self, name: impl AsRef<str>) -> RicerResult<(PathBuf, RepoEntry)> {
        let path = self.dir_manager.get_repo(name.as_ref())?;
        let repo = self.file_manager.get_repo(name.as_ref())?;
        Ok((path, repo))
    }

    /// Remove Git repository from configuration data.
    ///
    /// # Postconditions
    ///
    /// 1. Remove Git repository directory entry from `$XDG_CONFIG_HOME/ricer/repos`.
    /// 2. Remove configuration file repository entry.
    ///
    /// # Errors
    ///
    /// 1. Will fail if repository entry in `$XDG_CONFIG_HOME/ricer/repos`
    ///    cannot be deleted.
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
    /// config.remove_repo("vim")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn remove_repo(&mut self, repo_name: impl AsRef<str>) -> RicerResult<()> {
        self.dir_manager.remove_repo(repo_name.as_ref())?;
        self.file_manager.remove_repo(repo_name.as_ref())?;
        Ok(())
    }

    /// Rename repository in configuration data.
    ///
    /// # Postconditions
    ///
    /// 1. Rename repository entry in `$XDG_CONFIG_HOME/ricer/repos`.
    /// 2. Rename repository entry in configuration file.
    ///
    /// # Errors
    ///
    /// 1. Will fail if repository entry cannot be renamed for whatever reason.
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
    /// config.rename_repo("vim", "vimrc")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn rename_repo(&mut self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()> {
        self.dir_manager.rename_repo(from.as_ref(), to.as_ref())?;
        self.file_manager.rename_repo(from.as_ref(), to.as_ref())?;
        Ok(())
    }

    /// Get command hook entry from configuration data.
    ///
    /// # Postconditions
    ///
    /// 1. Return data from hook script.
    /// 2. Return entry data from configuration file.
    ///
    /// # Errors
    ///
    /// 1. Will fail if command hook entry does not exist in configuration file.
    /// 2. Will fail if hook
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
    /// let hook = config.get_cmd_hook("commit")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn get_cmd_hook(&self, name: impl AsRef<str>) -> RicerResult<(CommandHookEntry, String)> {
        let entry = self.file_manager.get_cmd_hook(name.as_ref())?;
        let data = self.dir_manager.get_cmd_hook(name.as_ref())?;
        Ok((entry, data))
    }

    /// Write ignore file in `$XDG_CONFIG_HOME/ricer/ignores`.
    ///
    /// # Postconditions
    ///
    /// 1. Write given data into target ignore file in
    ///    `$XDG_CONFIG_HOME/ricer/ignores`.
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
    /// config.write_ignore_file("vim", "data goes here!")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    /// - [`ConfigFileManager`]
    ///
    /// [`ConfigDirManager`]: crate::config::dir::ConfigDirManager
    /// [`ConfigFileManager`]: crate::config::file::ConfigFileManager
    pub fn write_ignore_file(
        &self,
        repo: impl AsRef<str>,
        data: impl AsRef<[u8]>,
    ) -> RicerResult<()> {
        self.dir_manager.write_ignore_file(repo.as_ref(), data.as_ref())?;
        Ok(())
    }
}
