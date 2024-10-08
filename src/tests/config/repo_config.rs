// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use anyhow::Result;
use indoc::indoc;
use rstest::{fixture, rstest};

use crate::config::{Config, Repo, RepoConfig, Toml};

#[fixture]
fn repo_toml_vim() -> String {
    String::from(indoc! {r#"
        [repos.vim]
        branch = "master"
        remote = "origin"
        workdir_home = true
    "#})
}

#[fixture]
fn repo_de_vim() -> Repo {
    Repo::builder("vim").branch("master").remote("origin").workdir_home(true).build()
}

#[rstest]
#[case(repo_toml_vim(), repo_de_vim())]
fn get_deserialize_no_error(#[case] input: String, #[case] expect: Repo) -> Result<()> {
    let doc: Toml = input.parse()?;
    let result = RepoConfig.get(&doc, "vim")?;
    assert_eq!(expect, result);
    Ok(())
}

#[rstest]
fn get_config_error(
    #[values("[no_repos]", "repos = 'not a table'", "[repos]")] input: &str,
) -> Result<()> {
    let doc: Toml = input.parse()?;
    let result = RepoConfig.get(&doc, "inexistent");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
#[case(repo_toml_vim(), repo_de_vim(), "")]
fn remove_no_error(#[case] input: String, #[case] repo_expect: Repo, #[case] toml_expect: &str) -> Result<()> {
    let mut doc: Toml = input.parse()?;
    let result = RepoConfig.remove(&mut doc, "vim")?;
    assert_eq!(repo_expect, result);
    assert_eq!(toml_expect, doc.to_string());
    Ok(())
}

#[rstest]
fn remove_config_error(
    #[values("[no_repos]", "repos = 'not a table'", "[repos]")] input: &str,
) -> Result<()> {
    let mut doc: Toml = input.parse()?;
    let result = RepoConfig.remove(&mut doc, "inexistent");
    assert!(matches!(result, Err(..)));
    Ok(())
}

#[rstest]
#[case(repo_toml_vim(), repo_de_vim(), repo_toml_vim().replace("vim", "neovim"))]
fn rename_no_error(#[case] input: String, #[case] repo_expect: Repo, #[case] toml_expect: String) -> Result<()> {
    let mut doc: Toml = input.parse()?;
    let result = RepoConfig.rename(&mut doc, "vim", "neovim")?;
    assert_eq!(repo_expect, result);
    assert_eq!(toml_expect, doc.to_string());
    Ok(())
}

#[rstest]
fn rename_config_error(
    #[values("[no_repos]", "repos = 'not a table'", "[repos]")] input: &str,
) -> Result<()> {
    let mut doc: Toml = input.parse()?;
    let result = RepoConfig.rename(&mut doc, "inexistent", "fail");
    assert!(matches!(result, Err(..)));
    Ok(())
}

