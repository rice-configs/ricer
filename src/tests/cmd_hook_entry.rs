// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use toml_edit::DocumentMut;

use crate::config::file::hooks_section::*;

#[fixture]
fn toml_doc_fixture() -> DocumentMut {
    let toml = indoc! {r#"
        [hooks]
        commit = [
            { pre = "hook.sh", post = "hook.sh", repo = "vim" },
            { pre = "hook.sh", post = "hook.sh" },
            { post = "hook.sh" }
        ]
        "#
    };

    let toml_doc: DocumentMut = toml.parse().expect("Failed to parse toml data");
    toml_doc
}

#[rstest]
fn deserialize_hook_entry_correctly(toml_doc_fixture: DocumentMut) {
    let hooks_table = toml_doc_fixture.get("hooks").expect("Section 'hooks' does not exist");
    let hooks_table = hooks_table.as_table().expect("Cannot convert 'hooks' into table");
    let hook_entry =
        hooks_table.get_key_value("commit").expect("Commit command hook does not exist");

    let result = CommandHookEntry::from(hook_entry);
    let mut expect = CommandHookEntry::new("commit");
    expect.add_hook(HookEntry::builder().pre("hook.sh").post("hook.sh").repo("vim").build());
    expect.add_hook(HookEntry::builder().pre("hook.sh").post("hook.sh").build());
    expect.add_hook(HookEntry::builder().post("hook.sh").build());
    assert_eq!(expect, result);
}
