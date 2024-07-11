// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli, RunHookOpts};

/// Options for delete command.
#[derive(Debug)]
pub struct DeleteContext {
    /// Target repository to delete.
    pub repo: String,

    /// Action to take when executing a hook specific to this command.
    pub run_cmd_hook: RunHookOpts,

    /// Action to take when executing a repository hook.
    pub run_repo_hook: RunHookOpts,
}

impl From<RicerCli> for DeleteContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { cmd_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Delete(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'delete'!"),
        };

        Self {
            repo: cmd_set.repo,
            run_cmd_hook: cmd_opts.run_cmd_hook,
            run_repo_hook: cmd_opts.run_repo_hook,
        }
    }
}
