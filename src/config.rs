// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::{RicerError, RicerResult};

use anyhow::anyhow;
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

    pub fn add(
        &mut self,
        table: impl AsRef<str>,
        entry: (Key, Item),
    ) -> RicerResult<Option<(Key, Item)>> {
        todo!();
    }

    /// Get entry from target table in document.
    ///
    /// Return reference to full key-value pair in document.
    ///
    /// # Errors
    ///
    /// - Return [`RicerError::TomlTableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`RicerError::TomlNonTable`] if target table was not defined as
    ///   a table.
    /// - Return [`RicerError::TomlKeyValueNotFound`] if target key-value pair
    ///   is not found in document.
    ///
    /// [`RicerError::TomlTableNotFound`]: crate::error::RicerError::TomlTableNotFound
    /// [`RicerError::TomlNonTable`]: crate::error::RicerError::TomlNonTable
    /// [`RicerError::TomlKeyValueNotFound`]: crate::error::RicerError::TomlKeyValueNotFound
    pub fn get<S>(&self, table: S, key: S) -> RicerResult<(&Key, &Item)>
    where
        S: AsRef<str>,
    {
        info!("Get TOML entry '{}' from '{}' table", key.as_ref(), table.as_ref());
        let entry = self.get_table(table.as_ref())?;
        let entry = entry
            .get_key_value(key.as_ref())
            .ok_or_else(|| {
                anyhow!("TOML entry '{}' not found in '{}' table", key.as_ref(), table.as_ref())
            })
            .map_err(RicerError::TomlKeyValueNotFound)?;
        Ok(entry)
    }

    pub fn rename<S>(&mut self, table: S, from: S, to: S) -> RicerResult<(Key, Item)>
    where
        S: AsRef<str>,
    {
        todo!();
    }

    pub fn remove<S>(&mut self, table: S, key: S) -> RicerResult<(Key, Item)>
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
    /// - Return [`RicerError::TomlTableNotFound`] if target table is not found
    ///   in document.
    /// - Return [`RicerError::TomlNonTable`] if target table was not defined as
    ///   a table.
    ///
    /// [`RicerError::TomlTableNotFound`]: crate::error::RicerError::TomlTableNotFound
    /// [`RicerError::TomlNonTable`]: crate::error::RicerError::TomlNonTable
    pub(crate) fn get_table(&self, key: &str) -> RicerResult<&Table> {
        debug!("Get TOML table '{key}'");
        let table = self
            .doc
            .get(key)
            .ok_or_else(|| anyhow!("TOML entry '{key}' does not exist"))
            .map_err(RicerError::TomlTableNotFound)?;
        let table = table
            .as_table()
            .ok_or_else(|| anyhow!("TOML entry '{key}' is not a table"))
            .map_err(RicerError::TomlNonTable)?;
        Ok(table)
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

impl FromStr for Toml {
    type Err = RicerError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let doc: DocumentMut =
            data.parse().map_err(|e| -> RicerError { anyhow!("{}", e).into() })?;
        Ok(Self { doc })
    }
}
