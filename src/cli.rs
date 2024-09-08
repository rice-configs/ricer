// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Ricer CLI implementation.
//!
//! Ricer's CLI is mainly designed to allow the user to manage and organize a
//! collection of Git repositories through a Git-like interface. The idea is
//! to have the user modularize their rice/dotfile configurations into
//! individual repositories that are deployed into their home directory through
//! Ricer's CLI. The general design and usage of the CLI boils down to:
//!
//! ```markdown
//! # ricer [OPTIONS] <COMMAND> [CMD_ARGS]
//! ```
//!
//! Where `[OPTIONS]` are top-level options that are shareable with a Ricer
//! command, `<COMMAND>` is the name of the Ricer command, and `[CMD_ARGS]` are
//! the Ricer command's corresponding arguments to execute with.
//!
//! However, Ricer's existing CLI command set only implements a small modified
//! portion of the Git command set. If the user needs to use the full Git
//! command set for a target repository, then they need to use one of two
//! commands offered by Ricer: enter, or external subcommand shortcut.
//!
//! The enter command allows the user to enter a target repository through a
//! sub-shell so they can use the Git binary directly and exit the sub-shell
//! when done.
//!
//! The external subcommand shortcut takes the following form in the CLI:
//!
//! ```markdown
//! # ricer <REPO> <GIT_CMD>
//! ```
//!
//! Where `<REPO>` is the name of the target repository, and `<GIT_CMD>` is a
//! regular Git command to run on the target repository. This external
//! subcommand shortcut allows the user to make modifications to their
//! repositories as an alternative to the enter command.
//!
//! Currently, I have not figured out a way to get clap to document the
//! external subcommand shortcut automatically. My hacky solution is to use the
//! `after_help` and `after_long_help` methods to slap on an explanation of the
//! external subcommand shortcut to the user.

use crate::build;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::ffi::OsString;

use crate::context::{FixupAction, HookAction};

/// Ricer CLI structure.
///
/// Top-level structure that clap will deserialize command-line arguments into.
/// Obtain a valid parse through [`parse_args`].
///
/// [`parse_args`]: #method.parse_args
#[derive(Debug, Parser)]
#[command(
    about,
    after_help = EXTERNAL_SUBCOMMAND_INFORMATION,
    after_long_help = EXTERNAL_SUBCOMMAND_INFORMATION,
    long_about = None,
    subcommand_help_heading = "Ricer Command Set",
    version,
    long_version = build::CLAP_LONG_VERSION,
    term_width = 80
)]
pub struct RicerCli {
    /// Options for logging verbosity.
    #[command(flatten, next_help_heading = "Logging Options")]
    pub log_opts: Verbosity<InfoLevel>,

    /// Options that are shareable across Ricer commands.
    #[command(flatten)]
    pub shared_opts: SharedOptions,

    /// Ricer command set.
    #[command(subcommand)]
    pub cmd_set: CommandSet,
}

impl RicerCli {
    /// Parse command line arguments.
    ///
    /// Panics on error and issues its own error code.
    ///
    /// # Preconditions
    ///
    /// 1. Arguments are iterable.
    /// 2. Arguments are convertible _into_ [`OsString`].
    ///
    /// # Postconditions
    ///
    /// 1. Deserialize arguments into [`RicerCli`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::cli::RicerCli;
    ///
    /// let opts = RicerCli::parse_args(std::env::args_os());
    /// ```
    ///
    /// # See
    ///
    /// 1. <https://docs.rs/clap/latest/clap/trait.Parser.html#method.parse_from>
    /// 2. <https://docs.rs/clap/latest/clap/error/struct.Error.html#method.exit>
    pub fn parse_args<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        RicerCli::parse_from(args)
    }
}

/// Shareable top-level options used by all Ricer commands.
#[derive(Debug, Args)]
#[command(next_help_heading = "Command Options")]
pub struct SharedOptions {
    /// Tell Ricer how you want hooks to be executed.
    #[arg(default_value_t = HookAction::Prompt, long, short, value_enum, value_name = "ACTION")]
    pub run_hook: HookAction,
}

