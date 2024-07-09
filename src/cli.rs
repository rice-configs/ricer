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
    Add(AddOpts),

    /// Commit changes to repository(s).
    Commit(CommitOpts),

    /// Push changes to remote(s).
    Push(PushOpts),

    /// Pull changes from remote(s).
    Pull(PullOpts),

    /// Initialize a new repository.
    Init(InitOpts),

    /// Clone existing repository from a remote.
    Clone(CloneOpts),

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

#[derive(Args, Debug)]
pub struct AddOpts {
    /// Files to add content from.
    pub path_spec: Vec<String>,

    /// Do not add the file(s), just show if they exist and/or will be ignored.
    #[arg(long, short = 'n')]
    pub dry_run: bool,

    /// Stages modified and deleted files only, not new files.
    #[arg(long, short)]
    pub update: bool,
}

#[derive(Args, Debug)]
pub struct CommitOpts {
    /// Amend or reword current commit.
    #[arg(long, short, value_enum)]
    pub fixup: Option<FixupOpts>,

    /// Use <MSG> as the commit message.
    #[arg(long, short, value_name = "MSG")]
    pub message: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum FixupOpts {
    /// Ammend the changes made by the current commit.
    Amend,

    /// Reword the current commit.
    Reword,
}

#[derive(Args, Debug)]
pub struct PushOpts {
    /// Target remote to push too.
    pub remote: Option<String>,

    /// Target branch to push too.
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct PullOpts {
    /// Target remote to pull from.
    pub remote: Option<String>,

    /// Target branch to pull from.
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct InitOpts {
    /// Name of repository to initialize.
    pub name: String,

    /// Set initial remote to <ORIGIN>.
    #[arg(short = 'r', long, value_name = "ORIGIN")]
    pub initial_remote: Option<String>,

    /// Set initial branch to <BRANCH>.
    #[arg(short = 'b', long, value_name = "BRANCH")]
    pub initial_branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct CloneOpts {
    /// Remote to clone from.
    pub remote: String,

    /// Set name of cloned repository.
    pub repo: Option<String>,

    /// Clone from a branch.
    #[arg(short, long)]
    pub branch: Option<String>,
}
