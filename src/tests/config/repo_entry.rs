// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use toml_edit::{DocumentMut, Item, Key};

use crate::config::{OsType, RepoBootstrapEntry, RepoEntry};

fn setup_toml_doc(entry: (Key, Item)) -> Result<DocumentMut> {
    let mut doc: DocumentMut = "[repos]".parse()?;
    let table = doc.get_mut("repos").unwrap();
    let table = table.as_table_mut().unwrap();
    let (key, item) = entry;
    table.insert_formatted(&key, item);
    table.set_implicit(true);
    Ok(doc)
}

#[test]
fn to_toml_serializes_correctly() -> Result<()> {
    let bootstrap = RepoBootstrapEntry::builder()
        .clone("https://github.com/awkless/vim.git")
        .os(OsType::Unix)
        .users(["awkless", "sedgwick"])
        .hosts(["lovelace", "turing"])
        .build();
    let repo = RepoEntry::builder("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true)
        .bootstrap(bootstrap)
        .build();
    let entry = repo.to_toml();
    let doc = setup_toml_doc(entry)?;
    let expect = indoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true

        [repos.vim.bootstrap]
        clone = "https://github.com/awkless/vim.git"
        os = "unix"
        users = ["awkless", "sedgwick"]
        hosts = ["lovelace", "turing"]
    "#};
    let result = doc.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn to_toml_serializes_without_bootstrap_entry() -> Result<()> {
    let repo = RepoEntry::builder("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true)
        .build();
    let entry = repo.to_toml();
    let doc = setup_toml_doc(entry)?;
    let expect = indoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true
    "#};
    let result = doc.to_string();
    assert_eq!(expect, result);
    Ok(())
}

#[test]
fn from_deserializes_correctly() -> Result<()> {
    let bootstrap = RepoBootstrapEntry::builder()
        .clone("https://github.com/awkless/vim.git")
        .os(OsType::Unix)
        .users(["awkless", "sedgwick"])
        .hosts(["lovelace", "turing"])
        .build();
    let expect = RepoEntry::builder("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true)
        .bootstrap(bootstrap)
        .build();
    let entry = expect.to_toml();
    let doc = setup_toml_doc(entry)?;
    let result = RepoEntry::from(
        doc.get("repos").unwrap().as_table().unwrap().get_key_value("vim").unwrap(),
    );
    assert_eq!(expect, result);
    Ok(())
}
