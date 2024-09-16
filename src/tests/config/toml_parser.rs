// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use toml_edit::{Key, Item, Value};
use ricer_test_tools::fakes::FakeHomeDir;
use ricer_test_tools::fixtures::FileFixture;

use crate::config::TomlParser;

#[test]
fn read_catches_nonexistent_path() {
    let mut toml = TomlParser::new();
    let result = toml.read("nonexistant.toml");
    assert!(matches!(result, Err(..)));
}

#[test]
fn read_catches_bad_toml_formatting() {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"this "will fail""#)
        .build();
    let mut toml = TomlParser::new();
    let result = toml.read(fixture.as_path());
    assert!(matches!(result, Err(..)));
}

#[test]
fn read_parses_correctly() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            [testing]
            this = "will parse"
        "#})
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let expect = fixture.data();
    let result = toml.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn write_serializes_correctly() -> Result<()> {
    let fake = FakeHomeDir::new();
    let mut fixture = FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .build();
    let mut toml = TomlParser::new();
    let key = Key::new("this");
    let item = Item::Value(Value::from("will parse"));
    toml.add_entry("testing", (key, item))?;
    toml.write(fixture.as_path())?;
    fixture.sync();
    let expect = indoc! {r#"
        [testing]
        this = "will parse"
    "#};
    let result = fixture.data();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn get_entry_catches_nonexistent_section() {
    let toml = TomlParser::new();
    let result = toml.get_entry("nonexistent", "bad");
    assert!(matches!(result, Err(..)));
}
