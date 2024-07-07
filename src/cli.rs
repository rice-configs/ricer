// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Current command set of Ricer.
#[derive(Debug, Subcommand)]
pub enum CommandSet {
    /// Initialize a new "fake-bare" Git repository.
    Init,
}

/// Command-line interface.
///
/// Parses and executes the command-line arguments passed to the Ricer binary by
/// the user. Provides a git-like interface, with specialized commands to make
/// it easier to manage and organize rice configurations. See [`CommandSet`] for
/// full command set that Ricer's interface offers.
#[derive(Debug, Parser)]
pub struct Cli {
    // Control log level of binary...
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    // Link command set...
    #[command(subcommand)]
    cmd: CommandSet,
}

impl Cli {
    pub fn new_run() -> Result<String> {
        todo!();
    }
}
