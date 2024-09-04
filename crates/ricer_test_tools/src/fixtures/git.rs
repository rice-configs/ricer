// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use std::fs::create_dir;
use std::path::{Path, PathBuf};

/// Basic stub of a Git repository.
///
/// Mainly used to provide basic Git repository stubs for integration testing
/// with `FakeConfigDir`.
#[derive(Debug)]
pub struct GitRepoFixture {
    path: PathBuf,
}

impl GitRepoFixture {
    /// Create new Git repository stub instance.
    ///
    /// Errors:
    ///
    /// Panics if it cannot create Git repository.
    pub fn new(path: impl AsRef<Path>) -> Self {
        // TODO: Make this stub more like a Git repo rather than an empty dir...
        create_dir(path.as_ref()).expect("Failed to create repository");
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Get path to Git repository stub.
    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }
}
