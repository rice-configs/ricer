// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs::{create_dir, read_dir};
use std::path::{Path, PathBuf};

use crate::fakes::FakeHomeDir;
use crate::fixtures::FileFixture;

use crate::util::err_check;

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
pub struct FakeConfigDir {
    home_dir: FakeHomeDir,
    config_dir: PathBuf,
    hooks_dir: PathBuf,
    file_fixtures: HashMap<PathBuf, FileFixture>,
}

impl FakeConfigDir {
    /// Build new fake configuration directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fakes::FakeConfigDir;
    ///
    /// let fake_dir = FakeConfigDir::builder()
    ///     .config_file("config.toml", "data in here!")
    ///     .hook_script("hook.sh", "scripting in here!")
    ///     .build();
    /// assert_eq!(fake_dir.get_config_file("config.toml").data(), "data in here!");
    /// assert_eq!(fake_dir.get_hook_script("hook.sh").data(), "scripting in here!");
    /// ```
    ///
    /// # See also
    ///
    /// - [`FakeConfigDirBuilder`]
    pub fn builder() -> FakeConfigDirBuilder {
        FakeConfigDirBuilder::new()
    }

    /// Get tracked configuration file fixture.
    ///
    /// # Panics
    ///
    /// Will panic if configuration file fixture is not being tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fakes::FakeConfigDir;
    ///
    /// let fake_dir = FakeConfigDir::builder()
    ///     .config_file("config.toml", "data in here!")
    ///     .build();
    /// assert_eq!(fake_dir.get_config_file("config.toml").data(), "data in here!");
    /// ```
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    ///
    /// [`FileFixture`]: crate::fixtures::FileFixture
    pub fn get_config_file(&self, name: impl AsRef<str>) -> &FileFixture {
        match self.file_fixtures.get(&self.config_dir.join(name.as_ref())) {
            Some(file) => file,
            None => panic!("Configuration file '{}' is not being tracked", &name.as_ref()),
        }
    }

    /// Get tracked hook script fixture.
    ///
    /// # Panics
    ///
    /// Will panic if hook script is not being tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fakes::FakeConfigDir;
    ///
    /// let fake_dir = FakeConfigDir::builder()
    ///     .hook_script("hook.sh", "scripting in here!")
    ///     .build();
    /// assert_eq!(fake_dir.get_hook_script("hook.sh").data(), "scripting in here!");
    /// ```
    ///
    /// # See also
    ///
    /// - [`FileFixture`]
    ///
    /// [`FileFixture`]: crate::fixtures::FileFixture
    pub fn get_hook_script(&self, name: impl AsRef<str>) -> &FileFixture {
        match self.file_fixtures.get(&self.hooks_dir.join(name.as_ref())) {
            Some(script) => script,
            None => panic!("Hook script '{}' is not being tracked", &name.as_ref()),
        }
    }

    /// Synchronize fake configuration directory.
    ///
    /// Will synchronize currently tracked file fixtures in both the
    /// configuration directory and hooks sub-directory. Will also track any new
    /// files that were added by external processes or functions in either the
    /// configuration directory or the hooks sub-directory.
    ///
    /// # Panics
    ///
    /// Will panic if fake configuration directory cannot be read for traversal.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fakes::FakeConfigDir;
    /// use std::fs::write;
    ///
    /// let mut fake_dir = FakeConfigDir::builder().build();
    /// write(fake_dir.config_dir().join("repos.toml"), "some repo definition");
    /// fake_dir.sync();
    ///
    /// let file = fake_dir.get_config_file("repos.toml");
    /// assert_eq!(file.data(), "some repo definition");
    /// ```
    ///
    /// # See also
    ///
    /// - [`FileFixture::sync`]
    ///
    /// [`FileFixture::sync`]: crate::fixtures::FileFixture::sync
    pub fn sync(&mut self) {
        for (_, fixture) in self.file_fixtures.iter_mut() {
            fixture.sync();
        }

        self.sync_dir(self.config_dir.clone());
        self.sync_dir(self.hooks_dir.clone());
    }

    /// Synchronize target directory.
    ///
    /// # Panics
    ///
    /// Will panic if it cannot read the target directory for traversal.
    fn sync_dir(&mut self, path: impl AsRef<Path>) {
        for entry in err_check!(read_dir(&path.as_ref())) {
            let entry = err_check!(entry);
            let path = entry.path();
            if path.is_file() {
                let fixture = FileFixture::from(path);
                self.file_fixtures.insert(fixture.as_path().to_path_buf(), fixture);
            }
        }
    }

    /// Get path to fake home directory.
    pub fn home_dir(&self) -> &Path {
        self.home_dir.as_path()
    }

    /// Get path to configuration directory.
    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    /// Get path to hook sub-directory.
    pub fn hooks_dir(&self) -> &Path {
        self.hooks_dir.as_path()
    }
}

/// Builder for [`FakeConfigDir`].
#[derive(Debug)]
pub struct FakeConfigDirBuilder {
    home_dir: FakeHomeDir,
    config_dir: PathBuf,
    hooks_dir: PathBuf,
    file_fixtures: HashMap<PathBuf, FileFixture>,
}

impl FakeConfigDirBuilder {
    /// Construct new builder for [`FakeConfigDir`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeConfigDirBuilder;
    ///
    /// let builder = FakeConfigDirBuilder::new();
    /// ```
    pub fn new() -> Self {
        let home_dir = FakeHomeDir::new();
        let config_dir = PathBuf::from(format!("{}/ricer", home_dir.as_path().display()));
        let hooks_dir = PathBuf::from(format!("{}/hooks", config_dir.as_path().display()));
        err_check!(create_dir(&config_dir));
        err_check!(create_dir(&hooks_dir));
        Self { home_dir, config_dir, hooks_dir, file_fixtures: HashMap::default() }
    }

    /// Add a new configuration file fixture.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeConfigDirBuilder;
    ///
    /// let builder = FakeConfigDirBuilder::new().config_file("config.toml", "data in here!");
    /// ```
    ///
    /// # See also
    ///
    /// [`FileFixture`]
    ///
    /// [`FileFixture`]: crate::fixtures::FileFixture
    pub fn config_file(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let fixture = FileFixture::builder()
            .path(self.config_dir.as_path().join(name.as_ref()))
            .data(data.as_ref())
            .executable(false)
            .build();

        self.file_fixtures.insert(fixture.as_path().to_path_buf(), fixture);
        self
    }

    /// Add a new hook script fixture.
    ///
    /// # Examples
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeConfigDirBuilder;
    ///
    /// let builder = FakeConfigDirBuilder::new().hook_script("hook.sh", "data in here!");
    /// ```
    ///
    /// # See also
    ///
    /// [`FileFixture`]
    ///
    /// [`FileFixture`]: crate::fixtures::FileFixture
    pub fn hook_script(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let fixture = FileFixture::builder()
            .path(self.hooks_dir.as_path().join(name.as_ref()))
            .data(data.as_ref())
            .executable(true)
            .build();

        self.file_fixtures.insert(fixture.as_path().to_path_buf(), fixture);
        self
    }

    /// Build new [`FakeConfigDir`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeConfigDirBuilder;
    ///
    /// let fake_dir = FakeConfigDirBuilder::new()
    ///     .config_file("config.toml", "data in here!")
    ///     .hook_script("hook.sh", "data in here!")
    ///     .build();
    /// ```
    pub fn build(self) -> FakeConfigDir {
        FakeConfigDir {
            home_dir: self.home_dir,
            config_dir: self.config_dir,
            hooks_dir: self.hooks_dir,
            file_fixtures: self.file_fixtures,
        }
    }
}
