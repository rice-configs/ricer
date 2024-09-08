// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

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
use log::{debug, trace, warn};
use std::fs::{create_dir_all, read_to_string, remove_dir_all, rename, write, File};
use std::path::{Path, PathBuf};

use crate::config::DirLocator;
use crate::error::RicerResult;

/// Configuration directory manager representation.
pub trait ConfigDirManager {
    /// Setup configuration file at `$XDG_CONFIG_HOME/ricer/config.toml`.
    fn setup_config_file(&self) -> RicerResult<PathBuf>;

    /// Add Git repository entry at `$XDG_CONFIG_HOME/ricer/repos`.
    fn add_repo(&self, name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Get path to Git repository entry at `$XDG_CONFIG_HOME/ricer/repos`.
    fn get_repo(&self, name: impl AsRef<str>) -> RicerResult<PathBuf>;

    /// Remove Git repository at `$XDG_CONFIG_HOME/ricer/repos`.
    fn remove_repo(&self, name: impl AsRef<str>) -> RicerResult<()>;

    /// Rename Git repository entry at `$XDG_CONFIG_HOME/ricer/repos`.
    fn rename_repo(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()>;

    /// Get contents of hook script at `$XDG_CONFIG_HOME/ricer/hooks`.
    fn get_cmd_hook(&self, name: impl AsRef<str>) -> RicerResult<String>;

    /// Write ignore file in `$XDG_CONFIG_HOME/ricer/ignores`.
    fn write_ignore_file(&self, name: impl AsRef<str>, data: impl AsRef<[u8]>) -> RicerResult<()>;

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
    /// use ricer::config::{DefaultDirLocator, XdgDirLayout, DefaultConfigDirManager};
    ///
    /// let layout = XdgDirLayout::new_layout()?;
    /// let locator = DefaultDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`ConfigDirLocator`]: crate::config::locator::ConfigDirLocator
    pub fn new(locator: &dyn DirLocator) -> Self {
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
        let file_path = self.root_dir.join("config.toml");
        if !self.root_dir().exists() {
            debug!("Create root directory of configuration at '{}'", self.root_dir.display());
            create_dir_all(self.root_dir())?;
        } else {
            trace!("Root directory already exists");
        }

        if !file_path.exists() {
            debug!("Create configuration file at '{}'", file_path.display());
            File::create_new(&file_path)?;
        } else {
            trace!("Coniguration file already exists");
        }

        debug_assert!(file_path.is_absolute(), "Configuration file path is not absolute");
        Ok(file_path)
    }

    /// Add Git repository entry into `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Postconditions
    ///
    /// 1. Create new Git repository in `$XDG_CONFIG_HOME/ricer/repos`.
    ///    - Skip Git repository creation if it already exists.
    /// 2. Return path to new Git repository.
    ///
    /// # Invariants
    ///
    /// 1. Returned path to Git repository is guaranteed to be absolute.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if Git repository entry could
    ///    not be created.
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
    /// let path = cfg_dir_mgr.add_repo("vim")?;
    /// println!("{}", path.display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn add_repo(&self, name: impl AsRef<str>) -> RicerResult<PathBuf> {
        trace!("Add new Git repository into '$XDG_CONFIG_HOME/ricer/repos'");
        let repo_path = self.repos_dir.join(format!("{}.git", name.as_ref()));
        if !repo_path.exists() {
            debug!("Create Git repository at '{}'", repo_path.display());
            create_dir_all(&repo_path)?;
        } else {
            warn!("Git repository already exists at '{}'", repo_path.display());
        }

        debug_assert!(repo_path.is_absolute(), "Path to Git repository is not absolute");
        Ok(repo_path)
    }

    /// Get path Git repository entry into `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Postconditions
    ///
    /// 1. Return path to target Git repository.
    ///
    /// # Invariants
    ///
    /// 1. Returned path to Git repository is guaranteed to be absolute.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if Git repository entry could
    ///    not be created.
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
    /// let path = cfg_dir_mgr.get_repo("vim")?;
    /// println!("{}", path.display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn get_repo(&self, name: impl AsRef<str>) -> RicerResult<PathBuf> {
        debug!("Get Git repository '{}'", name.as_ref());
        let repo_path = self.repos_dir.join(format!("{}.git", name.as_ref()));
        if !repo_path.exists() {
            return Err(anyhow!("Git repository '{}' does not exist", name.as_ref()).into());
        }

        debug_assert!(repo_path.is_absolute(), "Git repository path is not absolute");
        Ok(repo_path)
    }

