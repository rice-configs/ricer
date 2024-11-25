// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use git2::{RepositoryInitOptions, Repository};
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
    repos: HashMap<PathBuf, RepoFixture>,
}

impl FixtureHarness {
    pub fn open() -> Result<Self> {
        let root = TempFileBuilder::new().tempdir()?;
        Ok(Self { root, fixtures: HashMap::new(), repos: HashMap::new() })
    }

    pub fn with_file(
        mut self,
        path: impl AsRef<Path>,
        callback: impl FnOnce(FileFixture) -> FileFixture,
    ) -> Self {
        let fixture = callback(FileFixture::new(self.root.path().join(path.as_ref())));
        self.fixtures.insert(fixture.as_path().into(), fixture);
        self
    }

    pub fn with_repo(
        mut self,
        path: impl AsRef<Path>,
        callback: impl FnOnce(RepoFixture) -> Result<RepoFixture>,
    ) -> Result<Self> {
        let fixture = callback(RepoFixture::init(self.root.path().join(path.as_ref()))?)?;
        self.repos.insert(fixture.as_path().into(), fixture);
        Ok(self)
    }

    pub fn with_fake_bare_repo(
        mut self,
        path: impl AsRef<Path>,
        callback: impl FnOnce(RepoFixture) -> Result<RepoFixture>,
    ) -> Result<Self> {
        let fixture = callback(RepoFixture::init_fake_bare(
            self.root.path().join(path.as_ref()), self.root.path()
        )?)?;
        self.repos.insert(fixture.as_path().into(), fixture);
        Ok(self)
    }

    pub fn get_file(&self, path: impl AsRef<Path>) -> Result<&FileFixture> {
        self.fixtures
            .get(&self.root.path().join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in fixture harness", path.as_ref().display()))
    }

    pub fn get_file_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut FileFixture> {
        self.fixtures
            .get_mut(&self.root.path().join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in fixture harness", path.as_ref().display()))
    }

    pub fn get_repo(&self, path: impl AsRef<Path>) -> Result<&RepoFixture> {
        self.repos
            .get(&self.root.path().join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in fixture harness", path.as_ref().display()))
    }

    pub fn get_repo_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut RepoFixture> {
        self.repos
            .get_mut(&self.root.path().join(path.as_ref()))
            .ok_or(anyhow!("Fixture '{}' not in fixture harness", path.as_ref().display()))
    }

    pub fn setup(self) -> Result<Self> {
        for (_, fixture) in self.fixtures.iter() {
            fixture.write()?;
        }

        for (_, fixture) in self.repos.iter() {
            fixture.commit("inital commit\n\nbody")?;
        }

        Ok(self)
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

pub struct RepoFixture {
    root: PathBuf,
    repo: Repository,
}

impl RepoFixture {
    pub fn init(path: impl Into<PathBuf>) -> Result<Self> {
        let root = path.into();
        let repo = Repository::init(&root)?;
        Ok(Self { root, repo })
    }

    pub fn init_fake_bare(gitdir: impl Into<PathBuf>, workdir: impl AsRef<Path>) -> Result<Self> {
        let root = gitdir.into();
        let mut opts = RepositoryInitOptions::new();
        opts.bare(false);
        opts.no_dotgit_dir(true);
        opts.workdir_path(workdir.as_ref());

        let repo = Repository::init_opts(&root, &opts)?;
        Ok(Self { root, repo })
    }

    pub fn stage(self, path: impl AsRef<Path>, data: impl AsRef<str>) -> Result<Self> {
        let full_path = self.repo.workdir().unwrap().join(path.as_ref());
        mkdirp(full_path.parent().unwrap())?;
        write(&full_path, data.as_ref())?;

        let mut index = self.repo.index()?;
        index.add_path(path.as_ref())?;
        index.write()?;

        Ok(self)
    }

    pub fn commit(&self, msg: impl AsRef<str>) -> Result<()> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let sig = self.repo.signature()?;
        let mut parents = Vec::new();

        if let Some(parent) = self.repo.head().ok().map(|h| h.target().unwrap()) {
            parents.push(self.repo.find_commit(parent)?);
        }
        let parents = parents.iter().collect::<Vec<_>>();

        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            msg.as_ref(),
            &self.repo.find_tree(tree_id).expect("Failed to find tree"),
            &parents,
        )?;

        Ok(())
    }

    pub fn as_path(&self) -> &Path {
        self.root.as_path()
    }
}
