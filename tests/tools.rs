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
    // TODO...
}
