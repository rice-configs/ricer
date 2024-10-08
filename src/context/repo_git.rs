// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::{CommandSet, RicerCli};
use std::ffi::OsString;

/// Context for external subcommand shortcut.
#[derive(Debug)]
pub struct RepoGitContext {
    /// Target repository to run Git binary on.
    pub repo: OsString,

    /// Arguments to pass into Git binary.
    pub git_args: Vec<OsString>,
}

impl From<RicerCli> for RepoGitContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::RepoGit(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'enter'!"),
        };

        Self { repo: cmd_set[0].clone(), git_args: cmd_set[1..].to_vec() }
    }
}
