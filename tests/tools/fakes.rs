// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use std::fs::remove_dir_all;
use std::path::{PathBuf, Path};
use tempfile::{Builder, TempDir};
use std::collections::HashMap;

use ricer_core::config::ConfigDir;

use crate::tools::stubs::FileStub;

#[derive(Debug)]
pub struct FakeConfigDir {
    base_dir: TempDir,
    hooks_dir: TempDir,
    repos_dir: TempDir,
    ignores_dir: TempDir,
    stub_files: HashMap<PathBuf, FileStub>,
}

impl FakeConfigDir {
    pub fn builder() -> FakeConfigDirBuilder {
        FakeConfigDirBuilder::new()
    }

    }
}

impl ConfigDir for FakeConfigDir {
    fn base_dir(&self) -> &Path {
        self.base_dir.path()
    }

    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.path()
    }

    fn repos_dir(&self) -> &Path {
        self.repos_dir.path()
    }

    fn ignores_dir(&self) -> &Path {
        self.ignores_dir.path()
    }
}

impl Drop for FakeConfigDir {
    fn drop(&mut self) {
        self.stub_files.clear();
        remove_dir_all(self.ignores_dir.path()).expect("Failed to close 'ignores/' fixture");
        remove_dir_all(self.repos_dir.path()).expect("Failed to close 'repos/' fixture");
        remove_dir_all(self.hooks_dir.path()).expect("Failed to close 'hooks/' fixture");
        remove_dir_all(self.base_dir.path()).expect("Failed to close base directory fixture");
    }
}

#[derive(Debug)]
pub struct FakeConfigDirBuilder {
    base_dir: TempDir,
    hooks_dir: TempDir,
    repos_dir: TempDir,
    ignores_dir: TempDir,
    stub_files: HashMap<PathBuf, FileStub>,
}

impl FakeConfigDirBuilder {
    pub fn new() -> Self {
        let base_dir =
            Builder::new().prefix("ricer").tempdir().expect("Failed to create base directory");

        let hooks_dir = Builder::new()
            .prefix("hooks")
            .tempdir_in(base_dir.path())
            .expect("Failed to create 'hooks' directory");

        let repos_dir = Builder::new()
            .prefix("repos")
            .tempdir_in(base_dir.path())
            .expect("Failed to create 'repos' directory");

        let ignores_dir = Builder::new()
            .prefix("ignores")
            .tempdir_in(base_dir.path())
            .expect("Failed to create 'ignores' directory");

        Self { base_dir, hooks_dir, repos_dir, ignores_dir, stub_files: HashMap::default() }
    }

    pub fn ignore_file(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let stub_file = FileStub::builder()
            .path(self.ignores_dir.path().join(format!("{}.ignore", name.as_ref())))
            .data(data.as_ref())
            .executable(false)
            .build();

        self.stub_files.insert(stub_file.as_path().to_path_buf(), stub_file);
        self
    }

    pub fn hook_script(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let stub_file = FileStub::builder()
            .path(self.hooks_dir.path().join(name.as_ref()))
            .data(data.as_ref())
            .executable(true)
            .build();

        self.stub_files.insert(stub_file.as_path().to_path_buf(), stub_file);
        self
    }

    pub fn build(self) -> FakeConfigDir {
        FakeConfigDir { 
            base_dir: self.base_dir,
            hooks_dir: self.hooks_dir,
            repos_dir: self.repos_dir,
            ignores_dir: self.ignores_dir,
            stub_files: self.stub_files
        }
    }
}
