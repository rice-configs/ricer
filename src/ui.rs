// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use indoc::indoc;
use std::ffi::OsString;

use crate::context::{FixupAction, HookAction};

macro_rules! explain_cmd_shortcuts {
    () => {
        indoc! {r#"
        Command Shortcuts:
          <REPO> <GIT_CMD>  Shortcut to run user's Git binary on a target repository
        "#}
    };
}

#[derive(Debug, Parser)]
#[command(
    about,
    after_help = explain_cmd_shortcuts!(),
    after_long_help = explain_cmd_shortcuts!(),
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
    #[arg(default_value_t = HookAction::default(), long, short, value_enum, value_name = "ACTION")]
    pub run_hook: HookAction,
}

#[derive(Args, Debug)]
pub struct BootstrapOpts {
    /// Activate bootstrap wizard to configure target repository.
    #[arg(long, short, value_name = "REPO")]
    pub config: Option<String>,

    /// Bootstrap from core remote.
    #[arg(long, short, value_name = "URL")]
    pub from: Option<String>,

    /// Bootstrap only a set of specific repositories.
    #[arg(long, short, value_name = "REPOS")]
    pub only: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct CommitOpts {
    /// Amend or reword current commit.
    #[arg(long, short, value_name = "ACTION", value_enum)]
    pub fixup: Option<FixupAction>,

    /// Use MSG as the commit message.
    #[arg(long, short, value_name = "MSG")]
    pub message: Option<String>,
}

#[derive(Args, Debug)]
pub struct CloneOpts {
    /// Remove to clone from.
    pub remote: String,

    /// Set name of cloned repository.
    pub repo: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteOpts {
    /// Target repository to delete.
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct EnterOpts {
    /// Target repository to enter.
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct InitOpts {
    /// Name of repository to initialize.
    pub name: String,

    /// Use $HOME as working directory.
    #[arg(short, long)]
    pub workdir_home: bool,

    /// Set default branch to use.
    #[arg(short, long, value_name = "BRANCH")]
    pub branch: Option<String>,

    /// Set default remote to use.
    #[arg(short, long, value_name = "ORIGIN")]
    pub remote: Option<String>,
}

#[derive(Args, Debug)]
pub struct ListOpts {
    /// Show all tracked files in repositories.
    #[arg(short, long)]
    pub tracked: bool,

    /// Show all untracked files in repositories.
    #[arg(short, long)]
    pub untracked: bool,
}

#[derive(Args, Debug)]
pub struct PushOpts {
    /// Target remote to push to.
    pub remote: Option<String>,

    /// Target branch to push to.
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct PullOpts {
    /// Target remote to push to.
    pub remote: Option<String>,

    /// Target branch to push to.
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct RenameOpts {
    /// Target repository to rename.
    pub from: String,

    /// New name to give target repository.
    pub to: String,
}

#[derive(Args, Debug)]
pub struct StatusOpts {
    /// Give a short status report.
    #[arg(long, short)]
    pub terse: bool,
}

#[derive(Debug, Subcommand)]
pub enum CmdSet {
    /// Bootstrap available repository configurations.
    Bootstrap(BootstrapOpts),

    /// Clone existing repository from a remote.
    Clone(CloneOpts),

    /// Commit changes to all repositories.
    Commit(CommitOpts),

    /// Delete target repository.
    Delete(DeleteOpts),

    /// Enter a target repository.
    Enter(EnterOpts),

    /// Initialize a new repository.
    Init(InitOpts),

    /// List current set of repositories.
    List(ListOpts),

    /// Push changes from all repositories.
    Push(PushOpts),

    /// Pull changes to all repositories.
    Pull(PullOpts),

    /// Rename a repository.
    Rename(RenameOpts),

    /// Show status of repositories.
    Status(StatusOpts),

    /// Run user's Git binary on target repository.
    #[command(external_subcommand)]
    Git(Vec<OsString>),
}

#[cfg(test)]
mod tests {}
