// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum LocatorError {
    #[error("Cannot determine path to home directory")]
    NoWayHome,
}
