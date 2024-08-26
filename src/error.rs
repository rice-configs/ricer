// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Error system.
//!
//! This module contains all error types that a given module of the Ricer code
//! base may produce. Some of these errors are meant to be recoverable, others
//! are not. It all depends upon the API of a given module.

use anyhow::anyhow;

/// Specify result type that uses [`RicerError`] for `Err`.
pub type RicerResult<T> = Result<T, RicerError>;

/// All possible error types in Ricer's internal API.
#[derive(Debug, thiserror::Error)]
pub enum RicerError {
    /// Configuration directory located, but does not exist.
    ///
    /// Recoverable as the location is known, so a new configuration directory
    /// can simply be created at that location.
    #[error("Configuration directory located at expected path, but does not exist")]
    NoConfigDir(#[source] anyhow::Error),

    /// Error is not recoverable.
    ///
    /// Caller is meant to panic with no expected method of recovery.
    #[error(transparent)]
    Unrecoverable(#[from] anyhow::Error),
}

impl From<std::io::Error> for RicerError {
    fn from(err: std::io::Error) -> RicerError {
        RicerError::Unrecoverable(anyhow!("{}", err))
    }
}

impl From<toml_edit::TomlError> for RicerError {
    fn from(err: toml_edit::TomlError) -> RicerError {
        RicerError::Unrecoverable(anyhow!("{}", err))
    }
}

impl From<run_script::ScriptError> for RicerError {
    fn from(err: run_script::ScriptError) -> RicerError {
        RicerError::Unrecoverable(anyhow!("{}", err))
    }
}

impl From<std::env::VarError> for RicerError {
    fn from(err: std::env::VarError) -> RicerError {
        RicerError::Unrecoverable(anyhow!("{}", err))
    }
}
