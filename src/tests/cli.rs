// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::cli::*;
use clap::CommandFactory;

#[test]
fn verify_cli() {
    RicerCli::command().debug_assert();
}
