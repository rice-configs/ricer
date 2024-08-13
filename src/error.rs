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

    /// Issued error is not recoverable.
    ///
    /// Caller is either expected to pass this error up the call chain, or
    /// panic as there is no expected way to recover from this error.
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
