// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::context::{FixupAction, HookAction};

use clap::Args;

#[derive(Debug, Args)]
#[command(next_help_heading = "Command Options")]
pub struct SharedOptions {
    #[arg(default_value_t = HookAction::default(), long, short, value_enum, value_name = "ACTION")]
    pub run_hook: HookAction,
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
