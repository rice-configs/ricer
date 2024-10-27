// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

mod error;
mod locator;

#[doc(inline)]
pub use error::*;
pub use locator::*;

use crate::config::{Toml, Repository, TomlError};

#[cfg(test)]
use mockall::automock;

/// TOML serialization and deserialization manager.
///
/// Interface to simplify serialization and deserialization of parsed TOML data.
///
/// # See also
///
/// - [`Toml`]
///
/// [`Toml`]: crate::config::Toml
#[cfg_attr(test, automock(type Entry = Repository;))]
pub trait TomlManager {
    type Entry;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError>;
    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError>;
}
