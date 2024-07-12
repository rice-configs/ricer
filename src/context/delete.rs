// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Options for delete command.
#[derive(Debug)]
pub struct DeleteContext {
    /// Target repository to delete.
    pub repo: String,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for DeleteContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Delete(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'delete'!"),
        };

        Self {
            repo: cmd_set.repo,
            shared: shared_opts.into(),
        }
    }
}
