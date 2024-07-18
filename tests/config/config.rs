// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use ricer_core::config::Config;
use ricer_core::error::RicerError;

use crate::tools::fakes::FakeConfigDir;

#[fixture]
fn ignore_file_fixture() -> FakeConfigDir {
    let config_dir = FakeConfigDir::builder().ignore_file("fake_repo", "/*").build();
    config_dir
}

#[rstest]
fn try_to_find_ignore_gives_correct_path(ignore_file_fixture: FakeConfigDir) {
    let expect = ignore_file_fixture.find_ignore("fake_repo").as_path().to_path_buf();
    let result = match Config::new(ignore_file_fixture).try_to_find_ignore("fake_repo") {
        Ok(path) => path,
        Err(error) => panic!("Expect `try_to_find_ignore` to succeed, but got: {}", error),
    };

    assert_eq!(result, expect);
}

#[rstest]
fn try_to_find_ignore_no_ignore_found(ignore_file_fixture: FakeConfigDir) {
    let result = match Config::new(ignore_file_fixture).try_to_find_ignore("nonexistant") {
        Ok(path) => panic!("Expect `try_to_find_ignore` to fail, but got: {}", path.display()),
        Err(error) => error,
    };

    assert!(matches!(result, RicerError::ConfigError(..)));
}
