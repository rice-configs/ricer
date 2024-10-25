// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use std::fmt;

/// Repository configuration settings.
///
/// Intermediary structure meant to help make it easier to deserialize and
/// serialize repository configuration file data.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Repository {
    /// Name of repository.
    pub name: String,

    /// Default branch.
    pub branch: String,

    /// Default remote.
    pub remote: String,

    /// Flag to determine if repository's working directory is the user's home
    /// directory through _fake bare_ technique.
    pub workdir_home: bool,

    /// Bootstrap configuration for repository.
    pub bootstrap: Option<Bootstrap>,
}

/// Repository bootstrap configuration settings.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Bootstrap {
    /// URL to clone repository from.
    pub clone: Option<String>,

    /// Bootstrap repository if and only if user is using a specific OS.
    pub os: Option<OsType>,

    /// Bootstrap repository if and only if user is logged on to a specific
    /// set of user accounts.
    pub users: Option<Vec<String>>,

    /// Bootstrap repository if and only if user is logged on to a specific
    /// set of hosts.
    pub hosts: Option<Vec<String>>,
}

/// Operating System settings.
///
/// Simple enum used to determine the target OS user wants to bootstrap with.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum OsType {
    /// Bootstrap to any operating system.
    #[default]
    Any,

    /// Bootstrap to Unix-like systems only.
    Unix,

    /// Bootstrap to MacOS systems only.
    MacOs,

    /// Bootstrap to Windows system only.
    Windows,
}

impl fmt::Display for OsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsType::Any => write!(f, "any"),
            OsType::Unix => write!(f, "unix"),
            OsType::MacOs => write!(f, "macos"),
            OsType::Windows => write!(f, "windows"),
        }
    }
}
