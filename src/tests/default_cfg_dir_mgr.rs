// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use ricer_test_tools::fakes::FakeConfigDir;

use crate::config::dir::*;
use crate::config::locator::MockConfigDirLocator;
use crate::error::RicerError;

fn setup_cfg_dir_mgr(fake_dir: &FakeConfigDir) -> DefaultConfigDirManager {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(fake_dir.root_dir().to_path_buf());
    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    cfg_dir_mgr
}

#[fixture]
fn full_config_dir_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file("fake it till you make it!")
        .git_repo("vim")
        .hook_script("hook.sh", "fake it till you make it!")
        .ignore_file("vim", "fake it till you make it!")
        .build()
}

#[fixture]
fn empty_config_dir_fixture() -> FakeConfigDir {
    FakeConfigDir::builder().build()
}

#[rstest]
fn setup_config_file_creates_root_dir_and_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let expect = empty_config_dir_fixture.root_dir().join("config.toml");
    let result = cfg_dir_mgr.setup_config_file().expect("Expect success");
    assert_eq!(expect, result);
    assert!(expect.exists());
}

#[rstest]
fn setup_config_file_does_not_fail_if_config_file_exists(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let expect = full_config_dir_fixture.config_file_stub().as_path();
    let result = cfg_dir_mgr.setup_config_file().expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn add_repo_adds_new_entry_into_repos(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let expect = empty_config_dir_fixture.repos_dir().join("dwm.git");
    let result = cfg_dir_mgr.add_repo("dwm").expect("Expect success");
    assert_eq!(expect, result);
    assert!(expect.exists());
}

#[rstest]
fn add_repo_does_not_fail_if_repo_exists(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let expect = full_config_dir_fixture.git_repo_stub("vim").as_path();
    let result = cfg_dir_mgr.add_repo("vim").expect("Expect success");
    assert_eq!(expect, result);
    assert!(expect.exists());
}

#[rstest]
fn hook_script_path_gets_correct_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let expect = full_config_dir_fixture.hook_script_stub("hook.sh").as_path();
    let result = cfg_dir_mgr.hook_script_path("hook.sh").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn hook_script_path_returns_absolute_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let result = cfg_dir_mgr.hook_script_path("hook.sh").expect("Expect success");
    assert!(result.is_absolute());
}

#[rstest]
fn hook_script_path_catches_inexistent_path(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let result = cfg_dir_mgr.hook_script_path("nonexistant");
    assert!(matches!(result, Err(RicerError::NoHookScript { .. })));
}

#[rstest]
fn ignore_file_path_gets_correct_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let expect = full_config_dir_fixture.ignore_file_stub("vim").as_path();
    let result = cfg_dir_mgr.ignore_file_path("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn ignore_file_path_returns_absolute_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let result = cfg_dir_mgr.ignore_file_path("vim").expect("Expect success");
    assert!(result.is_absolute());
}

#[rstest]
fn ignore_file_path_catches_inexistent_path(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let result = cfg_dir_mgr.ignore_file_path("nonexistant");
    assert!(matches!(result, Err(RicerError::NoIgnoreFile { .. })));
}
