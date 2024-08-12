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
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return [`RepoEntryBuilder`].
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
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
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return new instance of repository entry builder.
    ///
    /// # Invariants
    ///
    /// 1. No field is empty.
    ///
    /// # Side Effects
    ///
    /// None.
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            branch: Default::default(),
            remote: Default::default(),
            url: Default::default(),
            target: Default::default(),
        }
    }

    /// Set branch field.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`branch`] field.
    ///
    /// Invariants
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`branch`]: #member.branch
    pub fn branch(mut self, branch: impl AsRef<str>) -> Self {
        self.branch = branch.as_ref().to_string();
        self
    }

    /// Set remote field.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`remote`] field.
    ///
    /// Invariants
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`remote`]: #member.remote
    pub fn remote(mut self, remote: impl AsRef<str>) -> Self {
        self.remote = remote.as_ref().to_string();
        self
    }

    /// Set URL field.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`url`] field.
    ///
    /// Invariants
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`url`]: #member.url
    pub fn url(mut self, url: impl AsRef<str>) -> Self {
        self.url = url.as_ref().to_string();
        self
    }

    /// Set target field.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`target`] field.
    ///
    /// Invariants
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`target`]: #member.target
    pub fn target(mut self, target: RepoTargetEntry) -> Self {
        self.target = target;
        self
    }

    /// Build new [`RepoEntry`].
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Valid instance of [`RepoEntry`].
    ///
    /// # Invariants
    ///
    /// 1. No field is empty.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::repos_section::{RepoEntryBuilder, RepoTargetEntry, TargetOsOption};
    ///
    /// let target = RepoTargetEntry::builder()
    ///     .home(true)
    ///     .os(TargetOsOption::Unix)
    ///     .build();
    /// let builder = RepoEntryBuilder::new("vim")
    ///     .branch("master")
    ///     .remote("origin")
    ///     .url("https://github.com/awkless/vim.git")
    ///     .target(target)
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> RepoEntry {
        trace!("Build new repository entry definition");
        debug_assert!(!self.name.is_empty(), "Name of repository entry is empty");
        debug_assert!(!self.branch.is_empty(), "Branch of repository entry is empty");
        debug_assert!(!self.remote.is_empty(), "Remote of repository entry is empty");
        debug_assert!(!self.url.is_empty(), "URL of repository entry is empty");

        RepoEntry {
            name: self.name,
            branch: self.branch,
            remote: self.remote,
            url: self.url,
            target: self.target,
        }
    }
}

/// Target bootstrap options for repository definition implementation.
///
/// # Invariants
///
/// 1. No field is empty.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct RepoTargetEntry {
    /// Repository will use the user's home directory as the main working tree.
    pub home: bool,

    /// Bootstrap repository if and only if user's is using a specific operating
    /// system.
    pub os: TargetOsOption,

    /// Bootstrap repository for a specific user only on the system.
    pub user: Option<String>,

    /// Bootstrap repository for a specific host only on the system.
    pub hostname: Option<String>,
}

impl RepoTargetEntry {
    pub fn builder() -> RepoTargetEntryBuilder {
        RepoTargetEntryBuilder::new()
    }
}

/// Builder for target bootstrap options for repository definition implementation.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct RepoTargetEntryBuilder {
    home: bool,
    os: TargetOsOption,
    user: Option<String>,
    hostname: Option<String>,
}

impl RepoTargetEntryBuilder {
    /// Construct new repository target entry builder.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return new repository target entry builder.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set home target.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`home`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`home`]: #member.home
    pub fn home(mut self, home: bool) -> Self {
        self.home = home;
        self
    }

    /// Set OS target.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`os`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`os`]: #member.os
    pub fn os(mut self, os: TargetOsOption) -> Self {
        self.os = os;
        self
    }

    /// Set user target
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`user`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`user`]: #member.user
    pub fn user(mut self, user: Option<impl AsRef<str>>) -> Self {
        self.user = user.map(|str| str.as_ref().to_string());
        self
    }

    /// Set hostname target.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`hostname`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`hostname`]: #member.hostname
    pub fn hostname(mut self, hostname: Option<impl AsRef<str>>) -> Self {
        self.hostname = hostname.map(|str| str.as_ref().to_string());
        self
    }

    /// Build new [`RepoTargetEntry`].
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return new [`RepoTargetEntry`].
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::repos_section::{RepoTargetEntryBuilder, TargetOsOption};
    ///
    /// let builder = RepoTargetEntryBuilder::new()
    ///     .home(true)
    ///     .os(TargetOsOption::Unix)
    ///     .user(Some("awkless"))
    ///     .hostname(Some("lovelace"))
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`os`]: #member.os
    pub fn build(self) -> RepoTargetEntry {
        trace!("Build new target entry for repository entry definition");

        RepoTargetEntry {
            home: self.home,
            os: self.os,
            user: self.user,
            hostname: self.hostname,
        }
    }
}

/// Target OS option types.
#[derive(Debug, Default, Eq, PartialEq)]
pub enum TargetOsOption {
    /// Target any operating system.
    #[default]
    Any,

    /// Only target Unix/Linux operating systems.
    Unix,

    /// Only target MacOs operating systems.
    MacOs,

    /// Only target Windows operating systems.
    Windows,
}
