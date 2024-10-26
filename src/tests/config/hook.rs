// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::{CommandHook, Hook};

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use toml_edit::{DocumentMut, Item, Key};
use rstest::rstest;

fn setup_toml_doc(entry: (Key, Item)) -> Result<DocumentMut> {
    let mut doc: DocumentMut = "[hooks]".parse()?;
    let table = doc.get_mut("hooks").unwrap();
    let table = table.as_table_mut().unwrap();
    let (key, item) = entry;
    table.insert_formatted(&key, item);
    table.set_implicit(true);
    Ok(doc)
}

#[rstest]
#[case(
    CommandHook::new("commit")
        .add_hook(Hook::new().pre("hook.sh").post("hook.sh").workdir("/some/path"))
        .add_hook(Hook::new().pre("hook.sh"))
        .add_hook(Hook::new().post("hook.sh")),
    indoc! {r#"
        [hooks]
        commit = [
            { pre = "hook.sh", post = "hook.sh", workdir = "/some/path" },
            { pre = "hook.sh" },
            { post = "hook.sh" }
        ]
    "#}
)]
fn to_toml_serializes(#[case] cmd_hook: CommandHook, #[case] expect: &str) -> Result<()> {
    let doc = setup_toml_doc(cmd_hook.to_toml())?;
    assert_eq!(doc.to_string(), expect);
    Ok(())
}

#[rstest]
#[case(
    CommandHook::new("commit")
        .add_hook(Hook::new().pre("hook.sh").post("hook.sh").workdir("/some/path"))
        .add_hook(Hook::new().pre("hook.sh"))
        .add_hook(Hook::new().post("hook.sh"))
)]
fn from_deserializes(#[case] expect: CommandHook) -> Result<()> {
    let doc = setup_toml_doc(expect.to_toml())?;
    let result = CommandHook::from(doc["hooks"].as_table().unwrap().get_key_value("commit").unwrap());
    assert_eq!(result, expect);
    Ok(())
}
