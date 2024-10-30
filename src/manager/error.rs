// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config;
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum LocatorError {
    #[error("Cannot determine path to home directory")]
    NoWayHome,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigManagerError {
    #[error("Failed to make parent directory '{path}' because '{source}'")]
    MakeDirP { source: io::Error, path: PathBuf },

    #[error("Failed to open '{path}' because '{source}'")]
    FileOpen { source: io::Error, path: PathBuf },

    #[error("Failed to read '{path}' because '{source}'")]
    FileRead { source: io::Error, path: PathBuf },

    #[error("Failed to write '{path}' because '{source}'")]
    FileWrite { source: io::Error, path: PathBuf },

    #[error("Failed to parse '{path}' because '{source}'")]
    Toml { source: config::TomlError, path: PathBuf },
}
