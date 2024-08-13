// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration file management.
//!
//! This module provides a simple interface to manipulate and manage Ricer's
//! configuration file. Ricer uses a special configuration file named
//! `config.toml` in its base directory in `$XDG_CONFIG_HOME/ricer`. This
//! configuration file uses the [TOML format[toml-spec] so the user can modify
//! it by hand in case they do not want to go through Ricer's command set for
//! whatever reason.
//!
//! [toml-spec]: https://toml.io/en/v1.0.0

use std::path::Path;

pub mod hooks_section;
pub mod repos_section;

use crate::error::RicerResult;
use repos_section::RepoEntry;

/// Configuration file manager representation.
pub trait ConfigFileManager {
    /// Read from configuration file at provided path.
    fn read(&mut self, path: impl AsRef<Path>) -> RicerResult<()>;

    /// Write to configuration file at provided path.
    fn write(&mut self, path: impl AsRef<Path>) -> RicerResult<()>;

    /// Show current configuration file data in string form.
    fn data(&self) -> String;

    /// Deserialize repository entry from parsed configuration file data.
    fn get_repo_entry(&self, repo_name: impl AsRef<str>) -> Option<RepoEntry>;
}
