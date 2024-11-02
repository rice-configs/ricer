// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::manager::LocatorError;

use directories::ProjectDirs;
use log::{debug, trace};
use std::path::{Path, PathBuf};

#[cfg(test)]
use mockall::automock;

/// Configuration directory locator.
///
/// Locates absolute paths to the three special directories that Ricer
/// relies on.
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
        let config_dir = layout.config_dir().join("ricer");
        let hooks_dir = config_dir.join("hooks");
        let hooks_config = config_dir.join("hooks.toml");
        let repos_dir = layout.repo_dir().join("ricer");
        let repos_config = config_dir.join("repos.toml");

        debug!("Configuration directory located at '{}'", config_dir.display());
        debug!("Hook script directory located at '{}'", hooks_dir.display());
        debug!("Repository directory located at '{}'", repos_dir.display());
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

/// Handle different configuration directory layouts.
///
/// At a high level of abstraction, Ricer mainly splits its configuration
/// directories into two categories: behavior data, and repository data.
/// The behavior data category houses all files that configure Ricer's
/// behavior, while the repository data category contains repositories that
/// Ricer needs to keep track of and manipulate.
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
/// [xdg]: https://specifications.freedesktop.org/basedir-spec/latest/
pub struct XdgDirLayout {
    layout: ProjectDirs,
}

impl XdgDirLayout {
    pub fn layout() -> Result<Self, LocatorError> {
        trace!("Construct XDG Base Directory Specification layout handler");
        let layout = ProjectDirs::from("com", "awkless", "ricer").ok_or(LocatorError::NoWayHome)?;
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
