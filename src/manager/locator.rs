// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Locate configuration directories.
//!
//! Ricer uses three configuration directories:
//!
//! 1. Configuration file directory housing configuration file data at
//!    `$XDG_CONFIG_HOME/ricer`.
//! 2. Hook script directory housing executable hook scripts at
//!    `$XDG_CONFIG_HOME/ricer/hooks`.
//! 3. Repository directory housing tracked repository data at
//!    `XDG_DATA_HOME/ricer`.
//!
//! This modules determines absolute paths to all three of Ricer's configuration
//! directories for later manipulation in the codebase. This module only
//! determines the _expected_ locations for each configuration directory,
//! __not__ whether any one of them actually exists. The job of existence
//! verification is left to the various configuration manager implementations in
//! Ricer's codebase.

use log::{debug, trace};
use std::path::{Path, PathBuf};

#[cfg(test)]
use mockall::automock;

mod layout;

#[doc(inline)]
pub use layout::*;

/// Configuration directory locator representation.
#[cfg_attr(test, automock)]
pub trait DirLocator {
    /// Expected absolute path to configuration file directory.
    fn config_dir(&self) -> &Path;

    /// Expected absolute path to hook script directory.
    fn hooks_dir(&self) -> &Path;

    /// Expected absolute path to repository directory.
    fn repos_dir(&self) -> &Path;
}

/// Default configuration directory locator.
pub struct DefaultDirLocator {
    config_dir: PathBuf,
    hooks_dir: PathBuf,
    repos_dir: PathBuf,
}

impl DefaultDirLocator {
    /// Construct default configuration directory locator.
    ///
    /// Will locate _expected_ absolute paths to Ricer's configuration
    /// directory, hook script directory, and repository directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use std::env;
    /// use std::path::PathBuf;
    ///
    /// use ricer::manager::{DirLocator, DefaultDirLocator, XdgDirLayout};
    ///
    /// let home = env::var("HOME")?;
    /// let layout = XdgDirLayout::new_layout()?;
    /// let locator = DefaultDirLocator::new_locate(&layout);
    /// assert_eq!(locator.config_dir(), PathBuf::from(format!("{}/ricer", home)).as_path());
    /// assert_eq!(locator.hooks_dir(), PathBuf::from(format!("{}/ricer/hooks", home)).as_path());
    /// assert_eq!(locator.repos_dir(), PathBuf::from(format!("{}/ricer", home)).as_path());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_locate(layout: &impl DirLayout) -> Self {
        trace!("Construct configuration directory locator");
        let config_dir = layout.config_dir().join("ricer");
        let hooks_dir = config_dir.join("hooks");
        let repos_dir = layout.data_dir().join("ricer_repos");

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
