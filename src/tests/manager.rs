// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::manager::{
    CommandHookData, ConfigManager, ConfigManagerError, MockDirLocator, RepositoryData, TomlManager,
};
use crate::tests::FakeConfigDir;

use anyhow::Result;
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
    let mut config = ConfigManager::new(config_type, locator);
    config.load()?;
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
    let mut config = ConfigManager::new(config_type, locator);
    let result = config.load();
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
    let mut config = ConfigManager::new(config_type, locator);
    let result = config.load();
    assert!(result.is_ok());
    assert!(config.location().exists());
    Ok(())
}
