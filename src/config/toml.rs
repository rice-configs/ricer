// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use log::{info, trace, debug};
use std::{fmt, str::FromStr};
use toml_edit::{DocumentMut, Table, Key, Item};

/// TOML parser.
///
/// Offers basic CRUD interface for TOML parsing. Expects TOML data in string
/// form. Leaves file handling to caller. Mainly operates on whole tables for
/// key-value pair manipulation. Note, that `document` is terminology used to
/// refer to parsed TOML data.
///
/// # Invariants
///
/// 1. Preserve original formatting of document.
///
/// # See also
///
/// - [`ConfigFile`]
#[derive(Clone, Default, Debug)]
pub struct Toml {
    doc: DocumentMut,
}

impl Toml {
    pub fn new() -> Self {
        trace!("Construct new TOML parser");
        Self { doc: DocumentMut::new() }
    }

    /// Add TOML entry into document.
    ///
    /// Will add given `entry` into target `table`. If `table` does not exist, then it
    /// will be created and `entry` will be inserted into it.
    ///
    /// Will replace any entries that match the key in `entry`, returning the
    /// old entry that was replaced. If no replacement took place, then `None`
    /// is returned instead.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    pub fn add(
        &mut self,
        table: impl AsRef<str>,
        entry: (Key, Item),
    ) -> Result<Option<(Key, Item)>, TomlError> {
        let (key, value) = entry;
        info!("Add TOML entry '{}' to '{}' table", key.get(), table.as_ref());
        let entry = match self.get_table_mut(table.as_ref()) {
            Ok(table) => table,
            Err(TomlError::TableNotFound { .. }) => {
                let mut new_table = Table::new();
                new_table.set_implicit(true);
                self.doc.insert(table.as_ref(), Item::Table(new_table));
                self.doc[table.as_ref()].as_table_mut().unwrap()
            }
            Err(err) => return Err(err),
        };
        let entry = entry.insert(key.get(), value).map(|old| (key, old));
        Ok(entry)
    }

    /// Get entry from target table in document.
    ///
    /// Return reference to full key-value pair in document.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    /// - Return [`TomlError::EntryNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    /// [`TomlError::EntryNotFound`]: crate::config::TomlError::EntryNotFound
    pub fn get<S>(&self, table: S, key: S) -> Result<(&Key, &Item), TomlError>
    where
        S: AsRef<str>,
    {
        info!("Get TOML entry '{}' from '{}' table", key.as_ref(), table.as_ref());
        let entry = self.get_table(table.as_ref())?;
        let entry = entry.get_key_value(key.as_ref()).ok_or_else(|| TomlError::EntryNotFound {
            table: table.as_ref().into(),
            key: key.as_ref().into(),
        })?;
        Ok(entry)
    }

    /// Rename TOML entry from document.
    ///
    /// Rename entry from target `table`. Returns old unrenamed entry.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    /// - Return [`TomlError::EntryNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    /// [`TomlError::EntryNotFound`]: crate::config::TomlError::EntryNotFound
    pub fn rename<S>(&mut self, table: S, from: S, to: S) -> Result<(Key, Item), TomlError>
    where
        S: AsRef<str>,
    {
        let entry = self.get_table_mut(table.as_ref())?;
        let (old_key, old_item) = entry.remove_entry(from.as_ref()).ok_or_else(|| {
            TomlError::EntryNotFound { table: table.as_ref().into(), key: from.as_ref().into() }
        })?;

        // INVARIANT: preserve original formatting that existed beforehand.
        let new_key = Key::new(to.as_ref()).with_leaf_decor(old_key.leaf_decor().clone());
        entry.insert_formatted(&new_key, old_item.clone());

        Ok((old_key, old_item))
    }

    /// Remove TOML entry from document.
    ///
    /// Remove `key` from target `table`. Returns removed entry.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    /// - Return [`TomlError::EntryNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    /// [`TomlError::EntryNotFound`]: crate::config::TomlError::EntryNotFound
    pub fn remove<S>(&mut self, table: S, key: S) -> Result<(Key, Item), TomlError>
    where
        S: AsRef<str>,
    {
        let entry = self.get_table_mut(table.as_ref())?;
        let entry = entry.remove_entry(key.as_ref()).ok_or_else(|| TomlError::EntryNotFound {
            table: table.as_ref().into(),
            key: key.as_ref().into(),
        })?;
        Ok(entry)
    }

    /// Get target table in document.
    ///
    /// Return reference to target table in document.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    pub(crate) fn get_table(&self, key: &str) -> Result<&Table, TomlError> {
        debug!("Get TOML table '{key}'");
        let table =
            self.doc.get(key).ok_or_else(|| TomlError::TableNotFound { table: key.into() })?;
        let table = table.as_table().ok_or_else(|| TomlError::NotTable { table: key.into() })?;
        Ok(table)
    }

    /// Get mutable target table in document.
    ///
    /// Return mutable reference to target table in document.
    ///
    /// # Errors
    ///
    /// - Return [`TomlError::TableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`TomlError::NotTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`TomlError::TableNotFound`]: crate::config::TomlError::TableNotFound
    /// [`TomlError::NotTable`]: crate::config::TomlError::NotTable
    pub(crate) fn get_table_mut(&mut self, key: &str) -> Result<&mut Table, TomlError> {
        debug!("Get mutable TOML table '{key}'");
        let table =
            self.doc.get_mut(key).ok_or_else(|| TomlError::TableNotFound { table: key.into() })?;
        let table =
            table.as_table_mut().ok_or_else(|| TomlError::NotTable { table: key.into() })?;
        Ok(table)
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

impl FromStr for Toml {
    type Err = TomlError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let doc: DocumentMut = data.parse().map_err(|err| TomlError::BadParse { source: err })?;
        Ok(Self { doc })
    }
}

