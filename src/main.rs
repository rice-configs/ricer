// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use ricer::cli::Cli;
use ricer::context::Context;
use ricer::hook::{CmdHook, HookKind};
use ricer::locate::{DefaultLocator, XdgDirLayout};

use anyhow::Result;
use log::{error, LevelFilter};
use std::ffi::OsString;

fn main() {
    env_logger::Builder::new()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(LevelFilter::max())
        .format_indent(Some(8))
        .init();

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
    let opts = Cli::parse_args(args())?;
    log::set_max_level(opts.log_opts.log_level_filter());

    let ctx = Context::from(opts);
    let layout = XdgDirLayout::layout()?;
    let locator = DefaultLocator::locate(layout);
    let hook_mgr = CmdHook::load(&ctx, &locator)?;
    hook_mgr.run_hooks(HookKind::Pre)?;
    hook_mgr.run_hooks(HookKind::Post)?;

    Ok(ExitCode::Success)
}

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
