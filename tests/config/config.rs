// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use ricer_core::config::Config;
use ricer_core::error::RicerError;

use crate::tools::fakes::FakeConfigDir;

#[fixture]
fn hook_script_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder().hook_script("fake_hook.sh", "chmod +x file").build();
    config_dir
}

#[fixture]
fn git_repo_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder().git_repo("fake_repo").build();
    config_dir
}

#[fixture]
fn ignore_file_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder().ignore_file("fake_repo", "/*").build();
    config_dir
}

#[rstest]
fn try_to_find_hook_gives_correct_path(hook_script_fixture: FakeConfigDir) {
    let expect = hook_script_fixture.path_to_hook_script("fake_hook.sh").as_path().to_path_buf();
    let result = match Config::new(hook_script_fixture).try_to_find_hook("fake_hook.sh") {
        Ok(path) => path,
        Err(error) => panic!("Expect `try_to_find_hook` to succeed, but got: {}", error),
    };

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_hook_no_hook_found(hook_script_fixture: FakeConfigDir) {
    let result = match Config::new(hook_script_fixture).try_to_find_hook("nonexistant.sh") {
        Ok(path) => panic!("Expect `try_to_find_hook` to fail, but got: {}", path.display()),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}

#[rstest]
fn try_to_find_git_repo(git_repo_fixture: FakeConfigDir) {
    let expect = git_repo_fixture.path_to_git_repo("fake_repo").as_path().to_path_buf();
    let result = match Config::new(git_repo_fixture).try_to_find_git_repo("fake_repo") {
        Ok(path) => path,
        Err(error) => panic!("Expect `try_to_find_git_repo` to succeed, but got: {}", error),
    };

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_git_repo_no_repo_found(git_repo_fixture: FakeConfigDir) {
    let result = match Config::new(git_repo_fixture).try_to_find_git_repo("nonexistant") {
        Ok(path) => panic!("Expect `try_to_find_git_repo` to fail, but got: {}", path.display()),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}

#[rstest]
fn try_to_find_ignore_gives_correct_path(ignore_file_fixture: FakeConfigDir) {
    let expect = ignore_file_fixture.path_to_ignore_file("fake_repo").as_path().to_path_buf();
    let result = match Config::new(ignore_file_fixture).try_to_find_ignore("fake_repo") {
        Ok(path) => path,
        Err(error) => panic!("Expect `try_to_find_ignore` to succeed, but got: {}", error),
    };

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_ignore_no_ignore_found(ignore_file_fixture: FakeConfigDir) {
    let result = match Config::new(ignore_file_fixture).try_to_find_ignore("nonexistant") {
        Ok(path) => panic!("Expect `try_to_find_ignore` to fail, but got: {}", path.display()),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}
