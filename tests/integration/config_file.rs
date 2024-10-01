// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use rstest::{fixture, rstest};
use pretty_assertions::assert_eq;

use ricer::config::{Config, RepoConfig, CmdHookConfig, ConfigFile};
use ricer_test_tools::fakes::{FakeConfigDir, FakeHomeDir};

#[fixture]
fn repo_config() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file("fixture.toml", indoc! {r#"
            [repos.vim]
            branch = "master"
            remote = "origin"
            workdir_home = true

            [repos.vim.bootstrap]
            clone = "https://github.com/awkless/vim.git"
            os = "any"
            users = ["awkless", "turing"]
            hosts = ["lovelace", "godel"]
        "#})
        .build()
}

#[fixture]
fn cmd_hook_config() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file("fixture.toml", indoc! {r#"
            [hooks]
            commits = [
                { pre = "hook.sh" },
                { post = "hook.sh", workdir = "/some/path" },
                { pre = "hook.sh", post = "hook.sh" }
            ]
        "#})
        .build()
}

#[fixture]
fn no_config() -> FakeHomeDir {
    FakeHomeDir::new()
}


#[rstest]
#[case::repo_config((RepoConfig, repo_config()))]
#[case::cmd_hook_config((CmdHookConfig, cmd_hook_config()))]
fn load_no_error(#[case] variant: (impl Config, FakeConfigDir)) -> Result<()> {
    let (config, fake) = variant;
    let fixture = fake.get_config_file("fixture.toml");
    let config = ConfigFile::load(config, fixture.as_path())?;
    assert_eq!(fixture.data(), config.to_string());
    Ok(())
}

#[rstest]
#[case::repo_config((RepoConfig, no_config()))]
#[case::cmd_hook_config((CmdHookConfig, no_config()))]
fn load_creates_nonexistent_file(#[case] variant: (impl Config, FakeHomeDir)) -> Result<()> {
    let (config, fake) = variant;
    let path = fake.as_path().join("new_config.toml");
    let config = ConfigFile::load(config, &path)?;
    assert!(path.exists());
    assert_eq!(config.path(), path);
    Ok(())
}
