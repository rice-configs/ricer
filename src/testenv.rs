// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use git2::{IndexAddOption, Repository, RepositoryInitOptions};
use is_executable::IsExecutable;
use mkdirp::mkdirp;
use std::{
    collections::HashMap,
    fs::{metadata, read_to_string, set_permissions, write},
    path::{Path, PathBuf},
};
use tempfile::{Builder as TempFileBuilder, TempDir};
use walkdir::WalkDir;

pub struct FixtureHarness {
    dir: TempDir,
    fixtures: FixtureClump,
}

impl FixtureHarness {
    pub fn open() -> Result<Self> {
        let dir = TempFileBuilder::new().tempdir()?;
        Ok(Self { dir, fixtures: FixtureClump::default() })
    }

    pub fn with_file_set<F>(mut self, callback: F) -> Self
    where
        F: FnOnce(FixtureClump) -> FixtureClump,
    {
        self.fixtures = callback(FixtureClump::new(self.dir.path()));
        self
    }

    pub fn setup(self) -> Result<Self> {
        self.fixtures.write_all()?;
        Ok(self)
    }

    pub fn get_fixture(&self, name: impl AsRef<Path>) -> Result<&Fixture> {
        self.fixtures.get(name.as_ref())
    }

    pub fn get_fixture_mut(&mut self, name: impl AsRef<Path>) -> Result<&mut Fixture> {
        self.fixtures.get_mut(name.as_ref())
    }

    pub fn sync_all(&mut self) -> Result<()> {
        self.fixtures.sync_all()?;

        // Track any new files that were added by some external process(es)...
        for entry in WalkDir::new(self.dir.path()).into_iter().filter_map(Result::ok) {
            // INVARIANT: only add in a _file_ that is not being tracked.
            if entry.file_type().is_file() && !self.fixtures.is_tracked(entry.path()) {
                let path = entry.path().to_path_buf();
                let data = read_to_string(&path)?;
                let kind = match path.is_executable() {
                    true => FixtureKind::ScriptFile,
                    false => FixtureKind::NormalFile,
                };
                let fixture = Fixture::new(path).with_data(data).with_kind(kind);
                self.fixtures.insert(fixture);
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
    root: PathBuf,
    fixtures: HashMap<PathBuf, Fixture>,
}

impl FixtureClump {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { root: path.into(), fixtures: HashMap::new() }
    }

    pub fn with_fixture<F, P>(mut self, path: P, callback: F) -> Self
    where
        F: FnOnce(Fixture) -> Fixture,
        P: AsRef<Path>,
    {
        let fixture = callback(Fixture::new(self.root.join(path.as_ref())));
        self.fixtures.insert(fixture.as_path().into(), fixture);
        self
    }

    pub fn insert(&mut self, fixture: Fixture) {
        self.fixtures.insert(fixture.as_path().into(), fixture);
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Result<&Fixture> {
        self.fixtures
            .get(&self.root.join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in clump", path.as_ref().display()))
    }

    pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut Fixture> {
        self.fixtures
            .get_mut(&self.root.join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in clump", path.as_ref().display()))
    }

    pub fn write_all(&self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter() {
            fixture.write()?;
        }

        Ok(())
    }

    pub fn sync_all(&mut self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter_mut() {
            fixture.sync()?;
        }

        Ok(())
    }

    pub fn is_tracked(&self, path: impl AsRef<Path>) -> bool {
        self.fixtures.contains_key(path.as_ref())
    }
}

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

    pub fn sync(&mut self) -> Result<()> {
        self.data = read_to_string(&self.path)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FixtureKind {
    #[default]
    NormalFile,

    ScriptFile,
}
