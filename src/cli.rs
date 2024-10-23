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

mod error;
mod options;

#[doc(inline)]
pub use error::*;
pub use options::*;

use clap::{Parser, Subcommand};
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

/// Command-line interface parser.
///
/// The general design of Ricer's CLI boils down to:
///
/// ```markdown
/// # ricer [OPTIONS] <COMMAND> [CMD_ARGS]
/// ```
///
/// Where `[OPTIONS]` are top-level options that are shareable with most of
/// Ricer's command set, `<COMMAND>` is the name of the Ricer command, and
/// `[CMD_ARGS]` are the arguments to execute with.
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
    /// [`RicerError::General`]: crate::error::RicerError::Unrecoverable
    pub fn parse_args<I, T>(args: I) -> Result<Self, CliError>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(args).map_err(|err| CliError::BadParse { source: err })
    }
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
