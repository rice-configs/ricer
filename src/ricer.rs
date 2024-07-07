// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use log::{error, info};
use std::process::ExitCode;

use ricer::cli::Cli;

fn main() -> ExitCode {
    match Cli::new_run() {
        Ok(out) => {
            info!("{}", out);
            ExitCode::SUCCESS
        }

        Err(error) => {
            error!("{}", error);
            ExitCode::FAILURE
        }
    }
}
