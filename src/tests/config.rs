// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::{Toml, TomlError};

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::rstest;
use toml_edit::{Item, Key, Value};

#[rstest]
fn toml_parse_parses_good_formatting(
    #[values("this = 'will parse'", "[so_will_this]", "hello.world = 'from ricer!'")] input: &str,
) -> Result<()> {
    let toml: Result<Toml, TomlError> = input.parse();
    assert!(toml.is_ok());
    Ok(())
}

#[rstest]
fn toml_parse_catches_bad_formatting(
    #[values("this 'will fail'", "[will # also fail", "not.gonna = [work]")] input: &str,
) {
    let result: Result<Toml, TomlError> = input.parse();
    assert!(matches!(result.unwrap_err(), TomlError::BadParse { .. }));
}

#[rstest]
#[case(
    indoc! {r#"
        [foo]
        bar = "get this"
    "#},
    (Key::new("bar"), Item::Value(Value::from("get this")))
)]
fn toml_get_returns_key_value_pair(#[case] input: &str, #[case] expect: (Key, Item)) -> Result<()> {
    let toml: Toml = input.parse()?;
    let (result_key, result_value) = toml.get("foo", "bar")?;
    let (expect_key, expect_value) = expect;
    assert_eq!(result_key, &expect_key);
    assert_eq!(result_value.as_str(), expect_value.as_str());
    Ok(())
}

#[rstest]
#[case::table_not_found("bar = 'foo not here'", TomlError::TableNotFound { table: "foo".into() })]
#[case::not_table("foo = 'not a table'", TomlError::NotTable { table: "foo".into() })]
#[case::entry_not_found(
    indoc! {r#"
        [foo]
        baz = 'bar not here'
    "#},
    TomlError::EntryNotFound { table: "foo".into(), key: "bar".into() }
)]
fn toml_get_catches_errors(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "bar");
    assert_eq!(result.unwrap_err(), expect);
    Ok(())
}
