// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use directories::ProjectDirs;
use log::{debug, trace};
use std::path::{PathBuf, Path};

#[cfg(test)]
use mockall::automock;

use crate::manager::LocatorError;

/// Configuration directory locator.
///
/// Locates absolute paths to the three special directories that Ricer
/// relies on.
#[cfg_attr(test, automock)]
pub trait DirLocator {
    /// Expected absolute path to configuration file directory.
    fn config_dir(&self) -> &Path;

    /// Expected absolute path to hook script directory.
    fn hooks_dir(&self) -> &Path;

    /// Expected absolute path to repository directory.
    fn repos_dir(&self) -> &Path;
}

pub struct DefaultDirLocator {
    config_dir: PathBuf,
    hooks_dir: PathBuf,
    repos_dir: PathBuf,
}

impl DefaultDirLocator {
    pub fn new_locate(layout: &impl DirLayout) -> Self {
        trace!("Construct configuration directory locator");
        let config_dir = layout.behavior_dir().join("ricer");
        let hooks_dir = config_dir.join("hooks");
        let repos_dir = layout.repo_dir().join("ricer");

        debug!("Configuration directory located at '{}'", config_dir.display());
        debug!("Hook script directory located at '{}'", hooks_dir.display());
        debug!("Repository directory located at '{}'", repos_dir.display());
        Self { config_dir, hooks_dir, repos_dir }
    }
}

impl DirLocator for DefaultDirLocator {
    fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.as_path()
    }

    fn repos_dir(&self) -> &Path {
        self.repos_dir.as_path()
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
    fn behavior_dir(&self) -> &Path;

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
        let layout = ProjectDirs::from("com", "awkless", "ricer").ok_or_else(|| LocatorError::NoWayHome)?;
        Ok(Self { layout })
    }
}

impl DirLayout for XdgDirLayout {
    fn behavior_dir(&self) -> &Path {
        self.layout.config_dir()
    }

    fn repo_dir(&self) -> &Path {
        self.layout.data_dir()
    }
}