    /// Remove Git repository entry from `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Postconditions
    ///
    /// 1. Remove Git repository from `$XDG_CONFIG_HOME/ricer/repos`.
    ///    - Will not fail if Git repository already does not exist.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if Git repository could not be
    ///    removed.
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
    /// cfg_dir_mgr.remove_repo("vim")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn remove_repo(&self, name: impl AsRef<str>) -> RicerResult<()> {
        trace!("Remove Git repository");
        let repo_path = self.repos_dir.join(format!("{}.git", name.as_ref()));
        if repo_path.exists() {
            debug!("Remove Git repository '{}' at '{}'", name.as_ref(), repo_path.display());
            remove_dir_all(&repo_path)?;
        } else {
            warn!("Git repository '{}' already removed", name.as_ref());
            return Ok(());
        }

        Ok(())
    }

    /// Rename Git repository entry from `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Preconditions
    ///
    /// 1. Repository directory entry exists in `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Postconditions
    ///
    /// 1. Rename Git repository entry in `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if Git repository could not be
    ///    renamed.
    /// 2. Return [`RicerError::Unrecoverable`] if repository does not exist in
    ///    `$XDG_CONFIG_HOME/ricer/repos`.
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
    /// cfg_dir_mgr.rename_repo("vi", "vim")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn rename_repo(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()> {
        trace!("Rename Git repository");
        let from_path = self.repos_dir.join(format!("{}.git", from.as_ref()));
        let to_path = self.repos_dir.join(format!("{}.git", to.as_ref()));
        if !from_path.exists() {
            return Err(anyhow!(
                "Cannot rename repository '{}', because it does not exist",
                from.as_ref()
            )
            .into());
        }

        debug!("Rename Git repository from '{}' to '{}'", from.as_ref(), to.as_ref());
        rename(&from_path, to_path)?;
        Ok(())
    }

    /// Get contents of hook script at `$XDG_CONFIG_HOME/ricer/hooks`.
    ///
    /// # Preconditions
    ///
    /// 1. Hook script exists in `$XDG_CONFIG_HOME/ricer/hooks`.
    ///
    /// # Postconditions
    ///
    /// 1. Obtain contents of hook script in string form.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if hook script does not exist
    ///    or cannot be read into a string for whatever reason.
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
    /// cfg_dir_mgr.rename_repo("vi", "vim")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn get_cmd_hook(&self, name: impl AsRef<str>) -> RicerResult<String> {
        debug!("Get hook '{}' contents", name.as_ref());
        let hook_path = self.hooks_dir.join(name.as_ref());
        let buffer = read_to_string(hook_path)?;
        Ok(buffer)
    }

    /// Write ignore file in `$XDG_CONFIG_HOME/ricer/ignores`.
    ///
    /// # Postconditions
    ///
    /// 1. Write data into target ignore file.
    ///     - Create `$XDG_CONFIG_HOME/ricer/ignores` if it does not exist.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if ignore file cannot be
    ///    created and/or written too.
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
    /// cfg_dir_mgr.write_ignore_file("vim", "hello world!")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn write_ignore_file(&self, name: impl AsRef<str>, data: impl AsRef<[u8]>) -> RicerResult<()> {
        debug!("Write ignore file for repository '{}'", name.as_ref());
        let ignore_path = self.ignores_dir.join(format!("{}.ignore", name.as_ref()));
        if !self.ignores_dir.exists() {
            debug!("Create ignores directory at '{}'", self.ignores_dir.display());
            create_dir_all(&self.ignores_dir)?;
        }

        write(ignore_path, data.as_ref())?;
        Ok(())
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
