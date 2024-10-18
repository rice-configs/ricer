// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use log::error;
use std::ffi::OsString;

use ricer::context::Context;
use ricer::error::ExitCode;
use ricer::ui::Cli;

fn main() {
    let code = match run_ricer(std::env::args_os) {
        Ok(code) => code,
        Err(err) => {
            error!("{:?}", err);
            ExitCode::Failure
        }
    }
    .into();

    std::process::exit(code);
}

fn run_ricer<I, F>(args: F) -> Result<ExitCode>
where
    I: IntoIterator<Item = OsString>,
    F: FnOnce() -> I + Clone,
{
    let opts = Cli::parse_args(args());
    env_logger::Builder::new()
        .format_target(false)
        .format_timestamp(None)
        .format_indent(Some(8))
        .filter_level(opts.log_opts.log_level_filter())
        .init();

    let ctx = Context::from(opts);
    println!("{:#?}", ctx);

    Ok(ExitCode::Success)
}
