// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! User interface for Ricer binary.
//!
//! This module provides user interface implementations for Ricer's binary. The
//! CLI is implemented here. The general design of Ricer's CLI boils down to:
//!
//! ```markdown
//! # ricer [OPTIONS] <COMMAND> [CMD_ARGS]
//! ```
//!
//! Where `[OPTIONS]` are top-level options that are shareable with most of
//! Ricer's command set, `<COMMAND>` is the name of the Ricer command, and
//! `[CMD_ARGS]` are the arguments to execute with.

use crate::context::{FixupAction, HookAction};
use crate::error::RicerResult;

use anyhow::anyhow;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use indoc::indoc;
use std::ffi::OsString;

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
    pub shared_opts: SharedOptions,

    #[command(subcommand)]
    pub cmd_set: CommandSet,
}

impl Cli {
    /// Parse a set of command-line arguments.
    ///
    /// # Errors
    ///
    /// Will return [`RicerError::General`] for invalid command-line arguments.
    ///
    /// [`RicerError::General`]: crate::error::RicerError::General
    pub fn parse_args<I, T>(args: I) -> RicerResult<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(args).map_err(|e| anyhow!("{}", e).into())
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Command Options")]
pub struct SharedOptions {
    #[arg(default_value_t = HookAction::default(), long, short, value_enum, value_name = "ACTION")]
    pub run_hook: HookAction,
}

#[derive(Debug, Subcommand)]
pub enum CommandSet {
    /// Bootstrap available repository configurations.
    Bootstrap(BootstrapOptions),

    /// Clone existing repository from a remote.
    Clone(CloneOptions),

    /// Commit changes to all repositories.
    Commit(CommitOptions),

    /// Delete target repository.
    Delete(DeleteOptions),

    /// Enter a target repository.
    Enter(EnterOptions),

    /// Initialize a new repository.
    Init(InitOptions),

    /// List current set of repositories.
    List(ListOptions),

    /// Push changes from all repositories.
    Push(PushOptions),

    /// Pull changes to all repositories.
    Pull(PullOptions),

    /// Rename a repository.
    Rename(RenameOptions),

    /// Show status of repositories.
    Status(StatusOptions),

    /// Run user's Git binary on target repository.
    #[command(external_subcommand)]
    Git(Vec<OsString>),
}

#[derive(Args, Debug)]
pub struct BootstrapOptions {
    /// Activate bootstrap wizard to configure target repository.
    #[arg(long, short, value_name = "REPO")]
    pub config: Option<String>,

    /// Bootstrap from core remote.
    #[arg(long, short, value_name = "URL")]
    pub from: Option<String>,

    /// Bootstrap only a set of specific repositories.
    #[arg(long, short, value_name = "REPOS", num_args = 1.., value_delimiter = ',')]
    pub only: Option<Vec<String>>,
}

#[derive(Args, Debug)]
pub struct CommitOptions {
    /// Amend or reword current commit.
    #[arg(long, short, value_name = "ACTION", value_enum)]
    pub fixup: Option<FixupAction>,

    /// Use MSG as the commit message.
    #[arg(long, short, value_name = "MSG")]
    pub message: Option<String>,
}

#[derive(Args, Debug)]
pub struct CloneOptions {
    /// Remove to clone from.
    pub remote: String,

    /// Set name of cloned repository.
    pub repo: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteOptions {
    /// Target repository to delete.
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct EnterOptions {
    /// Target repository to enter.
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct InitOptions {
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
pub struct ListOptions {
    /// Show all tracked files in repositories.
    #[arg(short, long)]
    pub tracked: bool,

    /// Show all untracked files in repositories.
    #[arg(short, long)]
    pub untracked: bool,
}

#[derive(Args, Debug)]
pub struct PushOptions {
    /// Target remote to push to.
    pub remote: Option<String>,

    /// Target branch to push to.
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct PullOptions {
    /// Target remote to push to.
    pub remote: Option<String>,

    /// Target branch to push to.
    pub branch: Option<String>,
}

#[derive(Args, Debug)]
pub struct RenameOptions {
    /// Target repository to rename.
    pub from: String,

    /// New name to give target repository.
    pub to: String,
}

#[derive(Args, Debug)]
pub struct StatusOptions {
    /// Give a short status report.
    #[arg(long, short)]
    pub terse: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_structure() {
        Cli::command().debug_assert();
    }
}
