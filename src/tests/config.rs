// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::*;
use crate::locate::MockLocator;
use crate::test_tools::{err_check, DirFixture, FileFixtureKind};
use crate::tests::FakeConfigDir;

use anyhow::Result;
use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use toml_edit::{DocumentMut, Item, Key, Value};

fn setup_repo_doc(entry: (Key, Item)) -> Result<DocumentMut> {
    let mut doc: DocumentMut = "[repos]".parse()?;
    let table = doc.get_mut("repos").unwrap();
    let table = table.as_table_mut().unwrap();
    let (key, item) = entry;
    table.insert_formatted(&key, item);
    table.set_implicit(true);
    Ok(doc)
}

fn setup_cmd_hook_doc(entry: (Key, Item)) -> Result<DocumentMut> {
    let mut doc: DocumentMut = "[hooks]".parse()?;
    let table = doc.get_mut("hooks").unwrap();
    let table = table.as_table_mut().unwrap();
    let (key, item) = entry;
    table.insert_formatted(&key, item);
    table.set_implicit(true);
    Ok(doc)
}

#[fixture]
fn good_configs() -> DirFixture {
    DirFixture::open()
        .with_file(
            "repos.toml",
            indoc! {r#"
                # should still exist!
                [repos.vim]
                branch = "master"
                remote = "origin"
                workdir_home = true
            "#},
            FileFixtureKind::Normal,
        )
        .with_file(
            "hooks.toml",
            indoc! {r#"
                # should still exist!
                [hooks]
                bootstrap = [
                    { pre = "hook.sh", post = "hook.sh", workdir = "/some/dir" },
                    { pre = "hook.sh" }
                ]
            "#},
            FileFixtureKind::Normal,
        )
        .write()
}

#[rstest]
fn config_file_load_works(good_configs: DirFixture) {
    let repos_fixture = good_configs.get_fixture("repos.toml");
    let hooks_fixture = good_configs.get_fixture("hooks.toml");

    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(repos_fixture.as_path().into());
    locator.expect_hooks_config().return_const(hooks_fixture.as_path().into());

    let repo_cfg = err_check!(ConfigFile::load(RepoConfig, &locator));
    let hook_cfg = err_check!(ConfigFile::load(CmdHookConfig, &locator));
    assert_eq!(repo_cfg.to_string(), repos_fixture.as_str());
    assert_eq!(hook_cfg.to_string(), hooks_fixture.as_str());
}

#[rstest]
#[case::repo_data(
    RepoConfig,
    FakeConfigDir::builder()?.config_file("repos.toml", "this 'will fail'")?.build(),
)]
#[case::hook_data(
    CmdHookConfig,
    FakeConfigDir::builder()?.config_file("hooks.toml", "this 'will fail'")?.build(),
)]
fn config_file_load_catches_toml_error(
    #[case] config_type: impl Config,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let result = ConfigFile::load(config_type, &locator);
    assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

    Ok(())
}

#[rstest]
#[case::repo_data(RepoConfig, FakeConfigDir::builder()?.build())]
#[case::hook_data(CmdHookConfig, FakeConfigDir::builder()?.build())]
fn config_file_load_creates_new_file(
    #[case] config_type: impl Config,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigFile::load(config_type, &locator)?;
    assert!(config.as_path().exists());

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepoConfig,
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
    RepoSettings::new("dwm").branch("main").remote("upstream").workdir_home(false),
)]
#[case::hook_data(
    CmdHookConfig,
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
    CmdHookSettings::new("commit").add_hook(HookSettings::new().post("hook.sh")),
)]
fn config_file_save_works<E, T>(
    #[case] config_type: T,
    #[case] mut config_data: FakeConfigDir,
    #[case] entry: E,
) -> Result<()>
where
    E: Settings,
    T: Config<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    config.add(entry)?;
    config.save()?;
    config_data.sync()?;
    assert_eq!(config.to_string(), config_data.fixture(config.as_path())?.as_str());

    Ok(())
}

#[rstest]
#[case::repo_data(RepoConfig, FakeConfigDir::builder()?.build())]
#[case::hook_data(CmdHookConfig, FakeConfigDir::builder()?.build())]
fn config_file_save_creates_new_file(
    #[case] config_type: impl Config,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    config.save()?;
    assert!(config.as_path().exists());

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepoConfig,
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
    RepoSettings::new("vim").branch("master").remote("origin").workdir_home(true),
)]
#[case::hook_data(
    CmdHookConfig,
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
    CmdHookSettings::new("commit")
        .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
        .add_hook(HookSettings::new().pre("hook.sh")),
)]
fn config_file_get_works<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] key: &str,
    #[case] expect: E,
) -> Result<()>
where
    E: Settings,
    T: Config<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigFile::load(config_type, &locator)?;
    let result = config.get(key)?;
    assert_eq!(result, expect);

    Ok(())
}

