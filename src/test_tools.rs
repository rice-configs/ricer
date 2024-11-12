// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use is_executable::IsExecutable;
use mkdirp::mkdirp;
use std::{
    collections::HashMap,
    fs::{metadata, read_to_string, set_permissions, write},
    path::{Path, PathBuf},
};
use tempfile::{Builder as TempFileBuilder, TempDir};
use walkdir::WalkDir;

/// Directory of file fixtures.
///
/// Used to bundle collections of file fixtures under one temporary directory.
/// Useful for creating a set of file fixtures that need to be stored in a
/// singular temporary location for testing.
pub struct DirFixture {
    dir: TempDir,
    fixtures: HashMap<PathBuf, FileFixture>,
}

impl DirFixture {
    /// Open new directory fixture.
    ///
    /// # Errors
    ///
    /// - May fail if temporary directory cannot be created.
    pub fn open() -> Result<Self> {
        let dir = TempFileBuilder::new().tempdir()?;
        Ok(Self { dir, fixtures: HashMap::new() })
    }

    pub fn with_file(
        mut self,
        path: impl AsRef<Path>,
        data: impl AsRef<str>,
        kind: FileFixtureKind,
    ) -> Self {
        let fixture = FileFixture::new(self.dir.path().join(path.as_ref()))
            .with_data(data.as_ref())
            .with_kind(kind);
        self.fixtures.insert(fixture.as_path().into(), fixture);
        self
    }

    /// Get tracked file fixture.
    ///
    /// # Errors
    ///
    /// - May fail if file fixture is not being tracked.
    pub fn get_fixture(&self, path: impl AsRef<Path>) -> Result<&FileFixture> {
        self.fixtures.get(&self.dir.path().join(path.as_ref())).ok_or(anyhow!(
            "Fixture '{}' not being tracked in '{}'",
            path.as_ref().display(),
            self.dir.path().display()
        ))
    }

    /// Write all file fixtures into directory fixture.
    ///
    /// # Errors
    ///
    /// - May fail if tracked fixture cannot be written for whatever reason.
    ///
    /// # See also
    ///
    /// - [`FileFixture::write`]
    pub fn write(&self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter() {
            fixture.write()?;
        }

        Ok(())
    }

    /// Synchronize all file fixtures in directory fixture.
    ///
    /// Will track any files that were newly added into the temporary directory.
    ///
    /// # Errors
    ///
    /// - May file if tracked file fixture cannot be syncronized for whatever
    ///   reason.
    ///
    /// # See also
    ///
    /// - [`FileFixture::sync`]
    pub fn sync(&mut self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter_mut() {
            fixture.sync()?;
        }

        // Track any new files that were added by some external process(es)...
        for entry in WalkDir::new(self.dir.path()) {
            let path = entry?.path().to_path_buf();
            let data = read_to_string(&path)?;
            let kind = match path.is_executable() {
                true => FileFixtureKind::Script,
                false => FileFixtureKind::Normal,
            };
            let fixture = FileFixture::new(path)
                .with_data(data)
                .with_kind(kind);
            self.fixtures.insert(fixture.as_path().into(), fixture);
        }

        Ok(())
    }

    pub fn as_path(&self) -> &Path {
        self.dir.path()
    }
}

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
    kind: FileFixtureKind,
}

impl FileFixture {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), data: Default::default(), kind: Default::default() }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }

    pub fn with_kind(mut self, kind: FileFixtureKind) -> Self {
        self.kind = kind;
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
        if self.kind == FileFixtureKind::Script {
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
        self.kind == FileFixtureKind::Script
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

/// Determine file fixture to write.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FileFixtureKind {
    /// Normal file with read and write permissions.
    #[default]
    Normal,

    /// Executable file with read and write permissions.
    Script,
}
