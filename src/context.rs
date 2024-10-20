// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Command context layer.
//!
//! A __context__ in Ricer is a definitive and flattened set of options required
//! for a Ricer command to function.
//!
//! The various CLI options provide an interface for the user, which
//! produces a tree-like structure that can be annoying to use internally. Thus,
//! the context layer converts CLI options into a flattened structure with
//! at most one to two levels of indirection.
//!
//! Command context also provides a layer of abstraction between the CLI and
//! command set implementations. So, changes to the CLI will not directly effect
//! any implementations of the command set in the codebase.

use clap::ValueEnum;
use std::ffi::OsString;

use crate::ui::{Cli, CommandSet, SharedOptions};

#[derive(Debug, Eq, PartialEq)]
pub enum Context {
    Bootstrap(BootstrapContext),
    Clone(CloneContext),
    Commit(CommitContext),
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
            CommandSet::Commit(_) => Self::Commit(CommitContext::from(opts)),
            CommandSet::Delete(_) => Self::Delete(DeleteContext::from(opts)),
            CommandSet::Enter(_) => Self::Enter(EnterContext::from(opts)),
            CommandSet::Init(_) => Self::Init(InitContext::from(opts)),
            CommandSet::List(_) => Self::List(ListContext::from(opts)),
            CommandSet::Push(_) => Self::Push(PushContext::from(opts)),
            CommandSet::Pull(_) => Self::Pull(PullContext::from(opts)),
            CommandSet::Rename(_) => Self::Rename(RenameContext::from(opts)),
            CommandSet::Status(_) => Self::Status(StatusContext::from(opts)),
            CommandSet::Git(_) => Self::Git(GitContext::from(opts)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

        Self { remote: cmd_set.remote, repo: cmd_set.repo, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CommitContext {
    pub fixup: Option<FixupAction>,
    pub message: Option<String>,
    pub shared: SharedContext,
}

impl From<Cli> for CommitContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Commit(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'commit'!"),
        };

        Self { fixup: cmd_set.fixup, message: cmd_set.message, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

        Self { repo: cmd_set.repo, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

        Self { repo: cmd_set.repo, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

        Self { tracked: cmd_set.tracked, untracked: cmd_set.untracked, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

        Self { remote: cmd_set.remote, branch: cmd_set.branch, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

        Self { remote: cmd_set.remote, branch: cmd_set.branch, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

        Self { from: cmd_set.from, to: cmd_set.to, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

        Self { terse: cmd_set.terse, shared: shared_opts.into() }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::ffi::OsString;

    #[rstest]
    #[case::shared_run_hook(
        ["ricer", "--run-hook", "always", "enter", "foo"],
        Context::Enter(EnterContext {
            repo: "foo".into(),
            shared: SharedContext { run_hook: HookAction::Always },
        })
    )]
    #[case::bootstrap(
        ["ricer", "bootstrap", "--config", "vim", "--only", "sh,mutt,vim", "--from", "url"],
        Context::Bootstrap(BootstrapContext {
            config: Some("vim".into()),
            from: Some("url".into()),
            only: Some(vec!["sh".into(), "mutt".into(), "vim".into()]),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::commit(["ricer", "commit", "--fixup", "amend", "--message", "hello world"],
        Context::Commit(CommitContext {
            fixup: Some(FixupAction::Amend),
            message: Some("hello world".into()),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::clone(
        ["never", "clone", "url", "foo"],
        Context::Clone(CloneContext {
            remote: "url".into(),
            repo: Some("foo".into()),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::delete(
        ["ricer", "delete", "foo"],
        Context::Delete( DeleteContext {
            repo: "foo".into(),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::enter(
        ["ricer", "enter", "foo"],
        Context::Enter(EnterContext {
            repo: "foo".into(),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::init(
        ["ricer", "init", "foo", "--workdir-home", "--branch", "main", "--remote", "origin"],
        Context::Init(InitContext {
            name: "foo".into(),
            workdir_home: true,
            branch: Some("main".into()),
            remote: Some("origin".into()),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::list(
        ["ricer", "list", "--tracked", "--untracked"],
        Context::List(ListContext {
            tracked: true,
            untracked: true,
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::push(
        ["ricer", "push", "origin", "main"],
        Context::Push(PushContext {
            remote: Some("origin".into()),
            branch: Some("main".into()),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::pull(
        ["ricer", "pull", "origin", "main"],
        Context::Pull(PullContext {
            remote: Some("origin".into()),
            branch: Some("main".into()),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::rename(
        ["ricer", "rename", "foo", "bar"],
        Context::Rename(RenameContext {
            from: "foo".into(),
            to: "bar".into(),
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::status(
        ["ricer", "status", "--terse"],
        Context::Status(StatusContext {
            terse: true,
            shared: SharedContext { run_hook: HookAction::default() },
        })
    )]
    #[case::git_shortcut(
        ["ricer", "foo", "add", "file.txt"],
        Context::Git(GitContext {
            repo: "foo".into(),
            git_args: vec!["add".into(), "file.txt".into()]
        })
    )]
    fn valid_ctx_from_cli<I, T>(#[case] args: I, #[case] expect: Context) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let opts = Cli::parse_args(args)?;
        let result = Context::from(opts);
        assert_eq!(expect, result);
        Ok(())
    }
}
