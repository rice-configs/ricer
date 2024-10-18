// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use clap::ValueEnum;
use std::ffi::OsString;

use crate::ui::{CommandSet, Cli, SharedOptions};

#[derive(Debug)]
pub enum Context {
    Bootstrap(BootstrapContext),
    Clone(CloneContext),
    Delete(DeleteContext),
    Enter(EnterContext),
    Init(InitContext),
    List(ListContext),
    Push(PushContext),
    Pull(PullContext),
    Rename(RenameContext),
    Status(StatusContext),
    Git(GitContext),
}

impl From<Cli> for Context {
    fn from(opts: Cli) -> Self {
        match opts.cmd_set {
            CommandSet::Bootstrap(_) => Self::Bootstrap(BootstrapContext::from(opts)),
            CommandSet::Clone(_) => Self::Clone(CloneContext::from(opts)),
            CommandSet::Delete(_) => Self::Delete(DeleteContext::from(opts)),
            CommandSet::Enter(_) => Self::Enter(EnterContext::from(opts)),
            CommandSet::Init(_) => Self::Init(InitContext::from(opts)),
            CommandSet::List(_) => Self::List(ListContext::from(opts)),
            CommandSet::Push(_) => Self::Push(PushContext::from(opts)),
            CommandSet::Pull(_) => Self::Pull(PullContext::from(opts)),
            CommandSet::Rename(_) => Self::Rename(RenameContext::from(opts)),
            CommandSet::Status(_) => Self::Status(StatusContext::from(opts)),
            CommandSet::Git(_) => Self::Git(GitContext::from(opts)),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct BootstrapContext {
    pub config: Option<String>,
    pub from: Option<String>,
    pub only: Option<Vec<String>>,
    pub shared: SharedContext,
}

impl From<Cli> for BootstrapContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Bootstrap(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'bootstrap'!"),
        };

        Self {
            config: cmd_set.config,
            from: cmd_set.from,
            only: cmd_set.only,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct CloneContext {
    pub remote: String,
    pub repo: Option<String>,
    pub shared: SharedContext,
}

impl From<Cli> for CloneContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Clone(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'clone'!"),
        };

        Self {
            remote: cmd_set.remote,
            repo: cmd_set.repo,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct DeleteContext {
    pub repo: String,
    pub shared: SharedContext,
}

impl From<Cli> for DeleteContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Delete(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'delete'!"),
        };

        Self {
            repo: cmd_set.repo,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct EnterContext {
    pub repo: String,
    pub shared: SharedContext,
}

impl From<Cli> for EnterContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Enter(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'enter'!"),
        };

        Self {
            repo: cmd_set.repo,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct InitContext {
    pub name: String,
    pub workdir_home: bool,
    pub branch: Option<String>,
    pub remote: Option<String>,
    pub shared: SharedContext,
}

impl From<Cli> for InitContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Init(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'init'!"),
        };

        Self {
            name: cmd_set.name,
            workdir_home: cmd_set.workdir_home,
            branch: cmd_set.branch,
            remote: cmd_set.remote,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct ListContext {
    pub tracked: bool,
    pub untracked: bool,
    pub shared: SharedContext,
}

impl From<Cli> for ListContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::List(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'list'!"),
        };

        Self {
            tracked: cmd_set.tracked,
            untracked: cmd_set.untracked,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct PushContext {
    pub remote: Option<String>,
    pub branch: Option<String>,
    pub shared: SharedContext,
}

impl From<Cli> for PushContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Push(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'push'!"),
        };

        Self {
            remote: cmd_set.remote,
            branch: cmd_set.branch,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct PullContext {
    pub branch: Option<String>,
    pub remote: Option<String>,
    pub shared: SharedContext,
}

impl From<Cli> for PullContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Pull(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'pull'!"),
        };

        Self {
            remote: cmd_set.remote,
            branch: cmd_set.branch,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct RenameContext {
    pub from: String,
    pub to: String,
    pub shared: SharedContext,
}

impl From<Cli> for RenameContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Rename(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'rename'!"),
        };

        Self {
            from: cmd_set.from,
            to: cmd_set.to,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct StatusContext {
    pub terse: bool,
    pub shared: SharedContext,
}

impl From<Cli> for StatusContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Status(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'status'!"),
        };

        Self {
            terse: cmd_set.terse,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct GitContext {
    pub repo: OsString,
    pub git_args: Vec<OsString>,
}

impl From<Cli> for GitContext {
    fn from(opts: Cli) -> Self {
        let Cli { cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Git(opts) => opts,
            _ => unreachable!("This should not happen. The command is not git shortcut!"),
        };

        Self { repo: cmd_set[0].clone(), git_args: cmd_set[1..].to_vec() }
    }
}

#[derive(Debug)]
pub struct SharedContext {
    pub run_hook: HookAction,
}

impl From<SharedOptions> for SharedContext {
    fn from(opts: SharedOptions) -> Self {
        Self { run_hook: opts.run_hook }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum HookAction {
    Always,

    #[default]
    Prompt,

    Never,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum FixupAction {
    #[default]
    Amend,

    Reword,
}
