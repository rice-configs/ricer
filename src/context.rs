// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Command context state layer.
//!
//! A simple layer of abstraction that flattens the [`RicerCli`] structure into
//! individual structures that house the required context state for a given
//! Ricer command. The main overall goal is to decouple Ricer's CLI from the
//! command set allowing for free modifications to Ricer's CLI without the need
//! to modify the existing internal interface for the command set.

pub mod clone;
pub mod commit;
pub mod delete;
pub mod enter;
pub mod git_on_repo;
pub mod init;
pub mod list;
pub mod pull;
pub mod push;
pub mod rename;
pub mod status;

use crate::cli::{CommandSet, RicerCli};
use clone::CloneContext;
use commit::CommitContext;
use delete::DeleteContext;
use enter::EnterContext;
use git_on_repo::UseGitBinOnRepoContext;
use init::InitContext;
use list::ListContext;
use pull::PullContext;
use push::PushContext;
use rename::RenameContext;
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
    UseGitBinOnRepo(UseGitBinOnRepoContext),
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
            CommandSet::UseGitBinOnRepo(_) => {
                Self::UseGitBinOnRepo(UseGitBinOnRepoContext::from(opts))
            }
        }
    }
}
