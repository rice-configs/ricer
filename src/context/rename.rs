// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, RicerCli, RunHookOpts};

/// Options for rename command.
#[derive(Debug)]
pub struct RenameContext {
    /// Target repository to rename.
    pub old_name: String,

    /// New new to give target repository.
    pub new_name: String,

    /// Action to take when executing a hook specific to this command.
    pub run_cmd_hook: RunHookOpts,

    /// Action to take when executing a repository hook.
    pub run_repo_hook: RunHookOpts,
}

impl From<RicerCli> for RenameContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { cmd_opts, cmd_set, ..} = opts;
        let cmd_set = match cmd_set {
            CommandSet::Rename(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'rename'!"),
        };

        Self {
            old_name: cmd_set.old_name,
            new_name: cmd_set.new_name,
            run_cmd_hook: cmd_opts.run_cmd_hook,
            run_repo_hook: cmd_opts.run_repo_hook,
        }
    }
}
