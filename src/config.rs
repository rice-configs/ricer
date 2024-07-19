// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration data management.
//!
//! This module is responsible for providing an interface for managing Ricer's
//! configuration data.
//!
//! Ricer uses `$XDG_CONFIG_HOME/ricer` to house all of its configuration
//! information. The following represents the overall structure of this
//! directory:
//!
//! ```markdown
//! $XDG_CONFIG_HOME/ricer/
//! |-- hooks/
//! |   |-- hook_script1.sh
//! |   |-- hook_script2.sh
//! |   `-- hook_scriptn.sh
//! |-- repos/
//! |   |-- config1.git/
//! |   |-- config2.git/
//! |   `-- confign.git/
//! |-- ignores/
//! |   |-- config1.ignore
//! |   |-- config2.ignore
//! |   `-- confign.ignore
//! `-- config.toml
//! ```
//!
//! The `config.toml` file is the main configuration file for Ricer. The user
//! can directly modify this configuration file through their preferred text
//! editor. However, the user can indirectly modify `config.toml` through
//! Ricer's command set, e.g., init and clone.
//!
//! The `hooks/` directory contains all scripts that the user can use as hooks
//! for Ricer's command set. Ricer _will_ only execute hooks stored in this
//! directory. Thus, the user can refer to hook scripts by name without the need
//! to provide an absolute or relative path, because Ricer will automatically
//! look in the `hooks/` directory for any hook scripts the user wants.
//!
//! > __NOTE:__ The limiting of hook scripts to the `hook/` directory is done
//! > as a very basic security measure. The hope is that the user can easily
//! > identify potentially dangerious hook scripts by centeralizing all scripts
//! > in one easily accessible location.
//!
//! The `repos/` directory contains all the cloned and initialized repositories
//! the user wants Ricer to keep track of. This directory is where Ricer finds
//! and modifies repository information the user passes in through the CLI.
//!
//! Finally, the `ignores/` directory houses exclude/ignore files that ensure
//! that each repository in `repos/` only tracks the portions of the user's
//! home directory that they are responsible for. This ensures that no
//! repository the user is tracking through Ricer attempts to track their
//! entire home directory.

use anyhow::anyhow;
use directories::ProjectDirs;
use log::{debug, trace};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub mod file;

use crate::error::RicerError;
use file::ConfigFile;

/// Ricer configuration manipulation.
///
/// Simple API that allows for the manipulation and management of Ricer's
/// configuration directory data.
#[derive(Debug)]
pub struct Config<D: ConfigDir> {
    /// Layout of Ricer's configuration directory.
    dir: D,

    /// Contents of configuration file.
    pub file: ConfigFile,
}

impl<D: ConfigDir> Config<D> {
    /// Create new Ricer configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_core::config::{ConfigDir, DefaultConfigDir, Config};
    ///
    /// let config_dir = DefaultConfigDir::try_new()
    ///     .expect("Failed to locate default configuration directory");
    /// let config = Config::new(config_dir);
    /// ```
    pub fn new(config_dir: D) -> Self {
        Self { dir: config_dir, file: ConfigFile::default() }
    }

    /// Read configuration file at the base directory.
    ///
    /// # Preconditions
    ///
    /// 1. Configuration file 'config.toml' needs to exist at `$XDG_CONFIG_HOME/ricer`, i.e.,
    ///    Ricer's base configuration directory.
    /// 2. Configuration file contains proper TOML formatting.
    ///
    /// # Postconditions
    ///
    /// 1. Read and deserialize configuration file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_core::config::{ConfigDir, DefaultConfigDir, Config};
    ///
    /// let config_dir = DefaultConfigDir::try_new()
    ///     .expect("Failed to locate defualt configuration directory");
    /// let mut config = Config::new(config_dir);
    /// config.try_read_config_file().expect("Failed to read configuration file");
    /// println!("{:#?}", config.file);
    /// ```
    pub fn try_to_read_config_file(&mut self) -> Result<(), RicerError> {
        let config_path = self.dir.base_dir().join("config.toml");

        debug!("Read configuration file at '{}'", &config_path.display());
        let buffer = read_to_string(&config_path).map_err(|error| {
            RicerError::ConfigError(anyhow!(
                "Failed to read '{}': {}",
                &config_path.display(),
                error
            ))
        })?;

        let config_file: ConfigFile = toml::from_str(&buffer).map_err(|error| {
            RicerError::ConfigError(anyhow!(
                "Failed to parse '{}': {}",
                &config_path.display(),
                error
            ))
        })?;

        self.file = config_file;
        Ok(())
    }

    /// Determine if a repository can be found in configuration directory by
    /// name.
    ///
    /// # Preconditions
    ///
    /// 1. Repository must exist in `repos/` directory.
    ///
    /// # Postconditions
    ///
    /// 1. Provide full path to repository, or error out if it does not exist.
    pub fn try_to_find_git_repo(&self, repo: impl AsRef<str>) -> Result<PathBuf, RicerError> {
        let repo_path = self.dir.repos_dir().join(format!("{}.git", repo.as_ref()).as_str());
        if !repo_path.exists() {
            return Err(RicerError::ConfigError(anyhow!(
                "Failed to find Git repository: '{}'",
                repo_path.display()
            )));
        }

        debug!("Found Git repository: '{}'", &repo_path.display());
        Ok(repo_path)
    }

