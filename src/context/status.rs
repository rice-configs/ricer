// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Context for status command.
#[derive(Debug)]
pub struct StatusContext {
    /// Give a short status report.
    pub terse: bool,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for StatusContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Status(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'status'!"),
        };

        Self { terse: cmd_set.terse, shared: shared_opts.into() }
    }
}
