// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use rstest::{fixture, rstest};
use pretty_assertions::assert_eq;

use crate::config::Toml;
use ricer_test_tools::fakes::{FakeHomeDir, FakeConfigDir};

#[fixture]
fn good_toml() -> FakeConfigDir {
    FakeConfigDir::builder()
        .config_file("fixture.toml", indoc! {r#"
            [test]
            key1 = "some data"
        "#})
        .build()
}

#[fixture]
fn bad_toml() -> FakeConfigDir {
    FakeConfigDir::builder().config_file("fixture.toml", r#"this "will fail""#).build()
}


#[rstest]
fn load_no_error(good_toml: FakeConfigDir) -> Result<()> {
    let file = good_toml.get_config_file("fixture.toml");
    let toml = Toml::load(file.as_path())?;
    assert_eq!(file.data(), toml.to_string());
    Ok(())
}

#[rstest]
fn load_creates_new_file() -> Result<()> {
    let home = FakeHomeDir::new();
    let toml = Toml::load(home.as_path().join("config.toml"))?;
    assert!(toml.as_path().exists());
    Ok(())
}

#[rstest]
fn load_toml_format_error(bad_toml: FakeConfigDir) {
    let result = Toml::load(bad_toml.get_config_file("fixture.toml").as_path());
    assert!(matches!(result, Err(..)));
}
