// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use std::collections::HashMap;

use ricer_core::config::file::*;
use ricer_core::config::Config;
use ricer_core::error::RicerError;

use crate::tools::fakes::FakeConfigDir;

#[fixture]
fn bad_config_file_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder()
        .config_file(indoc! {"
            [repos]
            vim = { target_home = false, main_branch = 'main', main_remote = 'main' # Not closed
            "
        })
        .build();

    config_dir
}

#[fixture]
fn no_config_file_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder().build();
    config_dir
}

#[fixture]
fn config_file_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder()
        .config_file(indoc! {"
            [repos]
            vim = { target_home = true, main_branch = 'main', main_remote = 'origin' }
            st = { target_home = false, main_branch = 'master', main_remote = 'origin' }

            [hooks]
            commit = [
                { pre = 'hook.sh', post = 'hook.sh', repo = 'vim' },
                { post = 'hook.sh' }
            ]

            clone = [
                { post = 'hook.sh', repo = 'st' },
                { pre = 'hook.sh' }
            ]
            "
        })
        .build();

    config_dir
}

#[fixture]
fn deserialized_config_file_fixture() -> ConfigFile {
    let mut stub_repos_table = HashMap::new();
    stub_repos_table.insert(
        "vim".to_string(),
        ReposTable { target_home: true, main_branch: "main".into(), main_remote: "origin".into() },
    );

    stub_repos_table.insert(
        "st".to_string(),
        ReposTable {
            target_home: false,
            main_branch: "master".into(),
            main_remote: "origin".into(),
        },
    );

    let stub_hooks_table = HooksTable {
        commit: Some(vec![
            HookConfig {
                pre: Some("hook.sh".into()),
                post: Some("hook.sh".into()),
                repo: Some("vim".into()),
            },
            HookConfig { pre: None, post: Some("hook.sh".into()), repo: None },
        ]),
        push: None,
        pull: None,
        init: None,
        clone: Some(vec![
            HookConfig { pre: None, post: Some("hook.sh".into()), repo: Some("st".into()) },
            HookConfig { pre: Some("hook.sh".into()), post: None, repo: None },
        ]),
        delete: None,
        rename: None,
        status: None,
        list: None,
        enter: None,
    };

    let config = ConfigFile { repos: Some(stub_repos_table), hooks: Some(stub_hooks_table) };
    config
}

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
fn try_to_read_config_file_catches_bad_formatting(bad_config_file_fixture: FakeConfigDir) {
    let mut config = Config::new(bad_config_file_fixture);
    let result = match config.try_to_read_config_file() {
        Ok(_) => panic!("Expect `try_to_read_config_file` to fail, but it somehow did not"),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}

#[rstest]
fn try_to_read_config_file_no_config_file_found(no_config_file_fixture: FakeConfigDir) {
    let mut config = Config::new(no_config_file_fixture);
    let result = match config.try_to_read_config_file() {
        Ok(_) => panic!("Expect `try_to_read_config_file` to fail, but it somehow did not"),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}

#[rstest]
fn try_to_read_config_file_deserializes_correctly(
    config_file_fixture: FakeConfigDir,
    deserialized_config_file_fixture: ConfigFile,
) {
    let expect = deserialized_config_file_fixture;
    let mut config = Config::new(config_file_fixture);

    config.try_to_read_config_file().expect("Failed to read configuration file");
    let result = config.file;

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_hook_script_gives_correct_path(hook_script_fixture: FakeConfigDir) {
    let expect = hook_script_fixture.path_to_hook_script("fake_hook.sh").as_path().to_path_buf();
    let result = match Config::new(hook_script_fixture).try_to_find_hook_script("fake_hook.sh") {
        Ok(path) => path,
        Err(error) => panic!("Expect `try_to_find_hook_script` to succeed, but got: {}", error),
    };

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_hook_script_no_hook_found(hook_script_fixture: FakeConfigDir) {
    let result = match Config::new(hook_script_fixture).try_to_find_hook_script("nonexistant.sh") {
        Ok(path) => panic!("Expect `try_to_find_hook_script` to fail, but got: {}", path.display()),
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
fn try_to_find_ignore_file_gives_correct_path(ignore_file_fixture: FakeConfigDir) {
    let expect = ignore_file_fixture.path_to_ignore_file("fake_repo").as_path().to_path_buf();
    let result = match Config::new(ignore_file_fixture).try_to_find_ignore_file("fake_repo") {
        Ok(path) => path,
        Err(error) => panic!("Expect `try_to_find_ignore_file` to succeed, but got: {}", error),
    };

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_ignore_file_no_ignore_found(ignore_file_fixture: FakeConfigDir) {
    let result = match Config::new(ignore_file_fixture).try_to_find_ignore_file("nonexistant") {
        Ok(path) => panic!("Expect `try_to_find_ignore_file` to fail, but got: {}", path.display()),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}
