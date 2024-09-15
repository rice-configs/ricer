// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use toml_edit::{DocumentMut, Item, Key};

use crate::config::{CmdHookEntry, HookEntry};

fn setup_toml_doc(entry: (Key, Item)) -> Result<DocumentMut> {
    let mut doc: DocumentMut = "[hooks]".parse()?;
    let table = doc.get_mut("hooks").unwrap();
    let table = table.as_table_mut().unwrap();
    let (key, item) = entry;
    table.insert_formatted(&key, item);
    table.set_implicit(true);
    Ok(doc)
}

#[test]
fn to_toml_serializes_correctly() -> Result<()> {
    let mut cmd_hook = CmdHookEntry::new("commit");
    cmd_hook.add_hook(
        HookEntry::builder().pre("hook.sh").post("hook.sh").workdir("/some/path").build(),
    );
    cmd_hook.add_hook(HookEntry::builder().pre("hook.sh").build());
    cmd_hook.add_hook(HookEntry::builder().post("hook.sh").build());
    let entry = cmd_hook.to_toml();
    let doc = setup_toml_doc(entry)?;
    let expect = indoc! {r#"
        [hooks]
        commit = [
            { pre = "hook.sh", post = "hook.sh", workdir = "/some/path" },
            { pre = "hook.sh" },
            { post = "hook.sh" }
        ]
    "#};
    let result = doc.to_string();
    assert_eq!(expect, result);
    Ok(())
}
