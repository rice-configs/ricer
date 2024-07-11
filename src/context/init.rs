// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli, RunHookOpts};

/// Context state for init command.
#[derive(Debug)]
pub struct InitContext {
    /// Name of repository to initialize.
    pub name: String,

    /// Set initial remote.
    pub initial_remote: Option<String>,

    /// Set initial branch.
    pub initial_branch: Option<String>,

    /// Action to take when executing a hook specific to this command.
    pub run_cmd_hook: RunHookOpts,

    /// Action to take when executing a repository hook.
    pub run_repo_hook: RunHookOpts,
}

impl From<RicerCli> for InitContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli {cmd_opts, cmd_set, ..} = opts;
        let cmd_set = match cmd_set {
            CommandSet::Init(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'push'!"),
        };

        Self {
            name: cmd_set.name,
            initial_remote: cmd_set.initial_remote,
            initial_branch: cmd_set.initial_branch,
            run_cmd_hook: cmd_opts.run_cmd_hook,
            run_repo_hook: cmd_opts.run_repo_hook,
        }
    }
}
