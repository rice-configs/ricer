// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::RicerError;

use anyhow::anyhow;
use log::trace;
use std::fmt;
use std::str::FromStr;
use toml_edit::DocumentMut;

#[derive(Clone, Default, Debug)]
pub struct Toml {
    doc: DocumentMut,
}

impl Toml {
    pub fn new() -> Self {
        trace!("Construct new TOML parser");
        Self { doc: DocumentMut::new() }
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
