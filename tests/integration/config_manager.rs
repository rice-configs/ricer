// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use std::path::{Path, PathBuf};

use ricer::config::dir::DefaultConfigDirManager;
use ricer::config::file::repos_section::{RepoEntry, RepoTargetEntry, TargetOsOption};
use ricer::config::file::DefaultConfigFileManager;
use ricer::config::locator::{DefaultConfigDirLocator, XdgBaseDirSpec};
use ricer::config::ConfigManager;
use ricer::error::RicerError;

use ricer_test_tools::fakes::FakeConfigDir;

struct StubXdgBaseDirSpec {
    stub_path: PathBuf,
}

impl StubXdgBaseDirSpec {
    fn new(fake_dir: &FakeConfigDir) -> Self {
        Self { stub_path: fake_dir.temp_dir().to_path_buf() }
    }
}

impl XdgBaseDirSpec for StubXdgBaseDirSpec {
    fn config_home_dir(&self) -> &Path {
        self.stub_path.as_path()
    }
}

fn setup_config_manager(
    fake_dir: &FakeConfigDir,
) -> ConfigManager<DefaultConfigDirManager, DefaultConfigFileManager> {
    let xdg_spec = StubXdgBaseDirSpec::new(&fake_dir);
    let locator = DefaultConfigDirLocator::new_locate(&xdg_spec).expect("Expect succuess");
    let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    let cfg_file_mgr = DefaultConfigFileManager::new();
    let config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    config
}

#[fixture]
fn config_dir_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            # This should not be overwritten.
            [repos.vim]
            branch = "main"
            remote = "origin"
            url = "https://github.com/awkless/vim.git"
            target = { home = true, os = "any", user = "awkless", hostname = "lovelace" }

            # This should not be overwritten.
            [hooks]
            commit = [
                { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
                { pre = "hook.sh", post = "hook.sh" },
                { post = "hook.sh" }
            ]
            "#})
        .git_repo("vim")
        .build()
}

#[fixture]
fn desynced_config_dir_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            # Entry and dir exists!
            [repos.vim]
            branch = "main"
            remote = "origin"
            url = "https://github.com/awkless/vim.git"
            target = { home = true, os = "any", user = "awkless", hostname = "lovelace" }

            # Entry exists, but not dir!
            [repos.dwm]
            branch = "master"
            remote = "upstream"
            url = "https://github.com/awkless/dwm.git"

            # No repo entry for dmenu, but dir does exist!

            [hooks]
            commit = [
                { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
                { pre = "hook.sh", post = "hook.sh" },
                { post = "hook.sh" }
            ]
            "#})
        .git_repo("vim")
        .git_repo("dmenu")
        .build()
}

#[fixture]
fn bad_config_file_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            [repos.vim]
            branch = "bad # not closed!
            "#})
        .build()
}

#[fixture]
fn empty_config_dir_fixture() -> FakeConfigDir {
    FakeConfigDir::builder().build()
}

#[fixture]
fn non_table_sections_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            repos = "not a table!"
            hooks = "not a table!"
        "#})
        .build()
}

#[rstest]
fn read_config_file_reads_correctly(config_dir_fixture: FakeConfigDir) {
    let cfg_file_stub = config_dir_fixture.config_file_stub();
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");

    let expect = cfg_file_stub.data();
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn read_config_file_catches_bad_formatting(bad_config_file_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&bad_config_file_fixture);
    let result = config.read_config_file();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn read_config_file_creates_empty_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    assert!(empty_config_dir_fixture.root_dir().join("config.toml").exists());
}
