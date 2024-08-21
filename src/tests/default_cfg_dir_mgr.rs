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
fn get_repo_gets_correct_path(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let expect = full_config_dir_fixture.git_repo_stub("vim").as_path();
    let result = cfg_dir_mgr.get_repo("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn get_repo_catches_inexistent_repo(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let result = cfg_dir_mgr.get_repo("nada");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn remove_repo_removes_git_repo(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let result = cfg_dir_mgr.remove_repo("vim");
    assert!(result.is_ok());
    assert!(!full_config_dir_fixture.git_repo_stub("vim").as_path().exists());
}

#[rstest]
fn remove_repo_does_not_fail_if_git_repo_does_not_exist(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let result = cfg_dir_mgr.remove_repo("vim");
    assert!(result.is_ok());
}

#[rstest]
fn rename_repo_renames_git_repo(full_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&full_config_dir_fixture);
    let result = cfg_dir_mgr.rename_repo("vim", "vimrc");
    assert!(result.is_ok());
    assert!(full_config_dir_fixture.repos_dir().join("vimrc.git").exists());
    assert!(!full_config_dir_fixture.repos_dir().join("vim.git").exists());
}

#[rstest]
fn rename_repo_creates_inexistent_repo(empty_config_dir_fixture: FakeConfigDir) {
    let cfg_dir_mgr = setup_cfg_dir_mgr(&empty_config_dir_fixture);
    let result = cfg_dir_mgr.rename_repo("vim", "vimrc");
    assert!(result.is_ok());
    assert!(empty_config_dir_fixture.repos_dir().join("vimrc.git").exists());
}
