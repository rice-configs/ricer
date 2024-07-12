// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Command context state layer.
//!
//! A _context_ in Ricer, is a definitive and flattened set of options required
//! for a Ricer command to function.
//!
//! The various `[...]Options` structures contained in [`RicerCli`] provide an
//! interface for the user. These structures provide a tree structure that can
//! be annoying to use internally for Ricer. Thus, the context layer converts
//! the various options offered by [`RicerCli`] into a flattened structure with
//! at most one level of indirection.
//!
//! Another benefit of the context layer, is the decoupling of Ricer commands
//! from the CLI. Thus, we can make modifications to the CLI freely without
//! worring about affecting any of the command set implementations.
//!
//! # Thanks
//!
//! The idea for a context layer came for the cargo-msrv project. See
//! <https://github.com/foresterre/cargo-msrv/blob/main/src/context.rs#L39C1-L56C10>.
//!
//! [`RicerCli`]: crate::cli::RicerCli

pub mod clone;
pub mod commit;
pub mod delete;
pub mod enter;
pub mod init;
pub mod list;
pub mod pull;
pub mod push;
pub mod rename;
pub mod repo_git;
pub mod status;

use crate::cli::{CommandSet, RicerCli, SharedOptions};
use clap::ValueEnum;
use clone::CloneContext;
use commit::CommitContext;
use delete::DeleteContext;
use enter::EnterContext;
use init::InitContext;
use list::ListContext;
use pull::PullContext;
use push::PushContext;
use rename::RenameContext;
use repo_git::RepoGitContext;
use status::StatusContext;

/// Context states for each Ricer command.
#[derive(Debug)]
pub enum Context {
    Commit(CommitContext),
    Push(PushContext),
    Pull(PullContext),
    Init(InitContext),
    Clone(CloneContext),
    Delete(DeleteContext),
    Rename(RenameContext),
    Status(StatusContext),
    List(ListContext),
    Enter(EnterContext),
    RepoGit(RepoGitContext),
}

impl From<RicerCli> for Context {
    fn from(opts: RicerCli) -> Self {
        match opts.cmd_set {
            CommandSet::Commit(_) => Self::Commit(CommitContext::from(opts)),
            CommandSet::Push(_) => Self::Push(PushContext::from(opts)),
            CommandSet::Pull(_) => Self::Pull(PullContext::from(opts)),
            CommandSet::Init(_) => Self::Init(InitContext::from(opts)),
            CommandSet::Clone(_) => Self::Clone(CloneContext::from(opts)),
            CommandSet::Delete(_) => Self::Delete(DeleteContext::from(opts)),
            CommandSet::Rename(_) => Self::Rename(RenameContext::from(opts)),
            CommandSet::Status(_) => Self::Status(StatusContext::from(opts)),
            CommandSet::List(_) => Self::List(ListContext::from(opts)),
            CommandSet::Enter(_) => Self::Enter(EnterContext::from(opts)),
            CommandSet::RepoGit(_) => Self::RepoGit(RepoGitContext::from(opts)),
        }
    }
}

/// Context that is shared across all command contexts.
#[derive(Debug)]
pub struct SharedContext {
    /// Hook execution action choice.
    pub hook_action: HookAction,
}

impl From<SharedOptions> for SharedContext {
    fn from(opts: SharedOptions) -> Self {
        Self { hook_action: opts.run_hook }
    }
}

/// Hook execution options.
///
/// Hooks pose as a potential security risk to a user. The user is expected to
/// know what __any__ hook is doing _before_ executing it, because Ricer does
/// not provide any way to verify if it is safe to run. However, Ricer does try
/// to help by offering options in executing a hook. The default behavior is to
/// show the user the contents of a given hook and prompt them about executing
/// it. Otherwise, they can choose never to run a hook or always execute a hook
/// with no prompting.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum HookAction {
    /// Run the hook no questions asked.
    Always,

    /// Show the user the contents of the hook and prompt them to execute it.
    Prompt,

    /// Do not execute the hook.
    Never,
}
