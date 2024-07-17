// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use std::path::Path;
use log::debug;
use std::fs::remove_dir_all;
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
        let base_dir = Builder::new()
            .prefix("ricer")
            .tempdir()
            .expect("Failed to create base directory");

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

        Self { base_dir, hooks_dir, repos_dir, ignores_dir }
    }
}
