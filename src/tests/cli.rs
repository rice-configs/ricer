// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::cli::*;
use clap::CommandFactory;

#[test]
fn verify_cli() {
    RicerCli::command().debug_assert();
}
