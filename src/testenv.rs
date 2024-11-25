// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use is_executable::IsExecutable;
use mkdirp::mkdirp;
use git2::{Repository, IndexAddOption, RepositoryInitOptions};
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
pub struct FakeDir {
    dir: TempDir,
    fixtures: HashMap<PathBuf, Fixture>,
    repos: HashMap<PathBuf, Repository>,
}

impl FakeDir {
    /// Open new directory fixture.
    ///
    /// # Errors
    ///
    /// - May fail if temporary directory cannot be created.
    pub fn open() -> Result<Self> {
        let dir = TempFileBuilder::new().tempdir()?;
        Ok(Self { dir, fixtures: HashMap::new(), repos: HashMap::new() })
    }

    pub fn with_file(
        mut self,
        path: impl AsRef<Path>,
        data: impl AsRef<str>,
        kind: FixtureKind,
    ) -> Self {
        let fixture = Fixture::new(self.dir.path().join(path.as_ref()))
            .with_data(data.as_ref())
            .with_kind(kind);
        self.fixtures.insert(fixture.as_path().into(), fixture);
        self
    }

    /// Setup file and repository fixtures inside fake directory.
    ///
    /// Will write file fixtures at specified locations, and will stage and
    /// commit files inside repository fixtures.
    ///
    /// # Errors
    ///
    /// - May fail if tracked fixture cannot be written for whatever reason.
    /// - May fail if tracked repository cannot update its index and create an
    ///   initial commit.
    ///
    /// # See also
    ///
    /// - [`FileFixture::write`]
    pub fn setup(self) -> Result<Self> {
        for (_, fixture) in self.fixtures.iter() {
            fixture.write()?
        }

        for (_, repo) in self.repos.iter() {
            let mut index = repo.index()?;
            index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
            index.write()?;

            let tree_id = index.write_tree()?;
            let sig = repo.signature()?;

            let mut parents = Vec::new();
            if let Some(parent) = repo.head().ok().map(|h| h.target().unwrap()) {
                parents.push(repo.find_commit(parent)?);
            }
            let parents = parents.iter().collect::<Vec<_>>();

            let tree = repo.find_tree(tree_id)?;
            repo.commit(Some("HEAD"), &sig, &sig, "initial commit\n\nbody", &tree, &parents)?;
        }

        Ok(self)
    }

    /// Get tracked file fixture.
    ///
    /// # Errors
    ///
    /// - May fail if file fixture is not being tracked.
    pub fn fixture(&self, path: impl AsRef<Path>) -> Result<&Fixture> {
        match self.fixtures.get(&self.dir.path().join(path.as_ref())) {
            Some(fixture) => Ok(fixture),
            None => Err(anyhow!("Fixture '{}' not being tracked", path.as_ref().display())),
        }
    }

    /// Get tracked mutable file fixture.
    ///
    /// # Errors
    ///
    /// - May fail if file fixture is not being tracked.
    pub fn fixture_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut Fixture> {
        match self.fixtures.get_mut(&self.dir.path().join(path.as_ref())) {
            Some(fixture) => Ok(fixture),
            None => Err(anyhow!("Fixture '{}' not being tracked", path.as_ref().display())),
        }
    }

    /// Get tracked Git repository fixture.
    ///
    /// # Errors
    ///
    /// - May fail if repository is not being tracked.
    pub fn repo(&self, path: impl AsRef<Path>) -> Result<&Repository> {
        match self.repos.get(&self.dir.path().join(path.as_ref())) {
            Some(repo) => Ok(repo),
            None => Err(anyhow!("Git repository '{}' not being tracked", path.as_ref().display())),
        }
    }

    /// Synchronize all file fixtures in directory fixture.
    ///
    /// Will track any files that were newly added into the temporary directory.
    ///
    /// # Errors
    ///
    /// - May fail if tracked file fixture cannot be syncronized for whatever
    ///   reason.
    /// - May fail if newly detected files in directory fixture cannot be tracked
    ///   for whatever reason.
    ///
    /// # See also
    ///
    /// - [`FileFixture::sync`]
    pub fn sync(&mut self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter_mut() {
            fixture.sync()?;
        }

        // Track any new files that were added by some external process(es)...
        for entry in WalkDir::new(self.dir.path()).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() && !self.fixtures.contains_key(entry.path()) {
                let path = entry.path().to_path_buf();
                let data = read_to_string(&path)?;
                let kind = match path.is_executable() {
                    true => FixtureKind::ScriptFile,
                    false => FixtureKind::NormalFile,
                };
                let fixture = Fixture::new(path).with_data(data).with_kind(kind);
                self.fixtures.insert(fixture.as_path().into(), fixture);
            }
        }

        Ok(())
    }

    pub fn as_path(&self) -> &Path {
        self.dir.path()
    }
}

#[derive(Debug, Default, Clone)]
pub struct FixtureClump {
    fixtures: HashMap<PathBuf, Fixture>,
}

impl FixtureClump {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_fixture<F, P>(mut self, path: P, callback: F) -> Self
    where
        F: FnOnce(Fixture) -> Fixture,
        P: AsRef<Path>,
    {
        let fixture = callback(Fixture::new(path.as_ref()));
        self.fixtures.insert(fixture.as_path().into(), fixture);
        self
    }

    pub fn write_all(&self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter() {
            fixture.write()?;
        }

        Ok(())
    }

    pub fn sync(&mut self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter_mut() {
            fixture.sync()?;
        }

        Ok(())
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Result<&Fixture> {
        self.fixtures.get(path.as_ref())
            .ok_or(anyhow!("Fixture '{}' not in clump", path.as_ref().display()))
    }

    pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut Fixture> {
        self.fixtures.get_mut(path.as_ref())
            .ok_or(anyhow!("Fixture '{}' not in clump", path.as_ref().display()))
    }

    pub fn extend(&mut self, clump: FixtureClump) {
        self.fixtures.extend(clump.fixtures.into_iter());
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
pub struct Fixture {
    path: PathBuf,
    data: String,
    kind: FixtureKind,
}

impl Fixture {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), data: Default::default(), kind: Default::default() }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }

    pub fn with_kind(mut self, kind: FixtureKind) -> Self {
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
        if self.kind == FixtureKind::ScriptFile {
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
        self.kind == FixtureKind::ScriptFile
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
pub enum FixtureKind {
    /// Normal file with read and write permissions.
    #[default]
    NormalFile,

    /// Executable file with read and write permissions.
    ScriptFile,
}
