// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::{CommandHook, ConfigEntry, Hook, Repository};
use crate::manager::{
    CommandHookData, ConfigManager, ConfigManagerError, MockLocator, RepositoryData, TomlManager,
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
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigManager::load(config_type, &locator)?;
    assert_eq!(config.to_string(), config_data.fixture(config.as_path())?.as_str());

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
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let result = ConfigManager::load(config_type, &locator);
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
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigManager::load(config_type, &locator)?;
    assert!(config.as_path().exists());

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?
        .config_file(
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
    E: ConfigEntry,
    T: TomlManager<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    config.add(entry)?;
    config.save()?;
    config_data.sync()?;
    assert_eq!(config.to_string(), config_data.fixture(config.as_path())?.as_str());

    Ok(())
}

#[rstest]
#[case::repo_data(RepositoryData, FakeConfigDir::builder()?.build())]
#[case::hook_data(CommandHookData, FakeConfigDir::builder()?.build())]
fn config_manager_save_creates_new_file(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    config.save()?;
    assert!(config.as_path().exists());

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
    E: ConfigEntry,
    T: TomlManager<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigManager::load(config_type, &locator)?;
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
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigManager::load(config_type, &locator)?;
    let result = config.get("non-existent");
    assert!(matches!(result.unwrap_err(), ConfigManagerError::Toml { .. }));

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?.build(),
    Repository::new("vim").branch("main").remote("origin").workdir_home(true),
    indoc! {r#"
        [repos.vim]
        branch = "main"
        remote = "origin"
        workdir_home = true
    "#},
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?.build(),
    CommandHook::new("commit")
        .add_hook(Hook::new().pre("hook.sh"))
        .add_hook(Hook::new().post("hook.sh")),
    indoc! {r#"
        [hooks]
        commit = [
            { pre = "hook.sh" },
            { post = "hook.sh" }
        ]
    "#},
)]
fn config_manager_new_data<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] entry: E,
    #[case] expect: &str,
) -> Result<()>
where
    E: ConfigEntry,
    T: TomlManager<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    config.add(entry)?;
    assert_eq!(config.to_string(), expect);

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?
        .config_file("repos.toml", "repos = 'not a table'")?
        .build(),
    Repository::default(),
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?
        .config_file("hooks.toml", "hooks = 'not a table'")?
        .build(),
    CommandHook::default(),
)]
fn config_manager_add_catches_errors<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] entry: E,
) -> Result<()>
where
    E: ConfigEntry,
    T: TomlManager<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    let result = config.add(entry);
    assert!(matches!(result.unwrap_err(), ConfigManagerError::Toml { .. }));

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepositoryData,
    FakeConfigDir::builder()?
        .config_file(
            "repos.toml",
            indoc! {r#"
                [repos.vim]
                branch = "main"
                remote = "origin"
                workdir_home = true
            "#}
        )?
        .build(),
    "vim",
    "neovim",
    Repository::new("vim").branch("main").remote("origin").workdir_home(true),
    indoc! {r#"
        [repos.neovim]
        branch = "main"
        remote = "origin"
        workdir_home = true
    "#}
)]
#[case::hook_data(
    CommandHookData,
    FakeConfigDir::builder()?
        .config_file(
            "hooks.toml",
            indoc! {r#"
                [hooks]
                commit = [
                    { pre = "hook.sh" },
                    { post = "hook.sh" }
                ]
            "#},
        )?
        .build(),
    "commit",
    "bootstrap",
    CommandHook::new("commit")
        .add_hook(Hook::new().pre("hook.sh"))
        .add_hook(Hook::new().post("hook.sh")),
    indoc! {r#"
        [hooks]
        bootstrap = [
            { pre = "hook.sh" },
            { post = "hook.sh" }
        ]
    "#}
)]
fn config_manager_rename_works<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] from: &str,
    #[case] to: &str,
    #[case] expect: E,
    #[case] doc: &str,
) -> Result<()>
where
    E: ConfigEntry,
    T: TomlManager<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    let result = config.rename(from, to)?;
    assert_eq!(result, expect);
    assert_eq!(config.to_string(), doc);

    Ok(())
}

#[rstest]
#[case::repo_data(RepositoryData, FakeConfigDir::builder()?.build())]
#[case::hook_data(CommandHookData, FakeConfigDir::builder()?.build())]
fn config_manager_rename_catches_errors(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    let result = config.rename("gonna", "fail");
    assert!(matches!(result.unwrap_err(), ConfigManagerError::Toml { .. }));

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

            [repos.st]
            branch = "master"
            remote = "origin"
            workdir_home = true
        "#},
    )?.build(),
    "vim",
    Repository::new("vim").branch("master").remote("origin").workdir_home(true),
    indoc! {r#"

        [repos.st]
        branch = "master"
        remote = "origin"
        workdir_home = true
    "#},
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

            bootstrap = [
                { pre = "hook.sh" },
                { post = "hook.sh" }
            ]
        "#},
    )?.build(),
    "commit",
    CommandHook::new("commit")
        .add_hook(Hook::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
        .add_hook(Hook::new().pre("hook.sh")),
    indoc! {r#"
            [hooks]

            bootstrap = [
                { pre = "hook.sh" },
                { post = "hook.sh" }
            ]
    "#},
)]
fn config_manager_remove_works<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] key: &str,
    #[case] expect: E,
    #[case] doc: &str,
) -> Result<()>
where
    E: ConfigEntry,
    T: TomlManager<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    let result = config.remove(key)?;
    assert_eq!(result, expect);
    assert_eq!(config.to_string(), doc);

    Ok(())
}

#[rstest]
#[case::repo_data(RepositoryData, FakeConfigDir::builder()?.build())]
#[case::hook_data(CommandHookData, FakeConfigDir::builder()?.build())]
fn config_manager_remove_catches_errors(
    #[case] config_type: impl TomlManager,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigManager::load(config_type, &locator)?;
    let result = config.remove("non-existent");
    assert!(matches!(result.unwrap_err(), ConfigManagerError::Toml { .. }));

    Ok(())
}