    /// Determine if a hook script can be found in configuration directory by
    /// name.
    ///
    /// # Preconditions
    ///
    /// 1. Hook script must exist in `hooks/` directory.
    ///
    /// # Postconditions
    ///
    /// 1. Provide full path to hook script, or error out if it does not exist.
    pub fn try_to_find_hook_script(&self, hook: impl AsRef<str>) -> Result<PathBuf, RicerError> {
        let hook_path = self.dir.hooks_dir().join(hook.as_ref());
        if !hook_path.exists() {
            return Err(RicerError::ConfigError(anyhow!(
                "Failed to find hook script: '{}'",
                &hook_path.display()
            )));
        }

        debug!("Found hook script: '{}'", &hook_path.display());
        Ok(hook_path)
    }

    /// Determine if a ignore file can be found in configuration directory by
    /// repository name.
    ///
    /// Ignore files in Ricer are repository specific. Hence, why we need to
    /// search for them by the name of a given repository.
    ///
    /// # Preconditions
    ///
    /// 1. Ignore file must exist in `ignores/` directory.
    ///
    /// # Postconditions
    ///
    /// 1. Provide full path to ignore file, or error out if it does not exist.
    pub fn try_to_find_ignore_file(&self, repo: impl AsRef<str>) -> Result<PathBuf, RicerError> {
        let ignore_path = self.dir.ignores_dir().join(format!("{}.ignore", repo.as_ref()).as_str());
        if !ignore_path.exists() {
            return Err(RicerError::ConfigError(anyhow!(
                "Failed to find ignore/exclude file: '{}'",
                &ignore_path.display()
            )));
        }

        debug!("Found ignore/exclude file: '{}'", &ignore_path.display());
        Ok(ignore_path)
    }
}

/// Configuration directory representation.
///
/// Meant to represent the layout of Ricer's configuration directory. As a trait
/// we can easily define and modify the directory layout any time without
/// much hassle.
///
/// # Invariants
///
/// 1. Implementors of this trait should only provide _expected_ paths for each
///    trait method such that path verification should be left to [`Config`].
///
/// [`Config`]: crate::config::Config
pub trait ConfigDir {
    /// Get location of Ricer's base configuration directory.
    fn base_dir(&self) -> &Path;

    /// Get location of Ricer's hook script directory.
    fn hooks_dir(&self) -> &Path;

    /// Get location of Ricer's tracked repositories directory.
    fn repos_dir(&self) -> &Path;

    /// Get location of Ricer's set of ignore/exclude files directory.
    fn ignores_dir(&self) -> &Path;
}

/// Default configuration directory.
///
/// Meant to represent Ricer's default configuration directory in
/// `$XDG_CONFIG_HOME/ricer`. See [`crate::config`] for more details about the
/// layout of Ricer's configuration directory.
///
/// [`crate::config`]: crate::config
#[derive(Debug)]
pub struct DefaultConfigDir {
    // $XDG_CONFIG_HOME/ricer/
    base_dir: PathBuf,

    // $XDG_CONFIG_HOME/ricer/hooks/
    hooks_dir: PathBuf,

    // $XDG_CONFIG_HOME/ricer/repos/
    repos_dir: PathBuf,

    // $XDG_CONFIG_HOME/ricer/ignores/
    ignores_dir: PathBuf,
}

impl DefaultConfigDir {
    /// Try to create new instance of default configuration directory.
    ///
    /// For Linux and macOS:
    ///
    /// 1. Use `$HOME` if it is set and not empty.
    /// 2. If `$HOME` is not set or empty, then function `getpwuid_r` is used to
    ///    determine home directory of current user.
    /// 3. If `getpwuid_r` lacks an entry for the current user ID or the
    ///    home directory field is empty, _then_ error out.
    ///
    /// For Windows:
    ///
    /// 1. Retrieve user profile folder using `SHGetKnownFolderPath`.
    /// 2. If this fails, _then_ error out.
    ///
    /// # Postconditions
    ///
    /// 1. Obtain instance of default configuration directory, or an error
    ///    indicating that the _expected_ path could not be obtained from the
    ///    user's environment.
    ///
    /// # Invariants
    ///
    /// 1. Only determine if defualt configuration directory path is reachable
    ///    in current user environment, _not_ whether it actually __exists__.
    ///
    /// # See
    ///
    /// - <https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.new>
    pub fn try_new() -> Result<Self, RicerError> {
        trace!("Locate default path to configuration directory");
        let dir = match ProjectDirs::from("com", "awkless", "ricer") {
            Some(dir) => dir,
            None => {
                return Err(RicerError::ConfigError(anyhow!(
                    "Failed to locate default configuration direcotry"
                )))
            }
        };

        let base_dir = dir.config_dir().to_path_buf();
        let hooks_dir = dir.config_dir().join("hooks/");
        let repos_dir = dir.config_dir().join("repos/");
        let ignores_dir = dir.config_dir().join("ignores/");

        debug!("Path to configuration directory: '{}'", &base_dir.display());
        Ok(Self { base_dir, hooks_dir, repos_dir, ignores_dir })
    }
}

impl ConfigDir for DefaultConfigDir {
    fn base_dir(&self) -> &Path {
        self.base_dir.as_path()
    }

    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.as_path()
    }

    fn repos_dir(&self) -> &Path {
        self.repos_dir.as_path()
    }

    fn ignores_dir(&self) -> &Path {
        self.ignores_dir.as_path()
    }
}