/// Error types for [`Toml`].
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum TomlError {
    #[error("Failed to parse TOML data")]
    BadParse { source: toml_edit::TomlError },

    #[error("TOML table '{table}' not found")]
    TableNotFound { table: String },

    #[error("TOML table '{table}' not defined as a table")]
    NotTable { table: String },

    #[error("TOML entry '{key}' not found in table '{table}'")]
    EntryNotFound { table: String, key: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use indoc::{formatdoc, indoc};
    use toml_edit::Value;
    use rstest::{fixture, rstest};

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
    fn toml_parse_str_accept_good_toml_format(
        #[values("this = 'will parse'", "[so_will_this]", "hello.world = 'from ricer!'")] input: &str,
    ) -> Result<()> {
        let toml: Result<Toml, TomlError> = input.parse();
        assert!(toml.is_ok());
        Ok(())
    }

    #[rstest]
    fn toml_parse_str_return_err_bad_parse(
        #[values("this 'will fail'", "[will # also fail", "not.gonna = [work]")] input: &str,
    ) {
        let result: Result<Toml, TomlError> = input.parse();
        assert!(matches!(result.unwrap_err(), TomlError::BadParse { .. }));
    }

    #[rstest]
    #[case("test", "foo", (Key::new("foo"), Item::Value(Value::from("hello"))))]
    #[case("test", "bar", (Key::new("bar"), Item::Value(Value::from(true))))]
    fn toml_get_return_key_item(
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
    fn toml_get_return_err(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
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
    fn toml_add_return_none(
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
    fn toml_add_return_some_key_item(
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
    fn toml_add_return_err(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
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
    fn toml_rename_return_old_key_value(
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
    fn toml_rename_return_err(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
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
    fn toml_remove_return_deleted_key_item(
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
    fn toml_remove_return_err(#[case] input: &str, #[case] expect: TomlError) -> Result<()> {
        let toml: Toml = input.parse()?;
        let result = toml.get("foo", "bar");
        assert_eq!(result.unwrap_err(), expect);
        Ok(())
    }
}
