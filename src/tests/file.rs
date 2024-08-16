// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use crate::config::file::{ConfigFileManager, DefaultConfigFileManager};
use crate::error::RicerError;

use ricer_test_tools::fakes::FakeConfigDir;

#[fixture]
fn config_file_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            # The following should not be overwritten.
            [repos.vim]
            branch = "main"
            remote = "orign"
            url = "https://github.com/awkless/vim.git"
            target = { home = true, os = "unix", user = "awkless", hostname = "lovelace" }

            # The following should not be overwritten.
            [hooks]
            commit = [
                { pre = "hook.sh", post = "hook.sh", repo = "vim" },
                { pre = "hook.sh" }
            ]
            "#})
        .build()
}

#[fixture]
fn empty_config_file_fixture() -> FakeConfigDir {
    FakeConfigDir::builder().config_file("# This comment should not be overwritten\n").build()
}

#[rstest]
fn read_parses_config_file_correctly(config_file_fixture: FakeConfigDir) {
    let cfg_stub = config_file_fixture.config_file_stub();
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(cfg_stub.as_path()).expect("Expect succes");

    let expect = cfg_stub.data();
    let result = cfg_file_mgr.to_string();
    assert_eq!(expect, result);
}

#[test]
fn read_catches_inexistent_config_file() {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    let result = cfg_file_mgr.read("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
