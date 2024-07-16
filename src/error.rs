// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

#[derive(Debug, thiserror::Error)]
pub enum RicerError {
    #[error("Failed to interpret configuration data")]
    ConfigError(#[from] anyhow::Error),
}