/// Current Ricer command set.
///
/// Git-like command set that provides a limited set of Git functionality. User
/// will either need to use the enter command or the extended subcommand
/// shortcut to gain access to full Git command set through Ricer.
#[derive(Debug, Subcommand)]
pub enum CommandSet {
    /// Commit changes to all repositories.
    Commit(CommitOptions),

    /// Push changes into each repository remote.
    Push(PushOptions),

    /// Pull changes from each repository remote.
    Pull(PullOptions),

    /// Initialize a new repository.
    Init(InitOptions),

    /// Clone existing repository from a remote.
    Clone(CloneOptions),

    /// Delete existing repository.
    Delete(DeleteOptions),

    /// Rename existing repository.
    Rename(RenameOptions),

    /// Show current status of all repositories.
    Status(StatusOptions),

    /// List current set of repositories.
    List(ListOptions),

    /// Enter a repository for direct modification.
    Enter(EnterOptions),

    /// Run user's Git binary on target repository.
    #[command(external_subcommand)]
    RepoGit(Vec<OsString>),
}

/// Options for commit command.
#[derive(Args, Debug)]
pub struct CommitOptions {
    /// Amend or reword current commit.
    #[arg(long, short, value_name = "ACTION", value_enum)]
    pub fixup: Option<FixupAction>,

    /// Use MSG as the commit message.
    #[arg(long, short, value_name = "MSG")]
    pub message: Option<String>,
}

/// Options for push command.
#[derive(Args, Debug)]
pub struct PushOptions {
    /// Target remote to push too.
    pub remote: Option<String>,

    /// Target branch to push too.
    pub branch: Option<String>,
}

/// Options for pull command.
#[derive(Args, Debug)]
pub struct PullOptions {
    /// Target remote to pull from.
    pub remote: Option<String>,

    /// Target branch to pull from.
    pub branch: Option<String>,
}

/// Options for init command.
#[derive(Args, Debug)]
pub struct InitOptions {
    /// Name of repository to initialize.
    pub name: String,

    /// Set initial remote to ORIGIN.
    #[arg(short = 'r', long, value_name = "ORIGIN")]
    pub initial_remote: Option<String>,

    /// Set initial branch to BRANCH.
    #[arg(short = 'b', long, value_name = "BRANCH")]
    pub initial_branch: Option<String>,
}

/// Options for clone command.
#[derive(Args, Debug)]
pub struct CloneOptions {
    /// Remote to clone from.
    pub remote: String,

    /// Set name of cloned repository.
    pub repo: Option<String>,

    /// Clone from a branch.
    #[arg(short, long)]
    pub branch: Option<String>,
}

/// Options for delete command.
#[derive(Args, Debug)]
pub struct DeleteOptions {
    /// Target repository to delete.
    pub repo: String,
}

/// Options for rename command.
#[derive(Args, Debug)]
pub struct RenameOptions {
    /// Target repository to rename.
    pub old_name: String,

    /// New new to give target repository.
    pub new_name: String,
}

/// Options for status command.
#[derive(Args, Debug)]
pub struct StatusOptions {
    /// Give a short status report.
    #[arg(long, short)]
    pub terse: bool,
}

/// Options for list command.
#[derive(Args, Debug)]
pub struct ListOptions {
    /// Show all tracked files in repositories.
    #[arg(short, long)]
    pub tracked: bool,

    /// Show all untracked files in repositories.
    #[arg(short, long)]
    pub untracked: bool,
}

/// Options for enter command.
#[derive(Args, Debug)]
pub struct EnterOptions {
    /// Target repository to enter.
    pub repo: String,
}

const EXTERNAL_SUBCOMMAND_INFORMATION: &str = r#"
Command Shortcuts:
  <REPO> <GIT_CMD>  Shortcut to execute a Git command directly on a target repository.
"#;
