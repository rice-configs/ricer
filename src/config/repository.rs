// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

/// Repository configuration settings.
///
/// Intermediary structure meant to help make it easier to deserialize and
/// serialize repository configuration file data.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Repository {
    pub name: String,

    pub branch: String,

    pub remote: String,

    pub workdir_home: bool,

    pub bootstrap: Option<Bootstrap>,
}
