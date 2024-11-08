// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::{
    cli::Cli,
    context::Context,
    hook::{CmdHook, HookKind},
    locate::MockLocator,
    tests::FakeConfigDir,
};

use anyhow::Result;
use indoc::indoc;
use rstest::{fixture, rstest};

#[fixture]
fn cmd_hook_config() -> Result<FakeConfigDir> {
    let fake = FakeConfigDir::builder()?
        .config_file(
            "hooks.toml",
            indoc! {r#"
                [hooks]
                bootstrap = [
                    { pre = "hook.sh" },
                    { post = "hook.sh" },
                ]
            "#},
        )?
        .hook_script(
            "hook.sh",
            indoc! {r#"
                #!/bin/sh

                echo "hello world"
                exit 0
            "#},
        )?
        .build();
    Ok(fake)
}

#[rstest]
fn cmd_hook_works(cmd_hook_config: Result<FakeConfigDir>) -> Result<()> {
    let config = cmd_hook_config?;
    let mut locator = MockLocator::new();
    locator.expect_hooks_config().return_const(config.config_dir().join("hooks.toml"));
    locator.expect_hooks_dir().return_const(config.hook_dir().to_path_buf());

    let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "bootstrap"])?);
    let hook_mgr = CmdHook::load(&ctx, &locator)?;
    assert!(hook_mgr.run_hooks(HookKind::Pre).is_ok());
    assert!(hook_mgr.run_hooks(HookKind::Post).is_ok());
    Ok(())
}

#[rstest]
fn cmd_hook_ignores_git_shortcut(cmd_hook_config: Result<FakeConfigDir>) -> Result<()> {
    let config = cmd_hook_config?;
    let mut locator = MockLocator::new();
    locator.expect_hooks_config().return_const(config.config_dir().join("hooks.toml"));
    locator.expect_hooks_dir().return_const(config.hook_dir().to_path_buf());

    let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "vim", "commit"])?);
    let hook_mgr = CmdHook::load(&ctx, &locator)?;
    assert!(hook_mgr.run_hooks(HookKind::Pre).is_ok());
    assert!(hook_mgr.run_hooks(HookKind::Post).is_ok());
    Ok(())
}
