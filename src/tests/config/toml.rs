// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use indoc::indoc;
use pretty_assertions::assert_eq;
use anyhow::Result;
use rstest::{fixture, rstest};
use toml_edit::{Key, Item, Value};

use crate::config::Toml;

#[fixture]
fn good_toml() -> String {
    String::from(indoc! {r#"
        razz = "some data"

        [test]
        foo = "some data"
        bar = "some data"

        [test.baaz]
        buzz = "some data"
    "#})
}

#[fixture]
fn new_entry() -> (Key, Item) {
    (Key::new("cool"), Item::Value(Value::from("new data")))
}

#[fixture]
fn replace_entry() -> (Key, Item) {
    (Key::new("foo"), Item::Value(Value::from("new data")))
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
    let result = config.get_entry("razz", "fail");
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

#[rstest]
fn add_entry_nontable_section_error(good_toml: String, new_entry: (Key, Item)) -> Result<()> {
    let mut config: Toml = good_toml.parse()?;
    let result = config.add_entry("razz", new_entry);
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
fn add_entry_create_new_section(new_entry: (Key, Item)) -> Result<()> {
    let mut config = Toml::new();
    config.add_entry("new_test", new_entry)?;
    let expect = indoc! {r#"
        [new_test]
        cool = "new data"
    "#};
    let result = config.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[rstest]
fn add_entry_no_error(good_toml: String, new_entry: (Key, Item)) -> Result<()> {
    let mut config: Toml = good_toml.parse()?;
    let old_entry = config.add_entry("test", new_entry)?;
    let expect = indoc! {r#"
        razz = "some data"

        [test]
        foo = "some data"
        bar = "some data"
        cool = "new data"

        [test.baaz]
        buzz = "some data"
    "#};
    let result = config.to_string();
    assert_eq!(expect, result);
    assert!(matches!(old_entry, None));
    Ok(())
}

#[rstest]
fn add_entry_replaces_entry(good_toml: String, replace_entry: (Key, Item)) -> Result<()> {
    let mut config: Toml = good_toml.parse()?;
    let (old_key, old_value) = config.add_entry("test", replace_entry)?.unwrap();
    let expect = good_toml.replace(r#"foo = "some data""#, r#"foo = "new data""#);
    let result = config.to_string();
    assert_eq!(expect, result);
    assert_eq!("foo", old_key.get());
    assert_eq!("some data", old_value.as_str().unwrap_or_default());
    Ok(())
}
