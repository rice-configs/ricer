// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Context for enter command.
#[derive(Debug)]
pub struct EnterContext {
    /// Target repository to enter.
    pub repo: String,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for EnterContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Enter(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'enter'!"),
        };

        Self { repo: cmd_set.repo, shared: shared_opts.into() }
    }
}
