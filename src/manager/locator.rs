// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use directories::ProjectDirs;
use log::trace;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

use crate::manager::LocatorError;

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
