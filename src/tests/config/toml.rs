// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use indoc::indoc;
use pretty_assertions::assert_eq;
use anyhow::Result;
use rstest::{fixture, rstest};

use crate::config::Toml;

#[fixture]
fn good_toml() -> String {
    String::from(indoc! {r#"
        [test]
        foo = "some data"
        bar = "some data"

        [test.baaz]
        buzz = "some data"
    "#})
}

#[fixture]
fn bad_toml() -> String {
    String::from(r#"this "will fail""#)
}

#[rstest]
fn parse_bad_toml_error(bad_toml: String) {
    let result: Result<Toml> = bad_toml.parse();
    assert!(matches!(result, Err(..)));
}

#[rstest]
fn parse_no_error(good_toml: String) -> Result<()> {
    let expect = good_toml;
    let config: Toml = expect.parse()?;
    let result = config.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[rstest]
fn get_entry_no_section_error(good_toml: String) -> Result<()> {
    let config: Toml = good_toml.parse()?;
    let result = config.get_entry("nonexistent", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn get_entry_nontable_section_error(good_toml: String) -> Result<()> {
    let config: Toml = good_toml.parse()?;
    let result = config.get_entry("foo", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn get_entry_no_key_error(good_toml: String) -> Result<()> {
    let config: Toml = good_toml.parse()?;
    let result = config.get_entry("test", "nonexistent");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn get_entry_returns_entry(good_toml: String) -> Result<()> {
    let config: Toml = good_toml.parse()?;
    let (key, value) = config.get_entry("test", "foo")?;
    assert_eq!("foo", key.get());
    assert_eq!("some data", value.as_str().unwrap_or_default());
    Ok(())
}
