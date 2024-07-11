// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Command context state layer.
//!
//! A simple layer of abstraction that flattens the [`RicerCli`] structure into
//! individual structures that house the required context state for a given
//! Ricer command. The main overall goal is to decouple Ricer's CLI from the
//! command set allowing for free modifications to Ricer's CLI without the need
//! to modify the existing internal interface for the command set.

pub mod commit;
pub mod push;
pub mod pull;

use crate::cli::{CommandSet, RicerCli};
use commit::CommitContext;
use push::PushContext;
use pull::PullContext;

/// Context states for each Ricer command.
#[derive(Debug)]
pub enum Context {
    Commit(CommitContext),
    Push(PushContext),
    Pull(PullContext),
}

impl From<RicerCli> for Context {
    fn from(opts: RicerCli) -> Self {
        let ctx = match opts.cmd_set {
            CommandSet::Commit(_) => Self::Commit(CommitContext::from(opts)),
            CommandSet::Push(_) => Self::Push(PushContext::from(opts)),
            CommandSet::Pull(_) => Self::Pull(PullContext::from(opts)),
            _ => todo!(),
        };

        ctx
    }
}
