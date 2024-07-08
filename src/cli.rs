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
    /// Parse and execute command set.
    ///
    /// Constructs new instance of Ricer's CLI, then parses and executes the
    /// command-line arguments the user passes in. If no errors occur, then this
    /// method provides any output that needs to be written to stdout.
    /// Otherwise, this method provides any output that details what went wrong
    /// for stderr.
    ///
    /// # Preconditions
    ///
    /// 1. User provides correct commands and arguments to execute.
    ///
    /// # Postconditions
    ///
    /// 1. Execute command with any arguments user passed.
    /// 1. Initialize logger to report what the Ricer binary is doing depending
    ///    on the verbosity level set by the user.
    ///
    /// # Invariants
    ///
    /// 1. Do not let `main` panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer::cli::Cli
    ///
    /// let out = Cli::new_run().expect("Failed to run Ricer");
    /// println!("{}", out);
    /// ```
    pub fn new_run() -> Result<String> {
        let args = Cli::parse();
        env_logger::Builder::new()
            .format_timestamp(None)
            .filter_level(args.verbose.log_level_filter())
            .init();

        let out = match args.cmd {
            CommandSet::Init => "todo".to_string(),
        };

        Ok(out)
    }
}
