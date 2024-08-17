// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use crate::config::file::hooks_section::{CommandHookEntry, HookEntry};
use crate::config::file::repos_section::{RepoEntry, RepoTargetEntry, TargetOsOption};
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
            remote = "origin"
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
fn read_deserializes_config_file_correctly(config_file_fixture: FakeConfigDir) {
    let cfg_file_stub = config_file_fixture.config_file_stub();
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(cfg_file_stub.as_path()).expect("Expect succes");

    let expect = cfg_file_stub.data();
    let result = cfg_file_mgr.to_string();
    assert_eq!(expect, result);
}

#[test]
fn read_catches_inexistent_config_file() {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    let result = cfg_file_mgr.read("nonexistant");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn write_serializes_config_file_correctly(mut config_file_fixture: FakeConfigDir) {
    let cfg_file_stub = config_file_fixture.config_file_stub_mut();
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.add_repo(&new_repo).expect("Expect success");
    cfg_file_mgr.write(cfg_file_stub.as_path()).expect("Expect success");
    cfg_file_stub.sync();
    let expect = cfg_file_stub.data();
    let result = cfg_file_mgr.to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn add_repo_serializes_correctly_to_existing_repos_table(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    cfg_file_mgr.add_repo(&new_repo).expect("Expect success");
    let expect = indoc! {r#"
            # The following should not be overwritten.
            [repos.vim]
            branch = "main"
            remote = "origin"
            url = "https://github.com/awkless/vim.git"
            target = { home = true, os = "unix", user = "awkless", hostname = "lovelace" }

            [repos.dwm]
            branch = "master"
            remote = "upstream"
            url = "https://github.com/awkless/dwm.git"

            # The following should not be overwritten.
            [hooks]
            commit = [
                { pre = "hook.sh", post = "hook.sh", repo = "vim" },
                { pre = "hook.sh" }
            ]
        "#};
    let result = cfg_file_mgr.to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn get_repo_deserializes_correctly(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let target = RepoTargetEntry::builder()
        .home(true)
        .os(TargetOsOption::Unix)
        .user("awkless")
        .hostname("lovelace")
        .build();
    let expect = RepoEntry::builder("vim")
        .branch("main")
        .remote("origin")
        .url("https://github.com/awkless/vim.git")
        .target(target)
        .build();
    let result = cfg_file_mgr.get_repo("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn get_repo_catches_inexistent_repo(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let result = cfg_file_mgr.get_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::NoRepoFound { .. })));
}

#[rstest]
fn get_repo_catches_no_repos_section(empty_config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr
        .read(empty_config_file_fixture.config_file_stub().as_path())
        .expect("Expect success");
    let result = cfg_file_mgr.get_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::NoReposSection)));
}

#[rstest]
fn get_repo_catches_non_table_repos_section(non_table_sections_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr
        .read(non_table_sections_fixture.config_file_stub().as_path())
        .expect("Expect success");
    let result = cfg_file_mgr.get_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::ReposSectionNotTable)));
}

#[rstest]
fn add_repo_serializes_corretly_with_no_repos_table(empty_config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr
        .read(empty_config_file_fixture.config_file_stub().as_path())
        .expect("Expect success");
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    cfg_file_mgr.add_repo(&new_repo).expect("Expect success");
    let expect = indoc! {r#"
        [repos.dwm]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/dwm.git"
        # This comment should not be overwritten
        "#};
    let result = cfg_file_mgr.to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn add_repo_catches_non_table_repos_section(non_table_sections_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr
        .read(non_table_sections_fixture.config_file_stub().as_path())
        .expect("Expect success");
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    let result = cfg_file_mgr.add_repo(&new_repo);
    assert!(matches!(result, Err(RicerError::ReposSectionNotTable)));
}

#[rstest]
fn remove_repo_removes_repo_from_toml_data(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let expect = indoc! {r#"

        # The following should not be overwritten.
        [hooks]
        commit = [
            { pre = "hook.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh" }
        ]
    "#};
    cfg_file_mgr.remove_repo("vim").expect("Expect success");
    let result = cfg_file_mgr.to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn remove_repo_provides_correct_repo_data(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let target = RepoTargetEntry::builder()
        .home(true)
        .os(TargetOsOption::Unix)
        .user("awkless")
        .hostname("lovelace")
        .build();
    let expect = RepoEntry::builder("vim")
        .branch("main")
        .remote("origin")
        .url("https://github.com/awkless/vim.git")
        .target(target)
        .build();
    let result = cfg_file_mgr.remove_repo("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn remove_repo_catches_no_repos_section(empty_config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr
        .read(empty_config_file_fixture.config_file_stub().as_path())
        .expect("Expect success");
    let result = cfg_file_mgr.remove_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::NoReposSection)));
}

#[rstest]
fn remove_repo_catches_non_table_repos_section(non_table_sections_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr
        .read(non_table_sections_fixture.config_file_stub().as_path())
        .expect("Expect success");
    let result = cfg_file_mgr.remove_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::ReposSectionNotTable)));
}

#[rstest]
fn remove_repo_catches_inexistent_repo(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let result = cfg_file_mgr.remove_repo("nonexistant");
    assert!(matches!(result, Err(RicerError::NoRepoFound { .. })));
}

#[rstest]
fn get_cmd_hook_deserializes_correctly(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let mut expect = CommandHookEntry::new("commit");
    let full_hook_entry = HookEntry::builder().pre("hook.sh").post("hook.sh").repo("vim").build();
    let small_hook_entry = HookEntry::builder().pre("hook.sh").build();
    expect.add_hook(full_hook_entry);
    expect.add_hook(small_hook_entry);
    let result = cfg_file_mgr.get_cmd_hook("commit").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn get_cmd_hook_catches_catches_no_hooks_section(empty_config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(empty_config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let result = cfg_file_mgr.get_cmd_hook("nonexistant");
    assert!(matches!(result, Err(RicerError::NoHooksSection)));
}

#[rstest]
fn get_cmd_hook_catches_non_table_hooks_section(non_table_sections_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(non_table_sections_fixture.config_file_stub().as_path()).expect("Expect success");
    let result = cfg_file_mgr.get_cmd_hook("nonexistant");
    assert!(matches!(result, Err(RicerError::HooksSectionNotTable)));
}

#[rstest]
fn get_cmd_hook_catches_inexistent_cmd_hook(config_file_fixture: FakeConfigDir) {
    let mut cfg_file_mgr = DefaultConfigFileManager::new();
    cfg_file_mgr.read(config_file_fixture.config_file_stub().as_path()).expect("Expect success");
    let result = cfg_file_mgr.get_cmd_hook("nonexistant");
    assert!(matches!(result, Err(RicerError::NoHookFound { .. })));
}
