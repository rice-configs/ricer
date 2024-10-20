// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use log::trace;
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
