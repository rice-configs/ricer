// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use mkdirp::mkdirp;
use std::{
    path::{Path, PathBuf},
    fs::{metadata, set_permissions, write, read_to_string},
};


/// Test file fixture.
///
/// Create and manage a file fixture for unit and integration testing.
/// File fixtures can be made executable in order to crate basic repeatable
/// scripts if needed.
///
/// Be warned, external processes can modify the file that this fixture keeps
/// track of, which can cause it to contain desynced data. The caller is
/// responsible for ensuring that data housed in a fixture remains synced with
/// the file it is tracking.
#[derive(Debug, Default, Clone)]
pub struct FileFixture {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileFixture {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            data: Default::default(),
            executable: Default::default(),
        }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }

    pub fn with_executable(mut self, flag: bool) -> Self {
        self.executable = flag;
        self
    }

    /// Write file fixture at tracked path.
    ///
    /// Writes file fixture at tracked path with executable permissions set.
    /// Will make parent directory if missing for a given path.
    ///
    /// # Errors
    ///
    /// - May fail if parent directory cannot be made if needed.
    /// - May fail if flie cannot be written.
    /// - May fail if executable permissions cannot be set.
    pub fn write(&self) -> Result<()> {
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

        Ok(())
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

    /// Synchronize file fixture at tracked path.
    ///
    /// # Errors
    ///
    /// Will fail if file cannot be synced, i.e., read into string form.
    pub fn sync(&mut self) -> Result<()> {
        self.data = read_to_string(&self.path)?;
        Ok(())
    }
}
