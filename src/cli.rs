// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use anyhow::Result;
use clap::{Args, Parser, ValueEnum, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(about, long_about = None, version)]
pub struct RicerCli {
    #[command(flatten, next_help_heading = "Logging Options")]
    pub log_opts: Verbosity<InfoLevel>,

    #[command(flatten)]
    pub cmd_opts: CommandOpts,

    #[command(subcommand)]
    pub cmd_set: CommandSet,
}

impl RicerCli {
    pub fn new_run() -> Result<()> {
        let opts = RicerCli::parse();
        env_logger::Builder::new()
            .format_timestamp(None)
            .filter_level(opts.log_opts.log_level_filter())
            .init();

        Ok(())
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Command Options")]
pub struct CommandOpts {
    /// Hook execution option.
    #[arg(default_value_t = RunHooksOpts::All, long, short, value_enum)]
    pub run_hooks: RunHooksOpts,

    /// Target repository to use following command on.
    pub repo: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum RunHooksOpts {
    /// Run global and repository hooks.
    All,

    /// Run global hooks only.
    GlobalOnly,

    /// Run repository hooks only.
    RepoOnly,

    /// Do not run any hooks.
    Never,
}

#[derive(Debug, Subcommand)]
pub enum CommandSet {
    /// Add files to repository(s).
    Add,

    /// Commit changes to repository(s).
    Commit,

    /// Push changes to remote(s).
    Push,

    /// Pull changes from remote(s).
    Pull,

    /// Initialize a new repository.
    Init,

    /// Clone existing repository from a remote.
    Clone,

    /// Delete existing repository(s).
    Delete,

    /// Rename existing repository.
    Rename,

    /// Show current status of repository(s).
    Status,

    /// List current set of repositorys.
    List,

    /// Enter a repository for direct modification.
    Enter,
}
