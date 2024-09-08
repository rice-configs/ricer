// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::{CommandSet, RicerCli};
use crate::context::SharedContext;

/// Options for rename command.
#[derive(Debug)]
pub struct RenameContext {
    /// Target repository to rename.
    pub old_name: String,

    /// New new to give target repository.
    pub new_name: String,

    /// Shared features.
    pub shared: SharedContext,
}

impl From<RicerCli> for RenameContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Rename(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'rename'!"),
        };

        Self { old_name: cmd_set.old_name, new_name: cmd_set.new_name, shared: shared_opts.into() }
    }
}
