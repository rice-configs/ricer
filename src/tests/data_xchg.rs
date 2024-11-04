// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

mod cmd_hook;
mod repo;

use crate::data_xchg::{Toml, TomlError};

use anyhow::Result;
use indoc::{formatdoc, indoc};
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use toml_edit::{Item, Key, Value};

#[fixture]
fn toml_input() -> String {
    String::from(indoc! {r#"
        # this coment should remain!
        [test]
        foo = "hello"
        bar = true
    "#})
}

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
#[case("test", "foo", (Key::new("foo"), Item::Value(Value::from("hello"))))]
#[case("test", "bar", (Key::new("bar"), Item::Value(Value::from(true))))]
fn toml_get_returns_entry(
    toml_input: String,
    #[case] table: &str,
    #[case] key: &str,
    #[case] expect: (Key, Item),
) -> Result<()> {
    let toml: Toml = toml_input.parse()?;
    let (result_key, result_value) = toml.get(table, key)?;
    let (expect_key, expect_value) = expect;
    assert_eq!(result_key, &expect_key);
    assert_eq!(result_value.is_value(), expect_value.is_value());
    Ok(())
}

#[rstest]
#[case::table_not_found("bar = 'foo not here'", TomlError::TableNotFound { table: "foo".into() })]
#[case::not_table("foo = 'not a table'", TomlError::NotTable { table: "foo".into() })]
#[case::entry_not_found(
    "[foo] # bar not here",
    TomlError::EntryNotFound { table: "foo".into(), key: "bar".into() }
)]
fn toml_get_catches_errors(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "bar");
    assert_eq!(result.unwrap_err(), expect);
    Ok(())
}

#[rstest]
#[case::add_into_table(
    toml_input(),
    "test",
    (Key::new("baz"), Item::Value(Value::from("add this"))),
    formatdoc! {r#"
        {}baz = "add this"
    "#, toml_input()}
)]
#[case::create_new_table(
    toml_input(),
    "new_test",
    (Key::new("baz"), Item::Value(Value::from("add this"))),
    formatdoc! {r#"
        {}
        [new_test]
        baz = "add this"
    "#, toml_input()}
)]
fn toml_add_new_entry(
    #[case] input: String,
    #[case] table: &str,
    #[case] entry: (Key, Item),
    #[case] expect: String,
) -> Result<()> {
    let mut toml: Toml = input.parse()?;
    let result = toml.add(table, entry)?;
    assert_eq!(toml.to_string(), expect);
    assert!(result.is_none());
    Ok(())
}

#[rstest]
#[case(
    toml_input(),
    "test",
    (Key::new("foo"), Item::Value(Value::from("replaced"))),
    toml_input().replace(r#"foo = "hello""#, r#"foo = "replaced""#)
)]
#[case(
    toml_input(),
    "test",
    (Key::new("bar"), Item::Value(Value::from(false))),
    toml_input().replace(r#"bar = true"#, r#"bar = false"#)
)]
fn toml_add_replace_entry(
    #[case] input: String,
    #[case] table: &str,
    #[case] entry: (Key, Item),
    #[case] expect: String,
) -> Result<()> {
    let mut toml: Toml = input.parse()?;
    let result = toml.add(table, entry)?;
    assert_eq!(toml.to_string(), expect);
    assert!(result.is_some());
    Ok(())
}

#[rstest]
#[case::not_table("foo = 'not a table'", TomlError::NotTable { table: "foo".into() })]
fn toml_add_catches_errors(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
    let mut toml: Toml = input.parse()?;
    let stub = (Key::new("fail"), Item::Value(Value::from("this")));
    let result = toml.add("foo", stub);
    assert_eq!(result.unwrap_err(), expect);
    Ok(())
}

#[rstest]
#[case(
    toml_input(),
    "test",
    "bar",
    "baz",
    (Key::new("bar"), Item::Value(Value::from(true))),
    toml_input().replace("bar", "baz"),
)]
fn toml_rename_renames_entry(
    #[case] input: String,
    #[case] table: &str,
    #[case] from: &str,
    #[case] to: &str,
    #[case] expect: (Key, Item),
    #[case] output: String,
) -> Result<()> {
    let mut toml: Toml = input.parse()?;
    let (return_key, return_value) = toml.rename(table, from, to)?;
    let (expect_key, expect_value) = expect;
    assert_eq!(toml.to_string(), output);
    assert_eq!(return_key, expect_key);
    assert_eq!(return_value.is_value(), expect_value.is_value());
    Ok(())
}

#[rstest]
#[case::table_not_found("bar = 'foo not here'", TomlError::TableNotFound { table: "foo".into() })]
#[case::not_table("foo = 'not a table'", TomlError::NotTable { table: "foo".into() })]
#[case::entry_not_found(
    "[foo] # bar not here",
    TomlError::EntryNotFound { table: "foo".into(), key: "bar".into() }
)]
fn toml_rename_catches_errors(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "bar");
    assert_eq!(result.unwrap_err(), expect);
    Ok(())
}

#[rstest]
#[case(
    toml_input(),
    "test",
    "foo",
    (Key::new("foo"), Item::Value(Value::from("world"))),
    toml_input().replace("foo = \"hello\"\n", ""),
)]
#[case(
    toml_input(),
    "test",
    "bar",
    (Key::new("bar"), Item::Value(Value::from(true))),
    toml_input().replace("bar = true\n", ""),
)]
fn toml_remove_returns_entry(
    #[case] input: String,
    #[case] table: &str,
    #[case] key: &str,
    #[case] expect: (Key, Item),
    #[case] output: String,
) -> Result<()> {
    let mut toml: Toml = input.parse()?;
    let (return_key, return_value) = toml.remove(table, key)?;
    let (expect_key, expect_value) = expect;
    assert_eq!(toml.to_string(), output);
    assert_eq!(return_key, expect_key);
    assert_eq!(return_value.is_value(), expect_value.is_value());
    Ok(())
}

#[rstest]
#[case::table_not_found("bar = 'foo not here'", TomlError::TableNotFound { table: "foo".into() })]
#[case::not_table("foo = 'not a table'", TomlError::NotTable { table: "foo".into() })]
#[case::entry_not_found(
    "[foo] # bar not here",
    TomlError::EntryNotFound { table: "foo".into(), key: "bar".into() }
)]
fn toml_remove_catches_errors(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
    let toml: Toml = input.parse()?;
    let result = toml.get("foo", "bar");
    assert_eq!(result.unwrap_err(), expect);
    Ok(())
}
