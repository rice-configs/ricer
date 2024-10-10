// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use ricer_test_tools::fakes::{FakeConfigDir, FakeHomeDir};
use rstest::{fixture, rstest};

use crate::config::{ConfigFile, MockConfig, Repo};

#[fixture]
#[once]
fn good_config() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file(
            "fixture.toml",
            indoc! {r#"
            [repos.vim]
            branch = "main"
            remote = "origin"
            workdir_home = true
        "#},
        )
        .build()
}

#[fixture]
#[once]
fn empty_config() -> FakeConfigDir {
    FakeConfigDir::builder().config_file("fixture.toml", "# empty").build()
}

#[fixture]
#[once]
fn bad_toml_config() -> FakeConfigDir {
    FakeConfigDir::builder().config_file("fixture.toml", r#"this "will fail" "#).build()
}

#[rstest]
fn load_parses_config_file(good_config: &FakeConfigDir) -> Result<()> {
    let mock_cfg_file = MockConfig::new();
    let fixture = good_config.get_config_file("fixture.toml");
    let config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    assert_eq!(fixture.data(), config.to_string());
    Ok(())
}

#[rstest]
fn load_creates_new_file() -> Result<()> {
    let fake = FakeHomeDir::new();
    let path = fake.as_path().join("new_config.toml");
    let mock_cfg_file = MockConfig::new();
    let config = ConfigFile::load(mock_cfg_file, &path)?;
    assert!(path.exists());
    assert_eq!(config.path(), path);
    Ok(())
}

#[rstest]
fn load_catches_invalid_toml(bad_toml_config: &FakeConfigDir) -> Result<()> {
    let mock_cfg_file = MockConfig::new();
    let fixture = bad_toml_config.get_config_file("fixture.toml");
    let result = ConfigFile::load(mock_cfg_file, fixture.as_path());
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn get_catches_error(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_get().returning(|_, _| Err(anyhow!("fail for whatever reason")));
    let fixture = good_config.get_config_file("fixture.toml");
    let config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let result = config.get("fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn get_deserialize_no_error(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_get().returning(|doc, key| {
        let data = doc.get("repos", key)?;
        Ok(Repo::from(data))
    });
    let fixture = good_config.get_config_file("fixture.toml");
    let config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let result = config.get("vim")?;
    let expect = Repo::builder("vim").branch("main").remote("origin").workdir_home(true).build();
    assert_eq!(expect, result);
    Ok(())
}

#[rstest]
fn add_catches_error(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_add().returning(|_, _| Err(anyhow!("fail for whatever reason")));
    let fixture = good_config.get_config_file("fixture.toml");
    let mut config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let result = config.add(Repo::default());
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn add_inserts_new_entry(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_add().returning(|doc, entry| {
        let entry = doc.add("repos", entry.to_toml())?.map(Repo::from);
        Ok(entry)
    });
    let fixture = good_config.get_config_file("fixture.toml");
    let mut config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let entry = Repo::builder("dwm").branch("main").remote("origin").workdir_home(true).build();
    let result = config.add(entry)?;
    let expect = formatdoc! {r#"
        {}
        [repos.dwm]
        branch = "main"
        remote = "origin"
        workdir_home = true
    "#, fixture.data()};
    assert_eq!(result, None);
    assert_eq!(expect, config.to_string());
    Ok(())
}

#[rstest]
fn add_replaces_entry(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_add().returning(|doc, entry| {
        let entry = doc.add("repos", entry.to_toml())?.map(Repo::from);
        Ok(entry)
    });
    let fixture = good_config.get_config_file("fixture.toml");
    let mut config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let entry = Repo::builder("vim").branch("master").remote("origin").workdir_home(true).build();
    let result = config.add(entry)?;
    let ret_expect =
        Some(Repo::builder("vim").branch("main").remote("origin").workdir_home(true).build());
    let str_expect = indoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true
    "#};
    assert_eq!(ret_expect, result);
    assert_eq!(str_expect, config.to_string());
    Ok(())
}

#[rstest]
fn add_inserts_new_section(empty_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_add().returning(|doc, entry| {
        let entry = doc.add("repos", entry.to_toml())?.map(Repo::from);
        Ok(entry)
    });
    let fixture = empty_config.get_config_file("fixture.toml");
    let mut config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let entry = Repo::builder("vim").branch("master").remote("origin").workdir_home(true).build();
    let result = config.add(entry)?;
    let expect = formatdoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true
        {}"#,
    fixture.data()};
    assert_eq!(result, None);
    assert_eq!(expect, config.to_string());
    Ok(())
}

#[rstest]
fn remove_catches_errors(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_remove().returning(|_, _| Err(anyhow!("fail for whatever reason")));
    let fixture = good_config.get_config_file("fixture.toml");
    let mut config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let result = config.remove("fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn remove_give_deleted_entry(good_config: &FakeConfigDir) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_remove().returning(|doc, key| {
        let entry = doc.remove("repos", key.as_ref())?;
        Ok(Repo::from(entry))
    });
    let fixture = good_config.get_config_file("fixture.toml");
    let mut config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let result = config.remove("vim")?;
    let expect = Repo::builder("vim").branch("main").remote("origin").workdir_home(true).build();
    assert_eq!(expect, result);
    assert_eq!("", config.to_string());
    Ok(())
}
