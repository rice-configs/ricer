// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Ricer binary implementation.
//!
//! Bringing everything from Ricer's core internal API together to create a
//! functional binary.

use anyhow::Result;
use log::error;
use std::ffi::OsString;

use ricer_core::cli::RicerCli;
use ricer_core::context::Context;

/// Starting point of Ricer binary.
///
/// # Postconditions
///
/// 1. Parse and execute Ricer command set.
/// 2. Log any errors.
/// 3. Provide [`ExitCode`] before exiting.
fn main() {
    std::process::exit(
        match run_ricer(std::env::args_os) {
            Ok(exit_code) => exit_code,
            Err(error) => {
                error!("{}", error);
                ExitCode::Failure
            }
        }
        .into(),
    );
}

/// Run Ricer binary.
///
/// # Postconditions
///
/// 1. Parse and execute Ricer command set.
/// 2. Provide [`ExitCode`] after processing.
fn run_ricer<I, F>(args: F) -> Result<ExitCode>
where
    I: IntoIterator<Item = OsString>,
    F: FnOnce() -> I + Clone,
{
    let opts = RicerCli::parse_args(args());
    env_logger::Builder::new()
        .format_timestamp(None)
        .filter_level(opts.log_opts.log_level_filter())
        .init();

    let ctx = Context::from(opts);
    println!("{:?}", ctx);

    // TODO: match and execute command in Ricer's command set...

    Ok(ExitCode::Success)
}

/// General exit code status.
enum ExitCode {
    /// Ricer binary exited with no errors.
    Success,

    /// Ricer binary exited with errors.
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
