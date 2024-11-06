// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Locate expected paths to configuration data on user's system.
//!
//! Provide reliable way to determine absolute paths to expected locations for
//! Ricer's configuration data on user's system. Location data is mainly
//! determined through a _directory layout_. A directory layout is an
//! abstraction that specifies an expected way Ricer's configuration directories
//! _should_ be setup on the user's filesystem. Currently, Ricer uses the [XDG
//! Base Directory Specification][xdg] to specify its standard directory layout
//! through [`XdgDirLayout`]. This means that Ricer expects the following
//! layout:
//!
//! - `$XDG_CONFIG_HOME/ricer` contains behavior data like configuration files
//!   and hook scripts.
//! - `$XDG_DATA_HOME/ricer` contains tracked Git
//!   repositories to manipulate.
//!
//! The [`DefaultLocator`] uses this directory layout information to properly
//! locate expected paths for various standard configuration files, Git
//! repositories, and hook scripts.
//!
//! [xdg]: https://specifications.freedesktop.org/basedir-spec/latest/

use directories::ProjectDirs;
use log::{debug, trace};
use std::path::{Path, PathBuf};

#[cfg(test)]
use mockall::automock;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum LocateError {
    #[error("Cannot determine path to home directory")]
    NoWayHome,
}

/// Configuration data locator.
#[cfg_attr(test, automock)]
pub trait Locator {
    /// Expected absolute path to configuration file directory.
    fn config_dir(&self) -> &Path;

    /// Expected absolute path to hook script directory.
    fn hooks_dir(&self) -> &Path;

    /// Expected absolute path to command hook configuration file.
    fn hooks_config(&self) -> &Path;

    /// Expected absolute path to repository directory.
    fn repos_dir(&self) -> &Path;

    /// Expected absolute path to repository configuration file.
    fn repos_config(&self) -> &Path;
}

/// Default configuration data locator.
///
/// # Invariants
///
/// 1. Caller must validate paths themselves.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DefaultLocator {
    config_dir: PathBuf,
    hooks_dir: PathBuf,
    hooks_config: PathBuf,
    repos_dir: PathBuf,
    repos_config: PathBuf,
}

impl DefaultLocator {
    pub fn locate(layout: impl DirLayout) -> Self {
        trace!("Construct configuration directory locator");
        let config_dir = layout.config_dir().to_path_buf();
        let hooks_dir = config_dir.join("hooks");
        let hooks_config = config_dir.join("hooks.toml");
        let repos_dir = layout.repo_dir().join("ricer");
        let repos_config = config_dir.join("repos.toml");

        debug!("Configuration directory located at '{}'", config_dir.display());
        debug!("Hook script directory located at '{}'", hooks_dir.display());
        debug!("Repository directory located at '{}'", repos_dir.display());
        debug!("Repository configuration file located at '{}'", repos_config.display());
        debug!("Hook configuration file located at '{}'", hooks_config.display());
        Self { config_dir, hooks_dir, hooks_config, repos_dir, repos_config }
    }
}

impl Locator for DefaultLocator {
    fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.as_path()
    }

    fn hooks_config(&self) -> &Path {
        self.hooks_config.as_path()
    }

    fn repos_dir(&self) -> &Path {
        self.repos_dir.as_path()
    }

    fn repos_config(&self) -> &Path {
        self.repos_config.as_path()
    }
}

/// Specify expected configuration directory layout.
#[cfg_attr(test, automock)]
pub trait DirLayout {
    /// Absolute path to directory where configuration files will be stored.
    fn config_dir(&self) -> &Path;

    /// Absolute path to directory where repository data will be stored.
    fn repo_dir(&self) -> &Path;
}

/// Configuration directory layout handler following [XDG Base Directory
/// Specification][xdg].
///
/// # Invariants
///
/// 1. Caller must validate paths themselves.
///
/// [xdg]: https://specifications.freedesktop.org/basedir-spec/latest/
pub struct XdgDirLayout {
    layout: ProjectDirs,
}

impl XdgDirLayout {
    pub fn layout() -> Result<Self, LocateError> {
        trace!("Construct XDG Base Directory Specification layout handler");
        let layout = ProjectDirs::from("com", "awkless", "ricer").ok_or(LocateError::NoWayHome)?;
        Ok(Self { layout })
    }
}

impl DirLayout for XdgDirLayout {
    fn config_dir(&self) -> &Path {
        self.layout.config_dir()
    }

    fn repo_dir(&self) -> &Path {
        self.layout.data_dir()
    }
}
