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

use log::trace;

/// Repository entry definition implementation.
///
/// # Invariants
///
/// 1. No field is empty.
#[derive(Debug, Default, Eq, PartialEq)]
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

impl RepoEntry {
    /// Build new repository entry definition.
    ///
    /// # Postconditions
    ///
    /// 1. Return [`RepoEntryBuilder`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::file::repos_section::RepoEntry;
    ///
    /// let repo_builder = RepoEntry::builder("vim:");
    /// ```
    pub fn builder(name: impl AsRef<str>) -> RepoEntryBuilder {
        RepoEntryBuilder::new(name)
    }
}

/// Repository entry builder.
///
/// Generally exists to make repository entry definitions much easier in the
/// future!
///
/// # Invariants
///
/// 1. No field is empty.
#[derive(Debug, Default)]
pub struct RepoEntryBuilder {
    name: String,
    branch: String,
    remote: String,
    url: String,
    target: RepoTargetEntry,
}

impl RepoEntryBuilder {
    /// Construct repository entry builder.
    ///
    /// # Postconditions
    ///
    /// 1. Return new instance of repository entry builder.
    ///
    /// ```no_run
    /// use ricer::config::file::repos_section::RepoEntryBuilder;
    ///
    /// let repo_builder = RepoEntryBuilder::new("vim");
    /// ```
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            branch: Default::default(),
            remote: Default::default(),
            url: Default::default(),
            target: Default::default(),
        }
    }

    pub fn branch(mut self, branch: impl AsRef<str>) -> Self {
        self.branch = branch.as_ref().to_string();
        self
    }

    pub fn remote(mut self, remote: impl AsRef<str>) -> Self {
        self.remote = remote.as_ref().to_string();
        self
    }

    pub fn url(mut self, url: impl AsRef<str>) -> Self {
        self.url = url.as_ref().to_string();
        self
    }

    pub fn target(mut self, target: RepoTargetEntry) -> Self {
        self.target = target;
        self
    }

    /// Build new [`RepoEntry`].
    ///
    /// # Postconditions
    ///
    /// 1. Valid instance of [`RepoEntry`].
    ///
    /// # Invariants
    ///
    /// 1. No field is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::file::repos_section::RepoEntryBuilder;
    ///
    /// let target = RepoTargetEntryBuilder::new().build();
    /// let repo_builder = RepoEntryBuilder::new("vim");
    ///     .branch("master")
    ///     .remote("origin")
    ///     .url("https://github.com/awkless/vim.git")
    ///     .target(target)
    ///     .build();
    /// ```
    pub fn build(self) -> RepoEntry {
        trace!("Build new repository entry definition");
        debug_assert_ne!(self.name, String::default(), "Name of repository entry is empty");
        debug_assert_ne!(self.branch, String::default(), "Branch of repository entry is empty");
        debug_assert_ne!(self.remote, String::default(), "Remote of repository entry is empty");
        debug_assert_ne!(self.url, String::default(), "URL of repository entry is empty");
        debug_assert_ne!(self.target, RepoTargetEntry::default(), "Target of repository is empty");

        RepoEntry {
            name: self.name,
            branch: self.branch,
            remote: self.remote,
            url: self.url,
            target: self.target,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
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
