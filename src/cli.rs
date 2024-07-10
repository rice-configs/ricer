// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::build;
use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use const_format::formatcp;
use indoc::indoc;
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(
    about,
    long_about = None,
    subcommand_help_heading = "Ricer Command Set",
    version,
    long_version = VERSION_INFORMATION,
    term_width = 80
)]
pub struct RicerCli {
    /// Options for logging verbosity.
    #[command(flatten, next_help_heading = "Logging Options")]
    pub log_opts: Verbosity<InfoLevel>,

    /// Options that are shareable across Ricer commands.
    #[command(flatten)]
    pub cmd_opts: CommandOpts,

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

#[derive(Debug, Args)]
#[command(next_help_heading = "Command Options")]
pub struct CommandOpts {
    /// Hook execution option.
    #[arg(default_value_t = RunHooksOpts::All, long, short, value_enum)]
    pub run_hooks: RunHooksOpts,
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
    /// Commit changes to all repository.
    Commit(CommitOpts),

    /// Push changes to each repository remote.
    Push(PushOpts),

    /// Pull changes from each repository remote.
    Pull(PullOpts),

    /// Initialize a new repository.
    Init(InitOpts),

    /// Clone existing repository from a remote.
    Clone(CloneOpts),

    /// Delete existing repository(s).
    Delete(DeleteOpts),

    /// Rename existing repository.
    Rename(RenameOpts),

    /// Show current status of repository(s).
    Status(StatusOpts),

    /// List current set of repositories.
    List(ListOpts),

    /// Enter a repository for direct modification.
    Enter(EnterOpts),

    /// Run user's Git binary on target repository.
    #[command(external_subcommand)]
    UseGitBinOnRepo(Vec<OsString>),
}

#[derive(Args, Debug)]
pub struct CommitOpts {
    /// Amend or reword current commit.
    #[arg(long, short, value_enum)]
    pub fixup: Option<FixupOpts>,

    /// Use MSG as the commit message.
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

    /// Set initial remote to ORIGIN.
    #[arg(short = 'r', long, value_name = "ORIGIN")]
    pub initial_remote: Option<String>,

    /// Set initial branch to BRANCH.
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

#[derive(Args, Debug)]
pub struct DeleteOpts {
    /// Target repository to delete.
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct RenameOpts {
    /// Target repository to rename.
    pub old_name: String,

    /// New new to give target repository.
    pub new_name: String,
}

#[derive(Args, Debug)]
pub struct StatusOpts {
    /// Give a short status report.
    pub terse: bool,
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
pub struct EnterOpts {
    /// Target repository to enter.
    pub repo: String,
}

const GPL_BOILERPLATE: &str = indoc! {"
    Copyright (C) 2024 Jason Pena <jasonpena@awkless.com>

    The Ricer program is free software; you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation; either version 2 of the License, or (at your option) any
    later version.

    This program also uses the GPL Cooperation Commitment version 1.0 to give itself
    the cure and reinstatement clauses offered by the GNU GPL version 3 to avoid
    instant termination of its GPL license for any reported violations.

    This program is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License and the
    Cooperation Commitment along with Ricer; if not, write to the Free Software
    Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
    "
};

const VERSION_INFORMATION: &str = formatcp!("{}\n\n{GPL_BOILERPLATE}", build::CLAP_LONG_VERSION);
