// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::{CommandHook, Entry, Hook, Repository};
use crate::manager::{
    CommandHookData, ConfigManager, ConfigManagerError, MockDirLocator, RepositoryData, TomlManager,
};
use crate::tests::FakeConfigDir;

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?.config_file("repos.toml", "this = 'will parse'\n")?.build(),
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?.config_file("hooks.toml", "this = 'will parse'\n")?.build(),
)]
fn config_manager_load_works(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let config = ConfigManager::load(config_type, locator)?;
    assert_eq!(config.to_string(), config_data.fixture(config.location())?.as_str());
    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?.config_file("repos.toml", "this 'will fail'")?.build(),
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?.config_file("hooks.toml", "this 'will fail'")?.build(),
)]
fn config_manager_load_catches_toml_error(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let result = ConfigManager::load(config_type, locator);
    assert!(matches!(result.unwrap_err(), ConfigManagerError::Toml { .. }));
    Ok(())
}

#[rstest]
#[case::repo_data(RepositoryData, FakeConfigDir::builder()?.build())]
#[case::hook_data(CommandHookData, FakeConfigDir::builder()?.build())]
fn config_manager_load_creates_new_file(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let config = ConfigManager::load(config_type, locator)?;
    assert!(config.location().exists());
    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?.config_file(
        "repos.toml",
        indoc! {r#"
            # should still exist after save!
            [repos.vim]
            branch = "master"
            remote = "origin"
            workdir_home = true
        "#},
    )?.build(),
    Repository::new("dwm").branch("main").remote("upstream").workdir_home(false),
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?.config_file(
        "hooks.toml",
        indoc! {r#"
            # should still exist after save!
            [hooks]
            bootstrap = [
                { pre = "hook.sh", post = "hook.sh", workdir = "/some/dir" },
                { pre = "hook.sh" }
            ]
        "#},
    )?.build(),
    CommandHook::new("commit").add_hook(Hook::new().post("hook.sh")),
)]
fn config_manager_save_works<E, T>(
    #[case] config_type: T,
    #[case] mut config_data: FakeConfigDir,
    #[case] entry: E,
) -> Result<()>
where
    E: Entry,
    T: TomlManager<ConfigEntry = E>,
{
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let mut config = ConfigManager::load(config_type, locator)?;
    config.add(entry)?;
    config.save()?;
    config_data.sync()?;
    assert_eq!(config.to_string(), config_data.fixture(config.location())?.as_str());
    Ok(())
}

#[rstest]
#[case::repo_data(RepositoryData, FakeConfigDir::builder()?.build())]
#[case::hook_data(CommandHookData, FakeConfigDir::builder()?.build())]
fn config_manager_save_creates_new_file(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let mut config = ConfigManager::load(config_type, locator)?;
    config.save()?;
    assert!(config.location().exists());
    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?.config_file(
        "repos.toml",
        indoc! {r#"
            [repos.vim]
            branch = "master"
            remote = "origin"
            workdir_home = true
        "#},
    )?.build(),
    "vim",
    Repository::new("vim").branch("master").remote("origin").workdir_home(true),
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?.config_file(
        "hooks.toml",
        indoc! {r#"
            [hooks]
            commit = [
                { pre = "hook.sh", post = "hook.sh", workdir = "/some/dir" },
                { pre = "hook.sh" }
            ]
        "#},
    )?.build(),
    "commit",
    CommandHook::new("commit")
        .add_hook(Hook::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
        .add_hook(Hook::new().pre("hook.sh")),

)]
fn config_manager_get_works<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] key: &str,
    #[case] expect: E,
) -> Result<()>
where
    E: Entry,
    T: TomlManager<ConfigEntry = E>,
{
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let config = ConfigManager::load(config_type, locator)?;
    let result = config.get(key)?;
    assert_eq!(result, expect);
    Ok(())
}

#[rstest]
#[case(RepositoryData, FakeConfigDir::builder()?.build())]
#[case(CommandHookData, FakeConfigDir::builder()?.build())]
fn config_manager_get_catches_errors(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockDirLocator::new();
    locator.expect_config_dir().return_const(config_data.config_dir().into());
    let config = ConfigManager::load(config_type, locator)?;
    let result = config.get("non-existent");
    assert!(matches!(result.unwrap_err(), ConfigManagerError::Toml { .. }));
    Ok(())
}
