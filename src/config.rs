// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::{RicerError, RicerResult};

use anyhow::anyhow;
use log::trace;
use std::fmt;
use std::str::FromStr;
use toml_edit::{DocumentMut, Key, Item};

#[derive(Clone, Default, Debug)]
pub struct Toml {
    doc: DocumentMut,
}

impl Toml {
    pub fn new() -> Self {
        trace!("Construct new TOML parser");
        Self { doc: DocumentMut::new() }
    }

    pub fn add(&mut self, table: impl AsRef<str>, entry: (Key, Item)) -> RicerResult<Option<(Key, Item)>> {
        todo!();
    }

    pub fn get<S>(&self, table: S, key: S) -> RicerResult<(&Key, &Item)>
    where
        S: AsRef<str>,
    {
        todo!();
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
