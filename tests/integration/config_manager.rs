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
fn empty_config_file_fixture() -> FakeConfigDir {
    FakeConfigDir::builder().config_file("# Nothing in here!").build()
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
    let result = config.file_manager_to_string();
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

#[rstest]
fn write_config_file_writes_to_existing_config_file(mut config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    config.add_git_repo(&new_repo).expect("Expect success");
    config.write_config_file().expect("Expect success");
    config_dir_fixture.sync_files();
    let expect = config_dir_fixture.config_file_stub().data();
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn write_config_file_writes_new_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_dir_fixture);
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    config.add_git_repo(&new_repo).expect("Expect success");
    config.write_config_file().expect("Expect success");
    config.read_config_file().expect("Expect success");
    let expect = indoc! {r#"
        [repos.dwm]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/dwm.git"
    "#};
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn add_git_repo_adds_repo_entry_to_existing_repos_section(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    config.add_git_repo(&new_repo).expect("Expect success");
    let expect = indoc! {r#"
        # This should not be overwritten.
        [repos.vim]
        branch = "main"
        remote = "origin"
        url = "https://github.com/awkless/vim.git"
        target = { home = true, os = "any", user = "awkless", hostname = "lovelace" }

        [repos.dwm]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/dwm.git"

        # This should not be overwritten.
        [hooks]
        commit = [
            { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh", post = "hook.sh" },
            { post = "hook.sh" }
        ]
        "#};
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
    assert!(config_dir_fixture.repos_dir().join("dwm.git").exists());
}

#[rstest]
fn add_git_repo_adds_repo_entry_with_new_repos_section(empty_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_dir_fixture);
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    config.add_git_repo(&new_repo).expect("Expect success");
    let expect = indoc! {r#"
        [repos.dwm]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/dwm.git"
        "#};
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
    assert!(empty_config_dir_fixture.repos_dir().join("dwm.git").exists());
}

#[rstest]
fn add_git_repo_catches_non_table_repos_section(non_table_sections_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&non_table_sections_fixture);
    config.read_config_file().expect("Expect success");
    let new_repo = RepoEntry::builder("dwm")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/dwm.git")
        .build();
    let result = config.add_git_repo(&new_repo);
    assert!(matches!(result, Err(RicerError::ReposSectionNotTable)));
}

#[rstest]
fn add_git_repo_does_not_fail_if_git_repo_dir_exists(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let new_repo = RepoEntry::builder("st")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/st.git")
        .build();
    config.add_git_repo(&new_repo).expect("Expect success");
    let expect = indoc! {r#"
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

        [repos.st]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/st.git"

        # No repo entry for dmenu, but dir does exist!

        [hooks]
        commit = [
            { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh", post = "hook.sh" },
            { post = "hook.sh" }
        ]
        "#};
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
    assert!(desynced_config_dir_fixture.repos_dir().join("st.git").exists());
}

#[rstest]
fn remove_git_repo_removes_all_repo_data(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    config.remove_git_repo("vim").expect("Expect success");
    let expect = indoc! {r#"

        # This should not be overwritten.
        [hooks]
        commit = [
            { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh", post = "hook.sh" },
            { post = "hook.sh" }
        ]
        "#};
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
    assert!(!config_dir_fixture.repos_dir().join("vim.git").exists());
}

#[rstest]
fn remove_git_repo_removes_returns_correct_repo_entry(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let target = RepoTargetEntry::builder()
        .home(true)
        .os(TargetOsOption::Any)
        .user("awkless")
        .hostname("lovelace")
        .build();
    let expect = RepoEntry::builder("vim")
        .branch("main")
        .remote("origin")
        .url("https://github.com/awkless/vim.git")
        .target(target)
        .build();
    let result = config.remove_git_repo("vim").expect("Expect success");
    assert_eq!(expect, result);
}

#[rstest]
fn remove_git_repo_does_not_fail_if_git_repo_dir_inexistent(
    desynced_config_dir_fixture: FakeConfigDir
) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    config.remove_git_repo("dwm").expect("Expect success");
    let expect = indoc! {r#"
            # Entry and dir exists!
            [repos.vim]
            branch = "main"
            remote = "origin"
            url = "https://github.com/awkless/vim.git"
            target = { home = true, os = "any", user = "awkless", hostname = "lovelace" }

            # No repo entry for dmenu, but dir does exist!

            [hooks]
            commit = [
                { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
                { pre = "hook.sh", post = "hook.sh" },
                { post = "hook.sh" }
            ]
        "#};
    let result = config.file_manager_to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn remove_git_repo_catches_inexistent_repo_entry(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.remove_git_repo("dmenu");
    assert!(matches!(result, Err(RicerError::NoRepoFound { .. })));
    assert!(!desynced_config_dir_fixture.repos_dir().join("dmenu.git").exists());
}

#[rstest]
fn remove_git_repo_catches_non_table_repos_section(non_table_sections_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&non_table_sections_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.remove_git_repo("vim");
    assert!(matches!(result, Err(RicerError::ReposSectionNotTable)));
}

#[rstest]
fn remove_git_repo_catches_no_repos_section(empty_config_file_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_file_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.remove_git_repo("fail_here");
    assert!(matches!(result, Err(RicerError::NoReposSection)));
}
