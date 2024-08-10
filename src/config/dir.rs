// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration directory layout management.
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
use std::path::{Path, PathBuf};

use crate::config::locator::ConfigDirLocator;
use crate::error::{RicerError, RicerResult};

/// Configuration directory manager representation.
pub trait ConfigDirManager {
    /// Get absolute path to configuration file.
    fn config_file_path(&self) -> RicerResult<PathBuf>;

    /// Find absolute path to Git repository.
    fn find_git_repo(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Find absolute path to hook script.
    fn find_hook_script(&self, hook_name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Find absolute path to ignore file.
    fn find_ignore_file(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Absolute path to root/top-level configuration directory.
    fn root_dir(&self) -> &Path;

    /// Absolute path to repositories directory.
    fn repos_dir(&self) -> &Path;

    /// Absolute path to hook scripts directory.
    fn hooks_dir(&self) -> &Path;

    /// Absolute path to ignore files directory.
    fn ignores_dir(&self) -> &Path;
}

pub struct DefaultConfigDirManager {
    root_dir: PathBuf,
    repos_dir: PathBuf,
    hooks_dir: PathBuf,
    ignores_dir: PathBuf,
}

impl DefaultConfigDirManager {
    /// Construct new default configuration directory manager.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::DefaultConfigDirManager;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::try_new()?;
    /// let locator = DefaultConfigDirLocator::try_new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(locator: &dyn ConfigDirLocator) -> Self {
        let root_dir = locator.config_dir().to_path_buf();
        let repos_dir = root_dir.join("repos");
        let hooks_dir = root_dir.join("hooks");
        let ignores_dir = root_dir.join("ignores");

        Self { root_dir, repos_dir, hooks_dir, ignores_dir }
    }
}

impl ConfigDirManager for DefaultConfigDirManager {
    /// Get path to configuration file.
    ///
    /// # Preconditions
    ///
    /// 1. Configuration file exists at `$XDG_CONFIG_HOME/ricer/config.toml`.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to configuration file.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// # Errors
    ///
    /// 1. Returns `RicerError::Unrecoverable` if configuration file does not
    ///    exist at `$XDG_CONFIG_HOME/ricer/config.toml`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::DefaultConfigDirManager;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::try_new()?;
    /// let locator = DefaultConfigDirLocator::try_new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// println!("{}", cfg_dir_mgr.config_file_path().display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn config_file_path(&self) -> RicerResult<PathBuf> {
        let cfg_file_path = self.root_dir.join("config.toml");
        debug_assert!(cfg_file_path.is_absolute(), "Configuration file path is not absolute");
        if !cfg_file_path.exists() {
            return Err(RicerError::Unrecoverable(anyhow!(
                "Configuration file does not exist at '{}'",
                cfg_file_path.display()
            )));
        }

        Ok(cfg_file_path)
    }

    fn find_git_repo(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf> {
        let repo_path = self.repos_dir.join(format!("{}.git", repo_name.as_ref()));
        if !repo_path.exists() {
            return Err(RicerError::Unrecoverable(anyhow!(
                "Git repository '{}' does not exist",
                repo_path.display()
            )));
        }

        Ok(repo_path)
    }

    fn find_hook_script(&self, hook_name: impl AsRef<str>) -> RicerResult<PathBuf> {
        let hook_path = self.hooks_dir.join(hook_name.as_ref());
        if !hook_path.exists() {
            return Err(RicerError::Unrecoverable(anyhow!(
                "Hook script '{}' does not exist",
                hook_path.display()
            )));
        }

        Ok(hook_path)
    }

    fn find_ignore_file(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf> {
        let ignore_path = self.ignores_dir.join(format!("{}.ignore", repo_name.as_ref()));
        if !ignore_path.exists() {
            return Err(RicerError::Unrecoverable(anyhow!(
                "Ignore file '{}' does not exist",
                ignore_path.display()
            )));
        }

        Ok(ignore_path)
    }

    fn root_dir(&self) -> &Path {
        self.root_dir.as_path()
    }

    fn repos_dir(&self) -> &Path {
        self.repos_dir.as_path()
    }

    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.as_path()
    }

    fn ignores_dir(&self) -> &Path {
        self.ignores_dir.as_path()
    }
}
