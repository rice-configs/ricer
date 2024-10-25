// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::{Bootstrap, OsType, Repository};

use anyhow::Result;
use indoc::indoc;
use pretty_assertions::assert_eq;
use rstest::rstest;
use toml_edit::{DocumentMut, Item, Key};

fn setup_toml_doc(entry: (Key, Item)) -> Result<DocumentMut> {
    let mut doc: DocumentMut = "[repos]".parse()?;
    let table = doc.get_mut("repos").unwrap();
    let table = table.as_table_mut().unwrap();
    let (key, item) = entry;
    table.insert_formatted(&key, item);
    table.set_implicit(true);
    Ok(doc)
}

#[rstest]
#[case::no_bootstrap(
    Repository::new("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true),
    indoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true
    "#}
)]
#[case::with_bootstrap(
    Repository::new("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true)
        .bootstrap(
            Bootstrap::new()
                .clone("https://github.com/awkless/vim.git")
                .os(OsType::Unix)
                .users(["awkless", "sedgwick"])
                .hosts(["lovelace", "turing"])
        ),
    indoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true

        [repos.vim.bootstrap]
        clone = "https://github.com/awkless/vim.git"
        os = "unix"
        users = ["awkless", "sedgwick"]
        hosts = ["lovelace", "turing"]
    "#}
)]
fn to_toml_serializes(#[case] repo: Repository, #[case] expect: &str) -> Result<()> {
    let doc = setup_toml_doc(repo.to_toml())?;
    assert_eq!(doc.to_string(), expect);
    Ok(())
}

#[rstest]
#[case::no_bootstrap(
    Repository::new("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true),
)]
#[case::with_bootstrap(
    Repository::new("vim")
        .branch("master")
        .remote("origin")
        .workdir_home(true)
        .bootstrap(
            Bootstrap::new()
                .clone("https://github.com/awkless/vim.git")
                .os(OsType::Unix)
                .users(["awkless", "sedgwick"])
                .hosts(["lovelace", "turing"])
        ),
)]
fn from_entry_deserializes(#[case] expect: Repository) -> Result<()> {
    let doc = setup_toml_doc(expect.to_toml())?;
    let result = Repository::from(doc["repos"].as_table().unwrap().get_key_value("vim").unwrap());
    assert_eq!(result, expect);
    Ok(())
}
