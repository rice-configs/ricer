// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

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