#[rstest]
#[case(RepoConfig, FakeConfigDir::builder()?.build())]
#[case(CmdHookConfig, FakeConfigDir::builder()?.build())]
fn config_file_get_catches_errors(
    #[case] config_type: impl Config,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let config = ConfigFile::load(config_type, &locator)?;
    let result = config.get("non-existent");
    assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepoConfig,
    FakeConfigDir::builder()?.build(),
    RepoSettings::new("vim").branch("main").remote("origin").workdir_home(true),
    indoc! {r#"
        [repos.vim]
        branch = "main"
        remote = "origin"
        workdir_home = true
    "#},
)]
#[case::hook_data(
    CmdHookConfig,
    FakeConfigDir::builder()?.build(),
    CmdHookSettings::new("commit")
        .add_hook(HookSettings::new().pre("hook.sh"))
        .add_hook(HookSettings::new().post("hook.sh")),
    indoc! {r#"
        [hooks]
        commit = [
            { pre = "hook.sh" },
            { post = "hook.sh" }
        ]
    "#},
)]
fn config_file_new_data<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] entry: E,
    #[case] expect: &str,
) -> Result<()>
where
    E: Settings,
    T: Config<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    config.add(entry)?;
    assert_eq!(config.to_string(), expect);

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepoConfig,
    FakeConfigDir::builder()?
        .config_file("repos.toml", "repos = 'not a table'")?
        .build(),
    RepoSettings::default(),
)]
#[case::hook_data(
    CmdHookConfig,
    FakeConfigDir::builder()?
        .config_file("hooks.toml", "hooks = 'not a table'")?
        .build(),
    CmdHookSettings::default(),
)]
fn config_file_add_catches_errors<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] entry: E,
) -> Result<()>
where
    E: Settings,
    T: Config<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    let result = config.add(entry);
    assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepoConfig,
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
    RepoSettings::new("vim").branch("main").remote("origin").workdir_home(true),
    indoc! {r#"
        [repos.neovim]
        branch = "main"
        remote = "origin"
        workdir_home = true
    "#}
)]
#[case::hook_data(
    CmdHookConfig,
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
    CmdHookSettings::new("commit")
        .add_hook(HookSettings::new().pre("hook.sh"))
        .add_hook(HookSettings::new().post("hook.sh")),
    indoc! {r#"
        [hooks]
        bootstrap = [
            { pre = "hook.sh" },
            { post = "hook.sh" }
        ]
    "#}
)]
fn config_file_rename_works<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] from: &str,
    #[case] to: &str,
    #[case] expect: E,
    #[case] doc: &str,
) -> Result<()>
where
    E: Settings,
    T: Config<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    let result = config.rename(from, to)?;
    assert_eq!(result, expect);
    assert_eq!(config.to_string(), doc);

    Ok(())
}

#[rstest]
#[case::repo_data(RepoConfig, FakeConfigDir::builder()?.build())]
#[case::hook_data(CmdHookConfig, FakeConfigDir::builder()?.build())]
fn config_file_rename_catches_errors(
    #[case] config_type: impl Config,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    let result = config.rename("gonna", "fail");
    assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

    Ok(())
}

#[rstest]
#[case::repo_data(
    RepoConfig,
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
    RepoSettings::new("vim").branch("master").remote("origin").workdir_home(true),
    indoc! {r#"

        [repos.st]
        branch = "master"
        remote = "origin"
        workdir_home = true
    "#},
)]
#[case::hook_data(
    CmdHookConfig,
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
    CmdHookSettings::new("commit")
        .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
        .add_hook(HookSettings::new().pre("hook.sh")),
    indoc! {r#"
            [hooks]

            bootstrap = [
                { pre = "hook.sh" },
                { post = "hook.sh" }
            ]
    "#},
)]
fn config_file_remove_works<E, T>(
    #[case] config_type: T,
    #[case] config_data: FakeConfigDir,
    #[case] key: &str,
    #[case] expect: E,
    #[case] doc: &str,
) -> Result<()>
where
    E: Settings,
    T: Config<Entry = E>,
{
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    let result = config.remove(key)?;
    assert_eq!(result, expect);
    assert_eq!(config.to_string(), doc);

    Ok(())
}

#[rstest]
#[case::repo_data(RepoConfig, FakeConfigDir::builder()?.build())]
#[case::hook_data(CmdHookConfig, FakeConfigDir::builder()?.build())]
fn config_file_remove_catches_errors(
    #[case] config_type: impl Config,
    #[case] config_data: FakeConfigDir,
) -> Result<()> {
    let mut locator = MockLocator::new();
    locator.expect_repos_config().return_const(config_data.config_dir().join("repos.toml"));
    locator.expect_hooks_config().return_const(config_data.config_dir().join("hooks.toml"));

    let mut config = ConfigFile::load(config_type, &locator)?;
    let result = config.remove("non-existent");
    assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

    Ok(())
}
