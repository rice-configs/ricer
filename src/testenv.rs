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
    root: TempDir,
    fixtures: HashMap<PathBuf, FileFixture>,
}

impl FixtureHarness {
    pub fn open() -> Result<Self> {
        let root = TempFileBuilder::new().tempdir()?;
        Ok(Self { root, fixtures: HashMap::new() })
    }

    pub fn with_fixture(
        mut self,
        path: impl AsRef<Path>,
        callback: impl FnOnce(FileFixture) -> FileFixture
    ) -> Self {
        let fixture = callback(FileFixture::new(self.root.path().join(path.as_ref())));
        self.fixtures.insert(fixture.as_path().into(), fixture);
        self
    }

    pub fn setup(self) -> Result<Self> {
        for (_, fixture) in self.fixtures.iter() {
            fixture.write()?;
        }
        Ok(self)
    }

    pub fn get_fixture(&self, path: impl AsRef<Path>) -> Result<&FileFixture> {
        self.fixtures
            .get(&self.root.path().join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in fixture harness", path.as_ref().display()))
    }

    pub fn get_fixture_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut FileFixture> {
        self.fixtures
            .get_mut(&self.root.path().join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in fixture harness", path.as_ref().display()))
    }

    pub fn sync_all(&mut self) -> Result<()> {
        for (_, fixture) in self.fixtures.iter_mut() {
            fixture.sync()?;
        }

        // Track any new files that were added by some external process(es)...
        for entry in WalkDir::new(self.root.path()).into_iter().filter_map(Result::ok) {
            // INVARIANT: only add in a _file_ that is not being tracked.
            if entry.file_type().is_file() && !self.fixtures.contains_key(entry.path()) {
                let path = entry.path().to_path_buf();
                let data = read_to_string(&path)?;
                let kind = match path.is_executable() {
                    true => FileKind::Script,
                    false => FileKind::Normal,
                };
                let fixture = FileFixture::new(path.clone()).with_data(data).with_kind(kind);
                self.fixtures.insert(path, fixture);
            }
        }

        Ok(())
    }

    pub fn as_path(&self) -> &Path {
        self.root.path()
    }
}

#[derive(Debug, Default, Clone)]
pub struct FileFixture {
    path: PathBuf,
    data: String,
    kind: FileKind,
}

impl FileFixture {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), data: Default::default(), kind: Default::default() }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }

    pub fn with_kind(mut self, kind: FileKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn write(&self) -> Result<()> {
        mkdirp(self.path.parent().unwrap())?;
        write(&self.path, &self.data)?;

        #[cfg(unix)]
        if self.kind == FileKind::Script {
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
        self.kind == FileKind::Script
    }

    pub fn sync(&mut self) -> Result<()> {
        self.data = read_to_string(&self.path)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FileKind {
    #[default]
    Normal,

    Script,
}
