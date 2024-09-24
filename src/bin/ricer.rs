// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Ricer binary implementation.
//!
//! Bringing everything from Ricer's core internal API together to create a
//! functional binary.

use anyhow::Result;
use log::error;
use std::ffi::OsString;

use ricer::cli::RicerCli;
use ricer::context::Context;

fn main() {
    std::process::exit(
        match run_ricer(std::env::args_os) {
            Ok(exit_code) => exit_code,
            Err(error) => {
                error!("{:?}", error);
                ExitCode::Failure
            }
        }
        .into(),
    );
}

fn run_ricer<I, F>(args: F) -> Result<ExitCode>
where
    I: IntoIterator<Item = OsString>,
    F: FnOnce() -> I + Clone,
{
    let opts = RicerCli::parse_args(args());
    env_logger::Builder::new()
        .format_target(false)
        .format_timestamp(None)
        .format_indent(Some(8))
        .filter_level(opts.log_opts.log_level_filter())
        .init();

    let _ctx = Context::from(opts);

    // TODO: match and execute command in Ricer's command set...

    Ok(ExitCode::Success)
}

enum ExitCode {
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
