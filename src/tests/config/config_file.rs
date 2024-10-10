// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Result};
use rstest::{fixture, rstest};
use indoc::indoc;
use pretty_assertions::assert_eq;
use ricer_test_tools::fakes::{FakeHomeDir, FakeConfigDir};

use crate::config::{ConfigFile, MockConfig, Repo};


#[fixture]
#[once]
fn good_config() ->  FakeConfigDir {
    FakeConfigDir::builder()
        .config_file("fixture.toml", indoc! {r#"
            [repos.vim]
            branch = "main"
            remote = "origin"
            workdir_home = true
        "#})
        .build()
}

#[fixture]
#[once]
fn expect_de_vim() -> Repo {
    Repo::builder("vim")
        .branch("main")
        .remote("origin")
        .workdir_home(true)
        .build()
}

#[fixture]
#[once]
fn bad_toml_config() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file("fixture.toml", r#"this "will fail" "#)
        .build()
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
fn get_deserialize_no_error(good_config: &FakeConfigDir, expect_de_vim: &Repo) -> Result<()> {
    let mut mock_cfg_file = MockConfig::new();
    mock_cfg_file.expect_get().returning(|doc, key| {
        let data = doc.get("repos", key)?;
        Ok(Repo::from(data))
    });
    let fixture = good_config.get_config_file("fixture.toml");
    let config = ConfigFile::load(mock_cfg_file, fixture.as_path())?;
    let result = config.get("vim")?;
    assert_eq!(expect_de_vim, &result);
    Ok(())
}
