// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::ffi::OsString;

use crate::context::{HookAction, FixupAction};

const EXTERNAL_SUBCOMMANDS: &str = r#"
Command Shortcuts:
  <REPO> <GIT_CMD>  Shortcut to run user's Git binary on a target repository
"#;

#[derive(Debug, Parser)]
#[command(
    about,
    after_help = EXTERNAL_SUBCOMMANDS,
    after_long_help = EXTERNAL_SUBCOMMANDS,
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

#[derive(Debug, Subcommand)]
pub enum CmdSet {
    /// Bootstrap available repository configurations.
    Bootstrap(BootstrapOpts),

    /// Commit changes to all repositories.
    Commit(CommitOpts),

    /// Run user's Git binary on target repository.
    #[command(external_subcommand)]
    Git(Vec<OsString>),
}
