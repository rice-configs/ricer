// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use toml_edit::DocumentMut;

use crate::config::file::repos_section::*;

#[fixture]
fn toml_doc_fixture() -> DocumentMut {
    let toml = indoc! {r#"
        [repos.full_entry]
        branch = "master"
        remote = "origin"
        url = "https://github.com/awkless/foobar.git"
        target = { home = true, os = "unix", user = "awkless", hostname = "lovelace" }

        [repos.no_target_entry]
        branch = "master"
        remote = "origin"
        url = "https://github.com/awkless/foobar.git"
        "#
    };

    let toml_doc: DocumentMut = toml.parse().expect("Failed to parse toml data");
    toml_doc
}

#[rstest]
fn deserialize_full_repo_entry(toml_doc_fixture: DocumentMut) {
    let repos_table = toml_doc_fixture.get("repos").expect("The 'repos' section does not exist");
    let repos_table = repos_table.as_table().expect("The 'repos' section is not a table");
    let repo_entry =
        repos_table.get_key_value("full_entry").expect("Full entry fixture does not exist");

    let result = RepoEntry::from(repo_entry);
    let target = RepoTargetEntry::builder()
        .home(true)
        .os(TargetOsOption::Unix)
        .user("awkless")
        .hostname("lovelace")
        .build();
    let expect = RepoEntry::builder("full_entry")
        .branch("master")
        .remote("origin")
        .url("https://github.com/awkless/foobar.git")
        .target(target)
        .build();
    assert_eq!(expect, result);
}

#[rstest]
fn deserialize_repo_entry_with_missing_target_entry(toml_doc_fixture: DocumentMut) {
    let repos_table = toml_doc_fixture.get("repos").expect("The 'repos' section does not exist");
    let repos_table = repos_table.as_table().expect("The 'repos' section is not a table");
    let repo_entry =
        repos_table.get_key_value("no_target_entry").expect("No target fixture does not exist");

    let result = RepoEntry::from(repo_entry);
    let target = RepoTargetEntry::default();
    let expect = RepoEntry::builder("no_target_entry")
        .branch("master")
        .remote("origin")
        .url("https://github.com/awkless/foobar.git")
        .target(target)
        .build();
    assert_eq!(expect, result);
}

#[rstest]
fn serialize_repo_entry_correctly(mut toml_doc_fixture: DocumentMut) {
    let target_entry = RepoTargetEntry::builder()
        .home(true)
        .os(TargetOsOption::Windows)
        .build();
    let repo_entry = RepoEntry::builder("test")
        .branch("master")
        .remote("upstream")
        .url("https://github.com/awkless/foobar.git")
        .target(target_entry)
        .build();
    let (key, value) = repo_entry.to_toml();
    let repos_table = toml_doc_fixture.get_mut("repos").expect("The 'repos' table does not exist");
    let repos_table = repos_table.as_table_mut().expect("Cannot convert 'repos' to table");
    repos_table.insert(&key, value);

    let result = toml_doc_fixture.to_string();
    let expect = indoc! {r#"
        [repos.full_entry]
        branch = "master"
        remote = "origin"
        url = "https://github.com/awkless/foobar.git"
        target = { home = true, os = "unix", user = "awkless", hostname = "lovelace" }

        [repos.no_target_entry]
        branch = "master"
        remote = "origin"
        url = "https://github.com/awkless/foobar.git"

        [repos.test]
        branch = "master"
        remote = "upstream"
        url = "https://github.com/awkless/foobar.git"
        target = { home = true, os = "windows" }
        "#
    };
    assert_eq!(expect, result);
}
