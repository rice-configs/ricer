// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config::Toml;
use crate::error::RicerResult;

use indoc::indoc;
use anyhow::Result;
use rstest::rstest;

#[rstest]
fn toml_parse_catches_bad_formatting(
    #[values("this 'will fail'", "[will # also fail", "not.gonna = [work]")] input: &str,
) {
    let result: RicerResult<Toml> = input.parse();
    assert!(matches!(result, Err(..)));
}

#[rstest]
fn toml_parse_parses_good_formatting(
    #[values(
        indoc! {r#"
            [foo]
            this = "will parse"
            will.also = "parse"
        "#}
    )] input: &str,
) -> Result<()> {
    let doc: Toml = input.parse()?;
    assert_eq!(doc.to_string(), input);
    Ok(())
}
