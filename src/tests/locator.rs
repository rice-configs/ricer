// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use std::path::PathBuf;
use tempfile::{Builder, TempDir};

use ricer_test_tools::fakes::FakeConfigDir;

use crate::config::locator::*;
use crate::error::RicerError;

#[fixture]
fn config_dir_fixture() -> FakeConfigDir {
    FakeConfigDir::builder().build()
}

#[fixture]
fn home_dir_fixture() -> TempDir {
    Builder::new().tempdir().expect("Failed to create fake home directory")
}

#[rstest]
fn new_locate_gives_correct_path(config_dir_fixture: FakeConfigDir) {
    let mut mock_xdg_spec = MockXdgBaseDirSpec::new();
    mock_xdg_spec
        .expect_config_home_dir()
        .return_const(config_dir_fixture.temp_dir().to_path_buf());

    let locator = DefaultConfigDirLocator::new_locate(&mock_xdg_spec).expect("Expect success");
    let expect = config_dir_fixture.root_dir();
    let result = locator.config_dir();
    assert_eq!(expect, result);
}

#[test]
fn new_locate_catches_nonexistant_config_dir() {
    let mut mock_xdg_spec = MockXdgBaseDirSpec::new();
    mock_xdg_spec.expect_config_home_dir().return_const(PathBuf::from("nonexistant"));

    let result = DefaultConfigDirLocator::new_locate(&mock_xdg_spec);
    assert!(matches!(result, Err(RicerError::NoConfigDir(..))));
}

#[rstest]
fn recover_default_config_dir_locator_makes_config_dir(home_dir_fixture: TempDir) {
    let mut mock_xdg_spec = MockXdgBaseDirSpec::new();
    mock_xdg_spec.expect_config_home_dir().return_const(home_dir_fixture.path().to_path_buf());

    let expect = home_dir_fixture.path().join("ricer");
    let result = recover_default_config_dir_locator(&mock_xdg_spec).expect("Expect success");
    assert_eq!(expect.as_path(), result.config_dir());
    assert_eq!(result.config_dir().exists(), true);
}

#[test]
fn recover_default_config_dir_locator_cannot_make_config_dir() {
    let mut mock_xdg_spec = MockXdgBaseDirSpec::new();
    mock_xdg_spec.expect_config_home_dir().return_const(PathBuf::from("nonexistant"));

    let result = recover_default_config_dir_locator(&mock_xdg_spec);
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
