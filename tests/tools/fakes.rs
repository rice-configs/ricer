// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use log::debug;
use std::fs::remove_dir_all;
use std::path::Path;
use tempfile::{Builder, TempDir};

use ricer_core::config::ConfigDir;

pub struct FakeConfigDir {
    base_dir: TempDir,
    hooks_dir: TempDir,
    repos_dir: TempDir,
    ignores_dir: TempDir,
}

impl FakeConfigDir {
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

        debug!("Fake configuration base directory '{}'", base_dir.path().display());
        debug!("Fake hooks directory '{}'", hooks_dir.path().display());
        debug!("Fake repos directory '{}'", repos_dir.path().display());
        debug!("Fake ignores directory '{}'", ignores_dir.path().display());
        Self { base_dir, hooks_dir, repos_dir, ignores_dir }
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
        debug!("Remove '{}'", self.ignores_dir.path().display());
        remove_dir_all(self.ignores_dir.path()).expect("Failed to close 'ignores/' fixture");

        debug!("Remove '{}'", self.repos_dir.path().display());
        remove_dir_all(self.repos_dir.path()).expect("Failed to close 'repos/' fixture");

        debug!("Remove '{}'", self.hooks_dir.path().display());
        remove_dir_all(self.hooks_dir.path()).expect("Failed to close 'hooks/' fixture");

        debug!("Remove '{}'", self.base_dir.path().display());
        remove_dir_all(self.base_dir.path()).expect("Failed to close base directory fixture");
    }
}
