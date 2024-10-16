// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::ffi::OsString;

use crate::context::HookAction;

#[derive(Debug, Parser)]
#[command(
    about,
    long_about = None,
    subcommand_help_heading = "Ricer Command Set",
    version,
    term_width = 80
)]
pub struct Cli {
    #[command(flatten, next_help_heading = "Logging Options")]
    pub log_opts: Verbosity<InfoLevel>,

    #[command(flatten)]
    pub shared_opts: SharedOpts,
    
    #[command(subcommand)]
    pub cmd_set: CmdSet,
}

impl Cli {
    pub fn parse_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Command Options")]
pub struct SharedOpts {

}

#[derive(Debug, Subcommand)]
pub enum CmdSet {

}
