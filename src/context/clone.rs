// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Context for clone command.
#[derive(Debug)]
pub struct CloneContext {
    /// Remote to clone from.
    pub remote: String,

    /// Set name of cloned repository.
    pub repo: Option<String>,

    /// Clone from a branch.
    pub branch: Option<String>,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for CloneContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Clone(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'init'!"),
        };

        Self {
            remote: cmd_set.remote,
            repo: cmd_set.repo,
            branch: cmd_set.branch,
            shared: shared_opts.into(),
        }
    }
}
