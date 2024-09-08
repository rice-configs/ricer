// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Handle different configuration directory layouts.
//!
//! At a high level of abstraction, Ricer mainly splits its configuration
//! directories into two categories: configuration file, and configuration data.
//! The configuration file category houses all files that configure Ricer's
//! behavior, while the configuration data category contains data that Ricer
//! needs to keep track of and manipulate.
//!
//! This module provides a standard way to organize the layout of both
//! categories. Mainly by providing a one-to-one mapping with the
//! [XDG Base Directory Specification][xdg] in order to make the layout more
//! predictable.
//!
//! [xdg]: https://specifications.freedesktop.org/basedir-spec/latest/

use anyhow::{anyhow, Result};
use directories::BaseDirs;
use log::trace;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

/// Configuration directory layout representation.
#[cfg_attr(test, automock)]
pub trait DirLayout {
    /// Absolute path to directory where configuration files will be stored.
    fn config_dir(&self) -> &Path;

    /// Absolute path to directory where configuration data will be stored.
    fn data_dir(&self) -> &Path;
}

/// Configuration directory layout handler following [XDG Base Directory
/// Specification][xdg].
///
/// [xdg]: https://specifications.freedesktop.org/basedir-spec/latest/
pub struct XdgDirLayout {
    xdg_spec: BaseDirs,
}

impl XdgDirLayout {
    /// Construct new XDG configuration directory layout handler.
    ///
    /// # Errors
    ///
    /// Will return `[RicerError::Unrecoverable]` if home directory cannot be
    /// determined.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::{DirLayout, XdgDirLayout};
    ///
    /// let layout = XdgDirLayout::new_layout()?;
    /// println!("{}", layout.config_dir().display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`directories::BaseDir::new`][dirs]
    /// - [XDG Base Directory Specification][xdg]
    ///
    /// [dirs]: https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.new
    /// [xdg]: https://specifications.freedesktop.org/basedir-spec/latest/
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    pub fn new_layout() -> Result<Self> {
        trace!("Construct XDG Base Direcotry Specification layout handler");
        let xdg_spec =
            BaseDirs::new().ok_or(anyhow!("Failed to handle XDG Base Directoy Specification"))?;
        Ok(Self { xdg_spec })
    }
}

impl DirLayout for XdgDirLayout {
    fn config_dir(&self) -> &Path {
        self.xdg_spec.config_dir()
    }

    fn data_dir(&self) -> &Path {
        self.xdg_spec.data_dir()
    }
}
