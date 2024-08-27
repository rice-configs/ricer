// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use rstest::{fixture, rstest};
use std::ffi::OsString;
use std::path::{PathBuf, Path};

use ricer_test_tools::fakes::FakeConfigDir;

use ricer::cli::RicerCli;
use ricer::config::dir::DefaultConfigDirManager;
use ricer::config::file::DefaultConfigFileManager;
use ricer::config::ConfigManager;
use ricer::config::locator::{DefaultConfigDirLocator, XdgBaseDirSpec};
use ricer::context::Context;
use ricer::error::RicerError;
use ricer::hook::{CommandHookManager, DefaultCommandHookManager};

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
    let mut config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    config.read_config_file().expect("Failed to read configuration file");
    config
}

fn setup_context(args: impl AsRef<str>) -> Context {
    let args: Vec<OsString> = args.as_ref().split_whitespace().map(|s| OsString::from(s)).collect();
    let opts = RicerCli::parse_args(args);
    Context::from(opts)
}

#[fixture]
fn good_hook_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            [repos.vim]
            branch = "main"
            remote = "origin"
            url = "https://github.com/awkless/vim.git"

            [hooks]
            commit = [
                { pre = "hook.sh" },
                { post = "hook.sh" },
                { pre = "hook.sh", repo = "vim" },
                { post = "hook.sh", repo = "vim" },
            ]
            "#
        })
        .hook_script(
            "hook.sh",
            indoc! {r#"
            #!/bin/sh
            echo "Will succeed!"
            "#
            },
        )
        .git_repo("vim")
        .build()
}

#[fixture]
fn no_hook_script_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            [hooks]
            commit = [
                # Hook script does not exist!
                { pre = "hook.sh" },
                { post = "hook.sh" },
            ]
            "#
        })
        .build()
}

#[fixture]
fn no_repo_fixture() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(indoc! {r#"
            [hooks]
            commit = [
                # Repository does not exist!
                { pre = "hook.sh", repo = "nonexistent" },
                { post = "hook.sh", repo = "nonexistent" },
            ]
            "#
        })
        .build()
}

#[fixture]
fn empty_config_file_fixture() -> FakeConfigDir {
    FakeConfigDir::builder().config_file("# No hook entries").build()
}

#[rstest]
fn run_pre_does_fail_for_no_cmd_hook(empty_config_file_fixture: FakeConfigDir) {
    let config = setup_config_manager(&empty_config_file_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_pre();
    assert!(result.is_ok());
}

#[rstest]
fn run_pre_catches_inexistent_hook_script(no_hook_script_fixture: FakeConfigDir) {
    let config = setup_config_manager(&no_hook_script_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_pre();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn run_pre_catches_inexistent_repo(no_repo_fixture: FakeConfigDir) {
    let config = setup_config_manager(&no_repo_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_pre();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn run_pre_does_not_fail_for_valid_pre_hook_setup(good_hook_fixture: FakeConfigDir) {
    let config = setup_config_manager(&good_hook_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_pre();
    assert!(result.is_ok());
}

#[rstest]
fn run_post_does_fail_for_no_cmd_hook(empty_config_file_fixture: FakeConfigDir) {
    let config = setup_config_manager(&empty_config_file_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_post();
    assert!(result.is_ok());
}

#[rstest]
fn run_post_catches_inexistent_hook_script(no_hook_script_fixture: FakeConfigDir) {
    let config = setup_config_manager(&no_hook_script_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_post();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn run_post_catches_inexistent_repo(no_repo_fixture: FakeConfigDir) {
    let config = setup_config_manager(&no_repo_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_post();
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}

#[rstest]
fn run_post_does_not_fail_for_valid_post_hook_setup(good_hook_fixture: FakeConfigDir) {
    let config = setup_config_manager(&good_hook_fixture);
    let ctx = setup_context("ricer commit");
    let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    let result = hook_mgr.run_post();
    assert!(result.is_ok());
}
