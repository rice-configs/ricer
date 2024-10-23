// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::Cli;
use crate::context::*;

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
