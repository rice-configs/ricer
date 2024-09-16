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

#[test]
fn get_entry_catches_non_table_section() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"section = "not a table""#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.get_entry("section", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn get_entry_catches_nonexistent_key() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"[empty]"#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.get_entry("empty", "nonexistent");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn get_entry_provides_correct_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            [testing]
            this = "will parse"
        "#})
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let expect_key = Key::new("this");
    let expect_item = Item::Value(Value::from("will parse"));
    let (result_key, result_item) = toml.get_entry("testing", "this")?;
    assert_eq!(expect_key.get(), result_key.get());
    assert_eq!(expect_item.as_str().unwrap_or_default(), result_item.as_str().unwrap_or_default());
    Ok(())
}

#[test]
fn add_entry_adds_to_existing_section() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            # This will not be overwritten!
            [testing]
            this = "will parse"
        "#})
        .build();
    let entry = (Key::new("cool"), Item::Value(Value::from("new data")));
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    toml.add_entry("testing", entry)?;
    let expect = indoc! {r#"
        # This will not be overwritten!
        [testing]
        this = "will parse"
        cool = "new data"
    "#};
    let result = toml.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn add_entry_creates_new_section() -> Result<()> {
    let entry = (Key::new("cool"), Item::Value(Value::from("new data")));
    let mut toml = TomlParser::new();
    toml.add_entry("testing", entry)?;
    let expect = indoc! {r#"
        [testing]
        cool = "new data"
    "#};
    let result = toml.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn add_entry_returns_replaced_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            # This will not be overwritten!
            [testing]
            this = "will parse"
        "#})
        .build();
    let new_entry = (Key::new("this"), Item::Value(Value::from("new data")));
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let (old_key, old_item) = toml.add_entry("testing", new_entry)?.unwrap();
    let expect = indoc! {r#"
        # This will not be overwritten!
        [testing]
        this = "new data"
    "#};
    let result = toml.to_string();
    assert_eq!(expect, result);
    assert_eq!("this", old_key.get());
    assert_eq!("will parse", old_item.as_str().unwrap_or_default());
    Ok(())
}

#[test]
fn add_entry_returns_none_for_new_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            # This will not be overwritten!
            [testing]
            this = "will parse"
        "#})
        .build();
    let new_entry = (Key::new("cool"), Item::Value(Value::from("new data")));
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.add_entry("testing", new_entry)?;
    assert!(matches!(result, None));
    Ok(())
}

#[test]
fn add_entry_catches_non_table_section() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"section = "not a table""#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let entry = (Key::new("this"), Item::Value(Value::from("will fail")));
    let result = toml.add_entry("section", entry);
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn remove_entry_catches_nonexistent_section() {
    let mut toml = TomlParser::new();
    let result = toml.remove_entry("nonexistent", "bad");
    assert!(matches!(result, Err(..)));
}

#[test]
fn remove_entry_catches_non_table_section() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"section = "not a table""#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.remove_entry("section", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn remove_entry_catches_nonexistent_key() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"[empty]"#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.remove_entry("empty", "nonexistent");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn remove_entry_removes_full_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            [testing]
            this = "will parse"
        "#})
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    toml.remove_entry("testing", "this")?;
    let expect = "[testing]\n";
    let result = toml.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn remove_entry_returns_correct_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            [testing]
            this = "will parse"
        "#})
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let expect_key = Key::new("this");
    let expect_item = Item::Value(Value::from("will parse"));
    let (result_key, result_item) = toml.remove_entry("testing", "this")?;
    assert_eq!(expect_key.get(), result_key.get());
    assert_eq!(expect_item.as_str().unwrap_or_default(), result_item.as_str().unwrap_or_default());
    Ok(())
}

#[test]
fn rename_entry_catches_nonexistent_section() {
    let mut toml = TomlParser::new();
    let result = toml.rename_entry("nonexistent", "bad", "fail");
    assert!(matches!(result, Err(..)));
}

#[test]
fn rename_entry_catches_non_table_section() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"section = "not a table""#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.rename_entry("section", "fail", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn rename_entry_catches_nonexistent_key() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture = FileFixture::builder()
        .path(fake.as_path().join("bad.toml"))
        .data(r#"[empty]"#)
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let result = toml.rename_entry("empty", "nonexistent", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[test]
fn rename_entry_renames_full_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            [testing]
            this = "will parse"
        "#})
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    toml.rename_entry("testing", "this", "that")?;
    let expect = indoc! {r#"
        [testing]
        that = "will parse"
    "#};
    let result = toml.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn rename_entry_returns_old_entry() -> Result<()> {
    let fake = FakeHomeDir::new();
    let fixture =  FileFixture::builder()
        .path(fake.as_path().join("good.toml"))
        .data(indoc! {r#"
            [testing]
            this = "will parse"
        "#})
        .build();
    let mut toml = TomlParser::new();
    toml.read(fixture.as_path())?;
    let (old_key, old_item) = toml.rename_entry("testing", "this", "that")?;
    assert_eq!("this", old_key.get());
    assert_eq!("will parse", old_item.as_str().unwrap_or_default());
    Ok(())
}
