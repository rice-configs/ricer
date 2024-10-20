// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::RicerError;
use crate::ui::*;

use clap::CommandFactory;
use rstest::rstest;
use std::ffi::OsString;

#[test]
fn verify_cli_structure() {
    Cli::command().debug_assert();
}

#[rstest]
#[case::invalid_bootstrap_args(["ricer", "bootstrap", "--non-existent"])]
#[case::invalid_commit_args(["ricer", "commit", "--non-existent"])]
#[case::invalid_clone_args(["ricer", "clone", "--non-existent"])]
#[case::invalid_delete_args(["ricer", "delete", "foo", "--non-existent"])]
#[case::invalid_enter_args(["ricer", "enter", "foo", "--non-existent"])]
#[case::invalid_init_args(["ricer", "init", "--non-existent"])]
#[case::invalid_list_args(["ricer", "list", "--non-existent"])]
#[case::invalid_push_args(["ricer", "push", "--non-existent"])]
#[case::invalid_pull_args(["ricer", "pull", "--non-existent"])]
#[case::invalid_rename_args(["ricer", "rename", "foo", "bar", "--non-existent"])]
#[case::invalid_status_args(["ricer", "status", "--non-existent"])]
#[case::invalid_shared_opts(["ricer", "--not-shared", "bootstrap"])]
fn cli_catches_invalid_args<I, T>(#[case] args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let result = Cli::parse_args(args);
    assert!(matches!(result, Err(RicerError::Unrecoverable(..))));
}
