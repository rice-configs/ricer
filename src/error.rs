// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Error system.
//!
//! This module contains all error types that a given module of the Ricer code
//! base may produce. Some of these errors are meant to be recoverable, others
//! are not. It all depends upon the API of a given module.

#[derive(Debug, thiserror::Error)]
pub enum RicerError {
    #[error("Failed to interpret configuration data")]
    ConfigError(#[from] anyhow::Error),
}
