// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Test stub management.
//!
//! This helper module is responsible for managing stubs for integration tests
//! in Ricer.

use std::fs::{read_to_string, create_dir, metadata, set_permissions, write};
use std::path::{Path, PathBuf};

/// Basic stub of `std::fs::File`.
///
/// The `std::fs::File` struct handler does not keep track of the path it is
/// operating on, nor does it keep track of a file being executable. This super
/// basic stub of `std::fs::File` is meant to provide this functionality to make
/// integration tests easier in the long run, while providing the ability to
/// write and read data.
///
/// It keeps the data the caller wrote to a target file with this stub file
/// handler in case the user wants to compare how well there test results read
/// the target file. Thus, avoiding the need to reread the file with this stub
/// file handler.
#[derive(Debug)]
pub struct FileStub {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileStub {
    /// Construct new builder instance.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStub;
    ///
    /// let file_stub_builder = FileStub::builder();
    /// ```
    pub fn builder() -> FileStubBuilder {
        FileStubBuilder::new()
    }

    /// Get path of stub file handler.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStub;
    ///
    /// let file_stub = FileStub::builder()
    ///     .path("/some/where.txt")
    ///     .build()
    /// let file_path = file_stub.as_path();
    /// ```
    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    /// Get data written to target file.
    ///
    /// May be out of sync if external method modifies the target file that
    /// `FileStub` instance is handling.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStub;
    ///
    /// let file_stub = FileStub::builder()
    ///     .path("/some/where.txt")
    ///     .data("Hello world!")
    ///     .build();
    /// let file_data = file_stub.data();
    /// ```
    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    /// Check if file is executable.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStub;
    ///
    /// let file_stub = FileStub::builder()
    ///     .path("/some/where.txt")
    ///     .executable(true)
    ///     .build();
    /// let file_exe = file_stub.is_executable();
    /// ```
    pub fn is_executable(&self) -> bool {
        self.executable
    }

    /// Synchronize file stub with its target file.
    ///
    /// Should be used in case some external method or process modifies the
    /// contents of the file that the stub file handler is managing.
    ///
    /// # Errors
    ///
    /// Panics if it cannot read the contents of the target file due to invalid
    /// UTF-8, or the file being deleted thus causing it not to exist anymore.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStub;
    ///
    /// let file_stub = FileStub::builder()
    ///     .path("/some/where.txt")
    ///     .build();
    /// file_stub.sync();
    /// ```
    pub fn sync(&mut self) {
        self.data = read_to_string(&self.path).expect("Failed to sync stub file");
    }
}

/// Builder for [`FileStub`].
#[derive(Debug)]
pub struct FileStubBuilder {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl FileStubBuilder {
    /// Construct new instance of file stub builder.
    ///
    /// Caller should use `FileStub::builder()` instead of directly calling this
    /// method. That way they can use `FileStub` more directly. Unless they need
    /// the file stub instance separate from their file stub builder instance for
    /// whatever reason (unlikely but possible).
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStubBuilder;
    ///
    /// let builder = FileStubBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self { path: PathBuf::default(), data: String::default(), executable: false }
    }

    /// Set path to a target file.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStubBuilder;
    ///
    /// let builder = FileStubBuilder::new()
    ///     .path("/some/where.txt");
    /// ```
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = path.as_ref().to_path_buf();
        self
    }

    /// Set data to write into target file.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStubBuilder;
    ///
    /// let builder = FileStubBuilder::new()
    ///     .path("/some/where.txt")
    ///     .data("Hello world!");
    /// ```
    pub fn data(mut self, data: impl AsRef<str>) -> Self {
        self.data = data.as_ref().to_string();
        self
    }

    /// Make target file executable.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStubBuilder;
    ///
    /// let builder = FileStubBuilder::new()
    ///     .path("/some/where.txt")
    ///     .executable(true);
    /// ```
    pub fn executable(mut self, flag: bool) -> Self {
        self.executable = flag;
        self
    }

    /// Build instance of `FileStub`.
    ///
    /// Panics if it cannot create `FileStub` instance.
    ///
    /// Postconditions:
    ///
    /// 1. Create valid instance of `FileStub`.
    ///
    /// Examples:
    ///
    /// ```
    /// use ricer_test_tools::stubs::FileStubBuilder;
    ///
    /// let file_stub = FileStubBuilder::new()
    ///     .path("/some/where.txt")
    ///     .data("Hello world")
    ///     .executable(true)
    ///     .build();
    /// ```
    pub fn build(self) -> FileStub {
        write(&self.path, &self.data).unwrap_or_else(|error| {
            panic!("Failed to create file '{}': {}", self.path.display(), error)
        });

        #[cfg(unix)]
        if self.executable {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = metadata(&self.path).unwrap().permissions();
            let mode = perms.mode();

            perms.set_mode(mode | 0o111);
            set_permissions(&self.path, perms).unwrap();
        }

        FileStub { path: self.path, data: self.data, executable: self.executable }
    }
}

/// Basic stub of a Git repository.
///
/// Mainly used to provide basic Git repository stubs for integration testing
/// with `FakeConfigDir`.
#[derive(Debug)]
pub struct GitRepoStub {
    path: PathBuf,
}

impl GitRepoStub {
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
