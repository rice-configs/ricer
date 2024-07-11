// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli, RunHookOpts};

/// Context for list command.
#[derive(Debug)]
pub struct ListContext {
    /// Show all tracked files in repositories.
    pub tracked: bool,

    /// Show all untracked files in repositories.
    pub untracked: bool,

    /// Action to take when executing a hook specific to this command.
    pub run_cmd_hook: RunHookOpts,

    /// Action to take when executing a repository hook.
    pub run_repo_hook: RunHookOpts,
}

impl From<RicerCli> for ListContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli {cmd_opts, cmd_set, ..} = opts;
        let cmd_set = match cmd_set {
            CommandSet::List(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'list'!"),
        };

        Self {
            tracked: cmd_set.tracked,
            untracked: cmd_set.untracked,
            run_cmd_hook: cmd_opts.run_cmd_hook,
            run_repo_hook: cmd_opts.run_repo_hook,
        }
    }
}
