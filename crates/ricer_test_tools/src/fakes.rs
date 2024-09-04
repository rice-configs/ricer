// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Fake test-doubles.
//!
//! This helper module is responsible for providing and managing fakes for
//! unit and integration testing. Each fake test-double mainly provides some
//! way to fake some aspect of the user's filesystem, given that Ricer needs
//! to interact extensively with it.
//!
//! Mainly, there is a fake for the user's home directory, a fake for Ricer's
//! configuration directory, and a fake for Ricer's data directory. The fake
//! home test-double mainly allows for testing aspects of Ricer's API that
//! require specific interaction with the user's home directory, e.g., testing
//! creation of configuration directories when they do not exist. The fake
//! configuration directory implementation exists to test Ricer's API that
//! requires interaction with a configuration directory, e.g., configuraiton
//! file manipulation testing. Finally, the fake data directory exists to test
//! Ricer's API that interacts with data, e.g., testing Git repository
//! manipulation functionality.

use std::path::Path;
use tempfile::{Builder, TempDir};

mod config_dir;

#[doc(inline)]
pub use config_dir::*;

/// Fake of home directory.
///
/// A basic fake of a user's home directory where the caller can stuff in any
/// data fixture they need that requires a home directory to function.
#[derive(Debug)]
pub struct FakeHomeDir {
    home_dir: TempDir,
}

impl FakeHomeDir {
    /// Construct new fake home directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeHomeDir;
    ///
    /// let home = FakeHomeDir::new();
    /// ```
    pub fn new() -> Self {
        let home_dir = Builder::new().tempdir().expect("Failed to create fake home directory");
        Self { home_dir }
    }

    /// Get path to fake home directory.
    ///
    /// # Invariants
    ///
    /// Path to fake home directory exists.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeHomeDir;
    ///
    /// let home = FakeHomeDir::new();
    /// println!("{}", home.as_path().display());
    /// ```
    pub fn as_path(&self) -> &Path {
        debug_assert!(self.home_dir.path().exists(), "Path to fake home directory does not exist");
        self.home_dir.path()
    }
}

impl Default for FakeHomeDir {
    fn default() -> Self {
        Self::new()
    }
}
