// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

mod config;
mod context;
mod manager;
mod ui;

use std::path::{PathBuf, Path};
use mkdirp::mkdirp;
use anyhow::Result;
use std::fs::{metadata, read_to_string, set_permissions, write};

/// Basic test file fixture.
///
/// Create and manage a basic file fixture for unit and integration testing.
/// File fixtures can be made executable in order to create basic repeatable
/// scripts if needed.
///
/// Be warned, external processes can modify the file that this fixture object
/// keeps track of, which can cause it to contain desynced data. The caller is
/// responsible for ensuring that data housed in this fixture remains synced
/// with the file it is tracking. See [`sync()`] for more details.
///
/// [`sync()`]: #method.sync
#[derive(Debug, Default, Clone)]
pub(crate) struct FileFixture {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileFixture {
    pub fn builder() -> FileFixtureBuilder {
        FileFixtureBuilder::new()
    }

    pub fn as_path(&self) -> &Path {
        &self.path
    }
    
    pub fn as_str(&self) -> &str {
        self.data.as_ref()
    }

    pub fn is_executable(&self) -> bool {
        self.executable
    }

    /// Synchronize file fixture with tracked file path.
    ///
    /// # Errors
    ///
    /// Will fail if file cannot be synced, i.e., read into string form.
    pub fn sync(&mut self) -> Result<()> {
        self.data = read_to_string(&self.path)?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct FileFixtureBuilder {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileFixtureBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }

    pub fn executable(mut self, flag: bool) -> Self {
        self.executable = flag;
        self
    }

    /// Build file fixture.
    ///
    /// Writes given file fixture data at target path with whatever permissions provided.
    /// Will construct parent directory if missing for a given path.
    ///
    /// # Errors
    ///
    /// 1. Will fail if parent directory cannot be constructed when needed.
    /// 2. Will fail if file cannot be written.
    /// 3. Will fail if file permissions cannot be set.
    pub fn build(self) -> Result<FileFixture> {
        mkdirp(self.path.parent().unwrap())?;
        write(&self.path, &self.data)?;

        #[cfg(unix)]
        if self.executable {
            use std::os::unix::fs::PermissionsExt;

            let metadata = metadata(&self.path)?;
            let mut perms = metadata.permissions();
            let mode = perms.mode();
            perms.set_mode(mode | 0o111);
            set_permissions(&self.path, perms)?;
        }

        Ok(FileFixture { path: self.path, data: self.data, executable: self.executable })
    }
}
