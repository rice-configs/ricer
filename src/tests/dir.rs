// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use ricer_tester::fakes::FakeConfigDir;

use crate::config::locator::MockConfigDirLocator;
use crate::config::dir::*;
use crate::error::RicerError;

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
fn try_find_config_file_finds_config_file(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_config_file().as_path();
    let result = cfg_dir_mgr.try_find_config_file().expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn try_find_config_file_cannot_find_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.try_find_config_file();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn try_find_git_repo_finds_git_repo(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_git_repo("vim").as_path();
    let result = cfg_dir_mgr.try_find_git_repo("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn try_find_git_repo_cannot_find_git_repo(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.try_find_git_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn try_find_hook_script_finds_hook_script(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_hook_script("hook.sh").as_path();
    let result = cfg_dir_mgr.try_find_hook_script("hook.sh").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn try_find_hook_script_cannot_find_hook_script(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.try_find_hook_script("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn try_find_ignore_file_finds_ignore_file(full_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator.expect_config_dir().return_const(full_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let expect = full_config_dir_fixture.path_to_ignore_file("vim").as_path();
    let result = cfg_dir_mgr.try_find_ignore_file("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn try_find_ignore_file_cannot_find_ignore_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut mock_locator = MockConfigDirLocator::new();
    mock_locator
        .expect_config_dir()
        .return_const(empty_config_dir_fixture.root_dir().to_path_buf());

    let cfg_dir_mgr = DefaultConfigDirManager::new(&mock_locator);
    let result = cfg_dir_mgr.try_find_ignore_file("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
