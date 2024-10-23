// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

mod error;

#[doc(inline)]
pub use error::*;

use log::{debug, info, trace};
use std::fmt;
use std::str::FromStr;
use toml_edit::{DocumentMut, Item, Key, Table};

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

    pub fn rename<S>(&mut self, table: S, from: S, to: S) -> Result<(Key, Item), TomlError>
    where
        S: AsRef<str>,
    {
        todo!();
    }

    pub fn remove<S>(&mut self, table: S, key: S) -> Result<(Key, Item), TomlError>
    where
        S: AsRef<str>,
    {
        todo!();
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
