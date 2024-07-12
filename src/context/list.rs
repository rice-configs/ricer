// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Context for list command.
#[derive(Debug)]
pub struct ListContext {
    /// Show all tracked files in repositories.
    pub tracked: bool,

    /// Show all untracked files in repositories.
    pub untracked: bool,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for ListContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::List(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'list'!"),
        };

        Self {
            tracked: cmd_set.tracked,
            untracked: cmd_set.untracked,
            shared: shared_opts.into(),
        }
    }
}
