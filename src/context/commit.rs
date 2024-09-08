// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;
use clap::ValueEnum;

/// Context state for commit command.
#[derive(Debug)]
pub struct CommitContext {
    /// Amend or reword current commit.
    pub fixup: Option<FixupAction>,

    /// Use a string as commit rather than opening up the user's text editor.
    pub message: Option<String>,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for CommitContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Commit(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'commit'!"),
        };

        Self { fixup: cmd_set.fixup, message: cmd_set.message, shared: shared_opts.into() }
    }
}

/// Fixup options for commit command.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum FixupAction {
    /// Amend changes of current commit. Any changes to index will be added to
    /// the commit.
    Amend,

    /// Reword current commit. Automatically opens user's text editor to edit
    /// the commit. No changes to index will be added to the commit.
    Reword,
}
