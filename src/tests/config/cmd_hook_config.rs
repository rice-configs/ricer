// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use crate::config::{CmdHook, CmdHookConfig, Config, Hook, Toml};

#[fixture]
fn cmd_hook_toml_commit() -> String {
    String::from(indoc! {r#"
        [hooks]
        commit = [
            { pre = "hook.sh" },
            { post = "hook.sh" },
            { pre = "hook.sh", post = "hook.sh", workdir = "/workdir/path" }
        ]
    "#})
}

#[fixture]
fn cmd_hook_de_commit() -> CmdHook {
    let mut commit = CmdHook::new("commit");
    commit.add_hook(Hook::builder().pre("hook.sh").build());
    commit.add_hook(Hook::builder().post("hook.sh").build());
    commit
        .add_hook(Hook::builder().pre("hook.sh").post("hook.sh").workdir("/workdir/path").build());
    commit
}

#[rstest]
#[case(cmd_hook_toml_commit(), cmd_hook_de_commit())]
fn get_deserialize_no_error(#[case] input: String, #[case] expect: CmdHook) -> Result<()> {
    let doc: Toml = input.parse()?;
    let result = CmdHookConfig.get(&doc, "commit")?;
    assert_eq!(expect, result);
    Ok(())
}

#[rstest]
fn get_config_error(
    #[values("[no_hooks]", "hooks = 'not a table'", "[hooks]")] input: &str,
) -> Result<()> {
    let doc: Toml = input.parse()?;
    let result = CmdHookConfig.get(&doc, "inexistent");
    assert!(matches!(result, Err(..)));
    Ok(())
}
