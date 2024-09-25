// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Format preserving configuration file handling.
//!
//! Ricer's standard configuration file handling module that provides efficient
//! ways to read, write, parse, serialize, and deserialize configuration file
//! data while preserving the original formatting.
//!
//! Ricer uses the [TOML file format][toml-spec] for configuration files.
//! Currently, Ricer splits configuration file data into two main categories:
//! repository configurations, and command hook configurations. Repository
//! configurations mainly deal with detailing _what_ kinds of repositories Ricer
//! needs to keep track of, and _how_ to manage them according to the user's
//! specifications. Command hook configurations specify what hook scripts need
//! to be executed for a given Ricer command, as well as _when_ and _where_
//! those hook scripts need to be executed.
//!
//! Repository configurations and command hook configurations recieve their own
//! separate configuration file. To reflect this design decision, this module
//! provides a repository configuration file handler, and a command hook
//! configuration file handler respectively.
//!
//! This module makes no assumptions about the locations of each configuration
//! file. All it cares about is that the caller provides valid configuration
//! file data to handle.
//!
//! [toml-spec]: https://toml.io/en/v1.0.0

use anyhow::{anyhow, Result};
use log::{debug, info, trace};
use std::fmt;
use toml_edit::{DocumentMut, Item, Key, Table};

/// TOML file parser.
///
/// # Invariants
///
/// Preserve original formatting for any modifications made to the file.
#[derive(Clone, Default, Debug)]
pub struct Toml {
    doc: DocumentMut,
}

impl Toml {
    /// Load TOML file at `path`.
    ///
    /// If TOML file exists at `path`, then it will be parsed into `Self`. If
    /// TOML file does not exist at `path`, then it will be created at `path`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::Toml;
    ///
    /// let toml = Toml::new();
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        trace!("Construct new TOML file parser");
        Self { doc: DocumentMut::new() }
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}
