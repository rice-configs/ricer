// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Context state for push command.
#[derive(Debug)]
pub struct PushContext {
    /// Target remote to push too.
    pub remote: Option<String>,

    /// Target branch to push too.
    pub branch: Option<String>,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for PushContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Push(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'push'!"),
        };

        Self { remote: cmd_set.remote, branch: cmd_set.branch, shared: shared_opts.into() }
    }
}
