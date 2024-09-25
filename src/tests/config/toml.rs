// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use indoc::indoc;
use pretty_assertions::assert_eq;
use anyhow::Result;

use crate::config::Toml;

#[test]
fn parse_bad_toml_error() {
    let result: Result<Toml> = r#"this "will fail""#.parse();
    assert!(matches!(result, Err(..)));
}

#[test]
fn parse_no_error() -> Result<()> {
    let expect = indoc! {r#"
        [test]
        foo = "some data"
        bar = "some data"

        [test.baaz]
        buzz = "some data"
    "#};
    let config: Toml = expect.parse()?;
    let result = config.to_string();
    assert_eq!(expect, result);
    Ok(())
}
