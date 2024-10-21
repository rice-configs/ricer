// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::Toml;
use crate::error::{RicerError, RicerResult};

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::rstest;
use toml_edit::{Key, Item, Value};

#[rstest]
fn toml_parse_catches_bad_formatting(
    #[values("this 'will fail'", "[will # also fail", "not.gonna = [work]")] input: &str,
) {
    let result: RicerResult<Toml> = input.parse();
    assert!(matches!(result, Err(..)));
}

#[rstest]
fn toml_parse_parses_good_formatting(
    #[values(
        indoc! {r#"
            [foo]
            this = "will parse"
            will.also = "parse"
        "#}
    )]
    input: &str,
) -> Result<()> {
    let toml: Toml = input.parse()?;
    assert_eq!(toml.to_string(), input);
    Ok(())
}

#[rstest]
#[case(
    indoc! {r#"
        [foo]
        bar = "get this"
    "#},
    (Key::new("bar"), Item::Value(Value::from("get this")))
)]
fn toml_get_returns_key_value_pair(
   #[case] input: &str, #[case] expect: (Key, Item),
) -> Result<()> {
    let toml: Toml = input.parse()?;
    let (result_key, result_value) = toml.get("foo", "bar")?;
    let (expect_key, expect_value) = expect;
    assert_eq!(result_key, &expect_key);
    assert_eq!(result_value.as_str(), expect_value.as_str());
    Ok(())
}

#[rstest]
fn toml_get_catches_non_table_error(
    #[values("foo = 'not a table'")] input: &str,
) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "fail");
    assert!(matches!(result, Err(RicerError::TomlNonTable(..))));
    Ok(())
}

#[rstest]
fn toml_get_catches_table_not_found_error(
    #[values(
        indoc! {r#"
            # No foo table anywhere to be seen here!
            [bar]
            this = "is not foo table"
        "#}
    )] input: &str,
) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "fail");
    assert!(matches!(result, Err(RicerError::TomlTableNotFound(..))));
    Ok(())
}

#[rstest]
fn toml_get_catches_key_value_not_found_error(
    #[values(
        indoc! {r#"
            # No bar key-value anywhere to be found here!
            [foo]
            this = "is not bar key-value"
        "#}
    )] input: &str,
) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "bar");
    assert!(matches!(result, Err(RicerError::TomlKeyValueNotFound(..))));
    Ok(())
}
