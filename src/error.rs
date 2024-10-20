// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Error types and handling.
//!
//! This module defines various error types that can be encountered inside
//! Ricer's internal library. The `anyhow::Result` type is used to indicate
//! that the caller of a function/method is not expected to perform error
//! recovery, even if there is a method to do so. The `RicerResult` type is
//! used to tell the caller that they are responsible for performing error
//! recovery when they can.

use anyhow::Result;
use thiserror::Error;

/// Use this to indicate that an error is potentially recoverable.
pub(crate) type RicerResult<T> = Result<T, RicerError>;

/// Standard internal error types for Ricer.
#[derive(Error, Debug)]
pub enum RicerError {
    /// Error deemed unrecoverable.
    ///
    /// Generally used to express the following situations:
    /// - Despite all efforts, there is not way to recover from this error.
    /// - The possible method(s) of recovery were deemed to expensive to perform.
    #[error(transparent)]
    Unrecoverable(#[from] anyhow::Error),
}

/// Standard exit codes for Ricer.
#[derive(Debug)]
pub enum ExitCode {
    Success,
    Failure,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        match code {
            ExitCode::Success => 0,
            ExitCode::Failure => 1,
        }
    }
}
