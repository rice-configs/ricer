// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

mod context;
mod data_xchg;
mod manager;
mod ui;

use anyhow::{anyhow, Result};
use is_executable::IsExecutable;
use mkdirp::mkdirp;
use std::collections::HashMap;
use std::fs::{metadata, read_dir, read_to_string, set_permissions, write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Fake Ricer's expected configuration directory.
///
/// Ricer's configuration directory houses all configuration files at the
/// top-level. A sub-directory exists known as the "hooks" directory that
/// contains all user-defined hooks to be executed for a given command hook
/// definition.
///
/// This fake implementation mainly exists to separate unit and integration
/// tests from the user's home directory in order to avoid messing with any of
/// their existing configurations, and to make any test function independent of
/// the user's filesystem.
///
/// Caller is expected to fill this fake configuration directory with file data
/// fixtures in order to test any of Ricer's internal library API that requires
/// access to the user's filesystem.
#[derive(Debug)]
pub(crate) struct FakeConfigDir {
    config_dir: TempDir,
    hook_dir: PathBuf,
    fixtures: HashMap<PathBuf, FileFixture>,
}

impl FakeConfigDir {
    /// Build new fake configuration directory.
    ///
    /// # Errors
    ///
    /// Will fail if builder fails to construct temporary directory to use.
    ///
    /// # See also
    ///
    /// - [`FakeConfigDirBuilder`]
    pub fn builder() -> Result<FakeConfigDirBuilder> {
        FakeConfigDirBuilder::open()
    }

    /// Get configuration file fixture.
    ///
    /// # Errors
    ///
    /// Will fail if configuration file is not being tracked, because caller
    /// is out of sync or fixture itself does not exist.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn config_file_fixture(&self, name: impl AsRef<str>) -> Result<&FileFixture> {
        self.fixtures
            .get(&self.config_dir.path().join(name.as_ref()))
            .ok_or(anyhow!("Configuration file fixture '{}' not being tracked", name.as_ref()))
    }

    /// Get hook script fixture.
    ///
    /// # Errors
    ///
    /// Will fail if hook script is not being tracked, because caller
    /// is out of sync or fixture itself does not exist.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn hook_script_fixture(&self, name: impl AsRef<str>) -> Result<&FileFixture> {
        self.fixtures
            .get(&self.hook_dir.join(name.as_ref()))
            .ok_or(anyhow!("Hook script fixture '{}' not being tracked", name.as_ref()))
    }

    /// Get file fixture through absolute path.
    ///
    /// # Errors
    ///
    /// Will fail if file fixture is not being tracked, because caller is out
    /// of sync or fixture itself does not exist.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn fixture(&self, path: impl AsRef<Path>) -> Result<&FileFixture> {
        self.fixtures
            .get(path.as_ref())
            .ok_or(anyhow!("Fixture '{}' not being tracked", path.as_ref().display()))
    }

    /// Synchronize fixtures across entire fake configuration directory.
    ///
    /// Sync top-level and hooks directory by updating existing fixtures being
    /// tracked, and add any new fixtures that were not being tracked before.
    ///
    /// # Errors
    ///
    /// Will fail if top-level directory or hooks directory cannot be read, or
    /// if any of the fixtures themselves can be synced.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn sync(&mut self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter_mut() {
            fixture.sync()?;
        }

        self.sync_dir(self.config_dir.path().to_path_buf())?;
        self.sync_dir(self.hook_dir.to_path_buf())?;
        Ok(())
    }

    /// Synchronize target directory only.
    ///
    /// Update existing fixtures and add any new fixtures in target directory
    /// that are not currently being tracked.
    ///
    /// # Errors
    ///
    /// Will fail if target directory cannot be read, or if any fixtures
    /// themselves cannot be synced.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn sync_dir(&mut self, path: impl AsRef<Path>) -> Result<()> {
        if path.as_ref().is_dir() {
            for entry in read_dir(path.as_ref())? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let fixture = FileFixture::try_from(path)?;
                    self.fixtures.insert(fixture.as_path().into(), fixture);
                }
            }
        }

        Ok(())
    }

    pub fn config_dir(&self) -> &Path {
        self.config_dir.path()
    }

    pub fn hook_dir(&self) -> &Path {
        self.hook_dir.as_ref()
    }
}

#[derive(Debug)]
pub(crate) struct FakeConfigDirBuilder {
    config_dir: TempDir,
    hook_dir: PathBuf,
    fixtures: HashMap<PathBuf, FileFixture>,
}

impl FakeConfigDirBuilder {
    /// Open new fake configuration directory.
    ///
    /// # Errors
    ///
    /// Will fail if temporary directory cannot be opened.
    pub fn open() -> Result<Self> {
        let config_dir = tempfile::Builder::new().tempdir()?;
        let hook_dir = config_dir.path().join("hooks");
        mkdirp(&hook_dir)?;
        Ok(Self { config_dir, hook_dir, fixtures: HashMap::new() })
    }

    /// Add new configuration file fixture.
    ///
    /// # Errors
    ///
    /// Will fail if file fixture cannot be constructed.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn config_file(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Result<Self> {
        let fixture = FileFixture::builder()
            .path(self.config_dir.path().join(name.as_ref()))
            .data(data.as_ref())
            .executable(false)
            .build()?;
        self.fixtures.insert(fixture.as_path().into(), fixture);
        Ok(self)
    }

    /// Add new executable hook script fixture.
    ///
    /// # Errors
    ///
    /// Will fail if executable file fixture cannot be constructed.
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    pub fn hook_script(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Result<Self> {
        let fixture = FileFixture::builder()
            .path(self.hook_dir.as_path().join(name.as_ref()))
            .data(data.as_ref())
            .executable(true)
            .build()?;
        self.fixtures.insert(fixture.as_path().into(), fixture);
        Ok(self)
    }

    pub fn build(self) -> FakeConfigDir {
        FakeConfigDir {
            config_dir: self.config_dir,
            hook_dir: self.hook_dir,
            fixtures: self.fixtures,
        }
    }
}

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

impl TryFrom<PathBuf> for FileFixture {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut fixture = FileFixture::default();
        fixture.path = path;
        fixture.sync()?;

        if fixture.path.is_executable() {
            fixture.executable = true;
        } else {
            fixture.executable = false;
        }

        Ok(fixture)
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
