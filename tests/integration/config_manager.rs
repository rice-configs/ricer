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

#[rstest]
fn read_config_file_catches_bad_formatting(bad_config_file_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&bad_config_file_fixture);
    let result = config.read_config_file();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
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
fn read_config_file_creates_empty_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    assert!(empty_config_dir_fixture.root_dir().join("config.toml").exists());
}

#[rstest]
fn write_config_file_writes_correctly(mut config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    let entry = RepoEntry::builder("st")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/st.git")
        .build();
    config.read_config_file().expect("Expect success");
    config.add_repo(&entry).expect("Expect success");
    config.write_config_file().expect("Expect success");
    config_dir_fixture.sync_files();
    let expect = config_dir_fixture.config_file_stub().data();
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn write_config_file_creates_config_file(empty_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&empty_config_dir_fixture);
    let entry = RepoEntry::builder("st")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/st.git")
        .build();
    config.add_repo(&entry).expect("Expect success");
    config.write_config_file().expect("Expect success");
    config.read_config_file().expect("Expect success");
    let expect = indoc! {r#"
        [repos.st]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/st.git"
        "#};
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
    assert!(empty_config_dir_fixture.root_dir().join("config.toml").exists());
}

#[rstest]
fn add_repo_adds_repo_and_preserves_formatting(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let entry = RepoEntry::builder("st")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/st.git")
        .build();
    config.add_repo(&entry).expect("Expect success");
    let expect = indoc! {r#"
        # This should not be overwritten.
        [repos.vim]
        branch = "main"
        remote = "origin"
        url = "https://github.com/awkless/vim.git"
        target = { home = true, os = "any", user = "awkless", hostname = "lovelace" }

        [repos.st]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/st.git"

        # This should not be overwritten.
        [hooks]
        commit = [
            { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh", post = "hook.sh" },
            { post = "hook.sh" }
        ]
        "#};
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
    assert!(config_dir_fixture.repos_dir().join("st.git").exists());
}

#[rstest]
fn add_repo_does_not_fail_if_repo_dir_exists(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let entry = RepoEntry::builder("dmenu")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/st.git")
        .build();
    config.add_repo(&entry).expect("Expect success");
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

        [repos.dmenu]
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
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
}

#[rstest]
fn get_repo_gets_correct_repo_data(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let expect_path = config_dir_fixture.git_repo_stub("vim").as_path();
    let target = RepoTargetEntry::builder()
        .home(true)
        .os(TargetOsOption::Any)
        .user("awkless")
        .hostname("lovelace")
        .build();
    let expect_entry = RepoEntry::builder("vim")
        .branch("main")
        .remote("origin")
        .url("https://github.com/awkless/vim.git")
        .target(target)
        .build();
    let (result_path, result_entry) = config.get_repo("vim").expect("Expect success");
    assert_eq!(expect_path, result_path);
    assert_eq!(expect_entry, result_entry);
}

#[rstest]
fn get_repo_catches_inexistent_repo_entry(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.get_repo("dmenu");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn get_repo_catches_inexistent_repo_dir(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.get_repo("dwm");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn remove_repo_removes_repo_data_correctly(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    config.remove_repo("vim").expect("Expect success");
    let expect = indoc! {r#"

        # This should not be overwritten.
        [hooks]
        commit = [
            { pre = "hooks.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh", post = "hook.sh" },
            { post = "hook.sh" }
        ]
        "#};
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
    assert!(!config_dir_fixture.repos_dir().join("vim.git").exists());
}

#[rstest]
fn remove_repo_does_not_file_for_inexistent_repo_entry(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.remove_repo("dmenu");
    assert!(result.is_ok());
}

#[rstest]
fn remove_repo_does_not_file_for_inexistent_repo_dir(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.remove_repo("dwm");
    assert!(result.is_ok());
}

#[rstest]
fn rename_repo_renames_repo_data_correctly(config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&config_dir_fixture);
    config.read_config_file().expect("Expect success");
    config.rename_repo("vim", "vimrc").expect("Expect success");
    let expect = indoc! {r#"
        # This should not be overwritten.
        [repos.vimrc]
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
        "#};
    let result = config.file_manager().to_string();
    assert_eq!(expect, result);
    assert!(config_dir_fixture.repos_dir().join("vimrc.git").exists());
    assert!(!config_dir_fixture.repos_dir().join("vim.git").exists());
}

#[rstest]
fn rename_repo_catches_inexistent_repo_entry(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.rename_repo("dmenu", "fail");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn rename_repo_catches_inexistent_repo_dir(desynced_config_dir_fixture: FakeConfigDir) {
    let mut config = setup_config_manager(&desynced_config_dir_fixture);
    config.read_config_file().expect("Expect success");
    let result = config.rename_repo("dwm", "fail");
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
