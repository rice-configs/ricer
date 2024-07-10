// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::{CommandSet, FixupOpts, RicerCli, RunHookOpts};

/// Context state for commit command.
///
/// # Preconditions
///
/// 1. The [`amend`] and [`reword`] flags cannot both be true.
///
/// > __NOTE__: Precondition 1 will always hold true, because the structure of
/// > [`RicerCli`] makes it impossible for the user to set both [`amend`] and
/// > [`reword`] at the same time.
/// >
/// > This precondition exists as a reminder in case [`RicerCli`] is somehow
/// > altered to allow this precondition to be violated by the user. In which
/// > case runtime checks __will__ need to be added for this.
///
/// [`RicerCli`]: crate::cli::RicerCli
/// [`amend`]: #member.amend
/// [`reword`]: #member.reword
#[derive(Debug)]
pub struct CommitContext {
    /// Amend changes of current commit. Any changes to index will be added to
    /// the commit.
    pub amend: bool,

    /// Reword current commit. Automatically opens user's text editor to edit
    /// the commit. No changes to index will be added to the commit.
    pub reword: bool,

    /// Use a string as commit rather than opening up the user's text editor.
    pub message: Option<String>,

    /// Action to take when executing a hook specific to this command.
    pub run_cmd_hook: RunHookOpts,

    /// Action to take when executing a repository hook.
    pub run_repo_hook: RunHookOpts,
}

impl From<RicerCli> for CommitContext {
    fn from(opts: RicerCli) -> Self {
        let RicerCli { cmd_opts, cmd_set, .. } = opts;

        let cmd_set = match cmd_set {
            CommandSet::Commit(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'commit'!"),
        };

        let (amend, reword) = match cmd_set.fixup {
            Some(FixupOpts::Amend) => (true, false),
            Some(FixupOpts::Reword) => (false, true),
            None => (false, false),
        };

        // Literally impossible, but paranoia is a hell of a drug...
        debug_assert!(
            !(amend == true && reword == true),
            "Members 'amend' and 'reword' cannot both be true"
        );

        Self {
            amend,
            reword,
            message: cmd_set.message,
            run_cmd_hook: cmd_opts.run_cmd_hook,
            run_repo_hook: cmd_opts.run_repo_hook,
        }
    }
}
