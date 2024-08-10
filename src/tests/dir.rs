// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use ricer_tester::fakes::FakeConfigDir;

use crate::config::locator::MockConfigDirLocator;
use crate::config::dir::*;
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
fn config_file_path_gets_correct_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let expect = full_config_dir_fixture.path_to_config_file().as_path();
    let result = cfg_dir_mgr.config_file_path().expect("Expect success");
    assert_eq!(result, expect);
}

#[rstest]
fn config_file_path_returns_absolute_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let result = cfg_dir_mgr.config_file_path().expect("Expect success");
    assert!(result.is_absolute());
}

#[rstest]
fn config_file_path_catches_inexistent_path(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let result = cfg_dir_mgr.config_file_path();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn git_repo_path_finds_git_repo(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_git_repo("vim").as_path();
    let result = cfg_dir_mgr.git_repo_path("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn git_repo_path_cannot_git_repo_path(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.git_repo_path("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn find_hook_script_finds_hook_script(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_hook_script("hook.sh").as_path();
    let result = cfg_dir_mgr.find_hook_script("hook.sh").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn find_hook_script_cannot_find_hook_script(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.find_hook_script("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn find_ignore_file_finds_ignore_file(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_ignore_file("vim").as_path();
    let result = cfg_dir_mgr.find_ignore_file("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn find_ignore_file_cannot_find_ignore_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.find_ignore_file("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
