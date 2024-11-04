// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::locate::LocateError;

use directories::ProjectDirs;
use log::trace;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

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
