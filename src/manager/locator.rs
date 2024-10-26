// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

#[cfg(test)]
use mockall::automock;

/// Handle different configuration directory layouts.
///
/// At a high level of abstraction, Ricer mainly splits its configuration
/// directories into two categories: behavior data, and repository data.
/// The behavior data category houses all files that configure Ricer's
/// behavior, while the repository data category contains repositories that
/// Ricer needs to keep track of and manipulate.
#[cfg_attr(test, automock)]
pub trait DirLayout {
    /// Absolute path to directory where configuration files will be stored.
    fn behavior_dir(&self) -> &Path;

    /// Absolute path to directory where repository data will be stored.
    fn repo_dir(&self) -> &Path;
}
