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

use anyhow::Result;
use toml_edit::DocumentMut;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use log::{debug, trace};
use std::fmt;

/// TOML file parser.
///
/// # Invariants
///
/// Preserve original formatting for any modifications made to the file.
#[derive(Debug)]
pub struct Toml {
    document: DocumentMut,
    file: File,
    path: PathBuf,
}

impl Toml {
    /// Load TOML file at `path`.
    ///
    /// If TOML file exists at `path`, then it will be parsed into `Self`. If
    /// TOML file does not exist at `path`, then it will be created at `path`.
    ///
    /// # Errors
    ///
    /// This function will fail for the following reasons:
    ///
    /// - One of the directory components of `path` does not exist.
    /// - Lacks permission to access the TOML file, or access one of the
    ///   directory components to the TOML file.
    /// - TOML file contains invalid TOML formatting, or invalid UTF-8.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::Toml;
    ///
    /// let toml = Toml::load("/path/to/config.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        trace!("Construct new TOML file parser");
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path.as_ref())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let document: DocumentMut = buffer.parse()?;

        debug!("Load TOML file at '{}'", path.as_ref().display());
        Ok(Self { document, file, path: path.as_ref().into() })
    }

    /// Save TOML file at loaded `path`.
    ///
    /// # Errors
    ///
    /// Will fail if writing to loaded `path` cannot be done.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::Toml;
    ///
    /// let mut toml = Toml::load("/path/to/config.toml")?;
    /// // Do some operations on `toml`...
    /// toml.save()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See also
    ///
    /// - [`std::io::Write::write`]
    ///
    /// [`std::io::Write::write`]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write
    pub fn save(&mut self) -> Result<()> {
        debug!("Save TOML file at '{}'", self.path.display());
        let buffer = self.document.to_string();
        self.file.write_all(buffer.as_bytes())?;
        Ok(())
    }

    /// Get path to TOML file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::Toml;
    ///
    /// let toml = Toml::load("/path/to/config.toml")?;
    /// let path = toml.as_path();
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_path(&self) -> &Path {
        &self.path
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.document)
    }
}
