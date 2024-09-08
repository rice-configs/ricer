// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Test fixtures.
//!
//! Standard test fixture creation and management for unit and integration
//! testing in Ricer's codebase.

use is_executable::IsExecutable;
use mkdirp::mkdirp;
use std::fs::{metadata, read_to_string, set_permissions, write};
use std::path::{Path, PathBuf};

mod dotfile;
mod git;

#[doc(inline)]
pub use dotfile::*;

#[doc(inline)]
pub use git::*;

use crate::util::err_check;

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
pub struct FileFixture {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileFixture {
    /// Build new file fixture through builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fixtures::FileFixture;
    /// use std::path::PathBuf;
    ///
    /// let file = FileFixture::builder()
    ///     .path("/tmp/test_data.toml")
    ///     .data("key = 'value'")
    ///     .executable(false)
    ///     .build();
    /// assert_eq!(file.as_path(), PathBuf::from("/tmp/test_data.toml").as_path());
    /// assert_eq!(file.data(), "key = 'value'");
    /// assert_eq!(file.is_executable(), false);
    /// ```
    ///
    /// # See
    ///
    /// - [`FileFixtureBuilder`]
    pub fn builder() -> FileFixtureBuilder {
        FileFixtureBuilder::new()
    }

    /// Get path to tracked file.
    ///
    /// # Invariants
    ///
    /// Always contains a path that exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fixtures::FileFixture;
    ///
    /// let file = FileFixture::builder().path("/tmp/test_data.toml").build();
    /// assert_eq!(file.as_path().exists(), true);
    /// ```
    pub fn as_path(&self) -> &Path {
        debug_assert!(self.path.exists(), "File fixture path does not exist");
        self.path.as_path()
    }

    /// Get data in tracked file.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fixtures::FileFixture;
    ///
    /// let file = FileFixture::builder().path("/tmp/test_data.toml").data("key = 'value'").build();
    /// assert_eq!(file.data(), "key = 'value'");
    /// ```
    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    /// Check if tracked file is executable.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fixtures::FileFixture;
    ///
    /// let file = FileFixture::builder()
    ///     .path("/tmp/script.sh")
    ///     .data("echo 'do something!'")
    ///     .executable(true)
    ///     .build();
    /// assert_eq!(file.is_executable(), true);
    /// ```
    pub fn is_executable(&self) -> bool {
        self.executable
    }

    /// Synchronize file fixture with tracked file.
    ///
    /// # Panics
    ///
    /// Will panic if file cannot be read for whatever reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fixtures::FileFixture;
    /// use std::fs::write;
    ///
    /// let mut file = FileFixture::builder()
    ///     .path("/tmp/test_data.toml")
    ///     .data("key = 'value'")
    ///     .executable(false)
    ///     .build();
    /// write(file.as_path(), "key = 'modified'");
    /// file.sync();
    /// assert_eq!(file.data(), "key = 'modified'");
    /// ```
    pub fn sync(&mut self) {
        self.data = err_check!(read_to_string(&self.path));
    }
}

impl From<PathBuf> for FileFixture {
    fn from(path: PathBuf) -> Self {
        let mut fixture = FileFixture::default();
        fixture.path = path;
        fixture.sync();

        if fixture.path.is_executable() {
            fixture.executable = true;
        } else {
            fixture.executable = false;
        }

        fixture
    }
}

/// Builder for [`FileFixture`].
#[derive(Debug, Default, Clone)]
pub struct FileFixtureBuilder {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileFixtureBuilder {
    /// Construct new builder for [`FileFixture`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::FileFixtureBuilder;
    ///
    /// let builder = FileFixtureBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set path to tracked file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::FileFixtureBuilder;
    ///
    /// let builder = FileFixtureBuilder::new()
    ///     .path("test_data.toml");
    /// ```
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = path.as_ref().to_path_buf();
        self
    }

    /// Set data to write into tracked file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::FileFixtureBuilder;
    ///
    /// let builder = FileFixtureBuilder::new()
    ///     .path("test_data.toml")
    ///     .data("Something to write!");
    /// ```
    pub fn data(mut self, data: impl AsRef<str>) -> Self {
        self.data = data.as_ref().to_string();
        self
    }

    /// Set executable flag on tracked file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::FileFixtureBuilder;
    ///
    /// let builder = FileFixtureBuilder::new()
    ///     .path("script.sh")
    ///     .data("echo 'do something'")
    ///     .executable(true);
    /// ```
    pub fn executable(mut self, flag: bool) -> Self {
        self.executable = flag;
        self
    }

    /// Build new [`FileFixture`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::FileFixtureBuilder;
    ///
    /// let file = FileFixtureBuilder::new()
    ///     .path("script.sh")
    ///     .data("echo 'do something'")
    ///     .executable(true)
    ///     .build();
    /// ```
    pub fn build(mut self) -> FileFixture {
        self.write();
        FileFixture { path: self.path, data: self.data, executable: self.executable }
    }

    /// Write contents of file fixture.
    ///
    /// Will create the parent path to a file if it does not exist. Will also
    /// set execute permissions if set by the caller.
    ///
    /// # Panics
    ///
    /// Will panic if parent path cannot be created, contents of the file cannot
    /// be written, or execute permission data cannot be set.
    pub(crate) fn write(&mut self) {
        err_check!(mkdirp(self.path.parent().unwrap()));
        err_check!(write(&self.path, &self.data));

        #[cfg(unix)]
        if self.executable {
            use std::os::unix::fs::PermissionsExt;

            let metadata = err_check!(metadata(&self.path));
            let mut perms = metadata.permissions();
            let mode = perms.mode();
            perms.set_mode(mode | 0o111);
            err_check!(set_permissions(&self.path, perms));
        }
    }
}
