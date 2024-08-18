// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use rstest::{fixture, rstest};
use std::path::{Path, PathBuf};

use ricer::config::dir::DefaultConfigDirManager;
use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
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
            # This should not be overwritten
            [repos]
            vim = { target_home = true, branch = "main", remote = "origin" }

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
fn read_config_file_catches_inexistent_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_dir_fixture);
    let result = config.read_config_file();
    assert!(matches!(result, Err(RicerError::NoConfigFile { .. })));
}

#[rstest]
fn read_config_file_catches_bad_formatting(bad_config_file_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&bad_config_file_fixture);
    let result = config.read_config_file();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
