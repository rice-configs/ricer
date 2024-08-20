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

use log::{debug, trace};
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};

use crate::config::locator::ConfigDirLocator;
use crate::error::{RicerError, RicerResult};

/// Configuration directory manager representation.
pub trait ConfigDirManager {
    /// Setup configuration file at expected location if needed.
    fn setup_config_file(&self) -> RicerResult<PathBuf>;

    /// Find absolute path to Git repository.
    fn git_repo_path(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Find absolute path to hook script.
    fn hook_script_path(&self, hook_name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Find absolute path to ignore file.
    fn ignore_file_path(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Absolute path to root/top-level configuration directory.
    fn root_dir(&self) -> &Path;

    /// Absolute path to repositories directory.
    fn repos_dir(&self) -> &Path;

    /// Absolute path to hook scripts directory.
    fn hooks_dir(&self) -> &Path;

    /// Absolute path to ignore files directory.
    fn ignores_dir(&self) -> &Path;
}

/// Default implementation of a configuration directory manager.
///
/// # Invariants
///
/// 1. Stored paths must be absolute.
pub struct DefaultConfigDirManager {
    root_dir: PathBuf,
    repos_dir: PathBuf,
    hooks_dir: PathBuf,
    ignores_dir: PathBuf,
}

impl DefaultConfigDirManager {
    /// Construct new default configuration directory manager.
    ///
    /// # Preconditions
    ///
    /// 1. Provide valid instance of [`ConfigDirLocator`].
    ///
    /// # Postconditions
    ///
    /// 1. Return valid [`DefaultConfigDirManager`] instance.
    ///
    /// # Invariants
    ///
    /// 1. All stored paths must be absolute.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::DefaultConfigDirManager;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`ConfigDirLocator`]: crate::config::locator::ConfigDirLocator
    pub fn new(locator: &dyn ConfigDirLocator) -> Self {
        trace!("Construct default configuration directory manager");
        let root_dir = locator.config_dir().to_path_buf();
        let repos_dir = root_dir.join("repos");
        let hooks_dir = root_dir.join("hooks");
        let ignores_dir = root_dir.join("ignores");

        debug_assert!(root_dir.is_absolute(), "Root directory path is not absolute");
        debug_assert!(repos_dir.is_absolute(), "The 'repos' directory path is not absolute");
        debug_assert!(hooks_dir.is_absolute(), "The 'hooks' directory path is not absolute");
        debug_assert!(ignores_dir.is_absolute(), "The 'ignores' directory path is not absolute");

        Self { root_dir, repos_dir, hooks_dir, ignores_dir }
    }
}

impl ConfigDirManager for DefaultConfigDirManager {
    /// Setup configuration file at `$XDG_CONFIG_HOME/ricer/config.toml`.
    ///
    /// # Postconditions
    ///
    /// 1. Create new empty configuration file at `$XDG_CONFIG_HOME/ricer/config.toml`
    ///    if it does not exist.
    ///    - Will also create all sub-directories in `$XDG_CONFIG_HOME` if they
    ///      do not already exist.
    /// 2. Return absolute path to configuration file.
    ///
    /// # Invariants
    ///
    /// 1. Returned path to configuration file is guaranteed to be absolute.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if sub-directories to
    ///    configuration file or configuration file itself could not be created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let path = cfg_dir_mgr.setup_config_file()?;
    /// println!("{}", path.display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn setup_config_file(&self) -> RicerResult<PathBuf> {
        trace!("Setup configuration file at '$XDG_CONFIG_HOME/ricer/config.toml'");
        let file_path  = self.root_dir.join("config.toml");
        if !self.root_dir().exists() {
            debug!("Create root directory of configuration at '{}'", self.root_dir.display());
            create_dir_all(self.root_dir())?;
        }

        if !file_path.exists() {
            debug!("Create configuration file at '{}'", file_path.display());
            File::create_new(&file_path)?;
        }

        debug_assert!(file_path.is_absolute(), "Configuration file path is not absolute");
        Ok(file_path)
    }

    /// Get path to Git repository.
    ///
    /// # Preconditions
    ///
    /// 1. Git repository exists in `$XDG_CONFIG_HOME/ricer/repos` directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to Git repository.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    ///
    /// # Errors
    ///
    /// 1. Returns `RicerError::NoGitRepo` if Git repository does not exist
    ///    in `$XDG_CONFIG_HOME/ricer/repos` directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let repo_path = cfg_dir_mgr.git_repo_path("vim")?;
    /// println!("{}", repo_path.display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::NoGitRepo`]: crate::error::RicerError::NoGitRepo
    fn git_repo_path(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf> {
        let repo_path = self.repos_dir.join(format!("{}.git", repo_name.as_ref()));
        debug_assert!(repo_path.is_absolute(), "Git repository path is not absolute");
        if !repo_path.exists() {
            return Err(RicerError::NoGitRepo { path: repo_path });
        }

        Ok(repo_path)
    }

    /// Get path to hook script.
    ///
    /// # Preconditions
    ///
    /// 1. Hook script exists in `$XDG_CONFIG_HOME/ricer/hooks` directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to hook script.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    ///
    /// # Errors
    ///
    /// 1. Returns `RicerError::NoHookScript` if hook script does not exist
    ///    in `$XDG_CONFIG_HOME/ricer/hooks` directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let repo_path = cfg_dir_mgr.hook_script_path("vim")?;
    /// println!("{}", repo_path.display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::NoHookScript`]: crate::error::RicerError::NoHookScript
    fn hook_script_path(&self, hook_name: impl AsRef<str>) -> RicerResult<PathBuf> {
        let hook_path = self.hooks_dir.join(hook_name.as_ref());
        debug_assert!(hook_path.is_absolute(), "Hook script path is not absolute");
        if !hook_path.exists() {
            return Err(RicerError::NoHookScript { path: hook_path });
        }

        Ok(hook_path)
    }

    /// Get path to ignore file.
    ///
    /// # Preconditions
    ///
    /// 1. Ignore file exists in `$XDG_CONFIG_HOME/ricer/ignores` directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to ignore file.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    ///
    /// # Errors
    ///
    /// 1. Returns `RicerError::NoIgnoreFile` if ignore file does not exist
    ///    in `$XDG_CONFIG_HOME/ricer/ignores` directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::config::dir::{ConfigDirManager, DefaultConfigDirManager};
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let ignore_path = cfg_dir_mgr.ignore_file_path("vim")?;
    /// println!("{}", ignore_path.display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::NoIgnoreFile`]: crate::error::RicerError::NoIgnoreFile
    fn ignore_file_path(&self, repo_name: impl AsRef<str>) -> RicerResult<PathBuf> {
        let ignore_path = self.ignores_dir.join(format!("{}.ignore", repo_name.as_ref()));
        debug_assert!(ignore_path.is_absolute(), "Ignore file path is not absolute");
        if !ignore_path.exists() {
            return Err(RicerError::NoIgnoreFile { path: ignore_path });
        }

        Ok(ignore_path)
    }

    /// Get path to root of configuration directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to root of configuration directory.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    fn root_dir(&self) -> &Path {
        self.root_dir.as_path()
    }

    /// Get path to `repos` sub-directory in configuration directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to `repos` sub-directory.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    fn repos_dir(&self) -> &Path {
        self.repos_dir.as_path()
    }

    /// Get path to `hooks` sub-directory in configuration directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to `hooks` sub-directory.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.as_path()
    }

    /// Get path to `ignores` sub-directory in configuration directory.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to `ignores` sub-directory.
    ///
    /// # Invariants
    ///
    /// 1. Path returned is guaranteed to be absolute.
    fn ignores_dir(&self) -> &Path {
        self.ignores_dir.as_path()
    }
}
