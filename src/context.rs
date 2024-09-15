// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

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

use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

mod clone;
mod commit;
mod delete;
mod enter;
mod init;
mod list;
mod pull;
mod push;
mod rename;
mod repo_git;
mod status;

#[doc(inline)]
pub use clone::*;
pub use commit::*;
pub use delete::*;
pub use enter::*;
pub use init::*;
pub use list::*;
pub use pull::*;
pub use push::*;
pub use rename::*;
pub use repo_git::*;
pub use status::*;

use crate::cli::{CommandSet, RicerCli, SharedOptions};

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

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Context::Commit(_) => write!(f, "commit"),
            Context::Push(_) => write!(f, "push"),
            Context::Pull(_) => write!(f, "pull"),
            Context::Init(_) => write!(f, "init"),
            Context::Clone(_) => write!(f, "clone"),
            Context::Delete(_) => write!(f, "delete"),
            Context::Rename(_) => write!(f, "rename"),
            Context::Status(_) => write!(f, "status"),
            Context::List(_) => write!(f, "list"),
            Context::Enter(_) => write!(f, "enter"),
            &_ => unreachable!("This should not happen. Cannot convert this context to string"),
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
/// Hooks pose a potential security risk to the user. The user is expected to
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
