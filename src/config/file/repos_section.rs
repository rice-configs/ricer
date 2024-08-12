// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Manage repository section definitions.
//!
//! The repository section of Ricer's configuration file houses all repository
//! definitions that Ricer needs to keep track of. This section is defined in
//! the following manner:
//!
//! ```markdown
//! [repos.repo_name]
//! branch = "master"
//! remote = "origin"
//! url = "https://github.com/awkless/vim.git"
//! target = { home = true, os = "all", user = "awkless", hostname = "lovelace" }
//! ```
//!
//! The `repo_name` field is the name of the repository entry. The `branch`
//! field is the main branch that will be used for pulls, pushes, etc. The
//! `remote` field is the main remote that will be used for pulls, pushes, etc.
//! The `url` field is the URL used to clone, and push to the remote repository.
//! Finally, the `target` field is used to configure when and where the
//! repository entry should be cloned, pulled, pushed, etc. The `target` field
//! is mainly used for bootstrapping purposes.
//!
//! In the `target` field, the `home` field determines if Ricer should make the
//! repository entry use the user's home directory as the primary working tree.
//! The `os` field makes Ricer boostrap the repository if and only if the user
//! is using a specific operating system. Current values for `os` is _unix_,
//! _macos_, _windows_, or _any_. The `user` and `hostname` fields will make
//! Ricer only bootstrap a repository for a specific user and host.

pub struct RepoEntry {
    /// Name of repository. Also used to name the cloned repository in the
    /// `repos` directory.
    pub name: String,

    /// Primary branch to use for Ricer's command set.
    pub branch: String,

    /// Primary remote to use for Ricer's command set.
    pub remote: String,

    /// Primary URL used to clone, push, and pull repository from.
    pub url: String,

    /// Bootstrapping options.
    pub target: RepoTargetEntry,
}

pub struct RepoTargetEntry {
    /// Repository will use the user's home directory as the main working tree.
    pub home: bool,

    /// Bootstrap repository if and only if user's is using a specific operating
    /// system.
    pub os: String,

    /// Bootstrap repository for a specific user only on the system.
    pub user: String,

    /// Bootstrap repository for a specific host only on the system.
    pub hostname: String,
}
