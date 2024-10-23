// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Failed to parse CLI arguments")]
    BadParse { source: clap::Error },
}
