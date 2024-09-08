// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use std::collections::HashMap;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

use crate::fakes::FakeHomeDir;
use crate::fixtures::{DotfileBuilder, GitRepoFixture};
use crate::util::err_check;

/// Fake Ricer's expected configuration data directory.
///
/// Ricer's configuration data directory houses all repository data tha needs
/// to managed and manipulated. This fake implementation mainly exists to
/// separate unit and integration tests from the user's home directory in order
/// to avoid messing with any of their existing repositories, and make any test
/// function independent of the user's filesystem.
///
/// Caller is expected to fill this fake configuration data directory with
/// Git repository fixture data in order to test any of Ricer's internal
/// library API that requires access to the user's filesystem.
pub struct FakeDataDir {
    home_dir: FakeHomeDir,
    data_dir: PathBuf,
    git_fixtures: HashMap<PathBuf, GitRepoFixture>,
}

impl FakeDataDir {
    /// Build new fake configuration data directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fakes::FakeDataDir;
    ///
    /// let fake = FakeDataDir::builder()
    ///     .git("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim here")
    ///             .file("data/exrc", "configure vi here")
    ///     })
    ///     .git_fake_bare("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim here")
    ///             .file("data/exrc", "configure vi here")
    ///     })
    ///     .build();
    /// assert!(fake.get_git_repo("vim").as_path().join("vimrc").exists());
    /// assert!(fake.home_dir().join("vimrc").exists());
    /// ```
    pub fn builder() -> FakeDataDirBuilder {
        FakeDataDirBuilder::new()
    }

    /// Get tracked git repository fixture.
    ///
    /// # Panics
    ///
    /// Will panic if git repository is not being tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer_test_tools::fakes::FakeDataDir;
    ///
    /// let fake = FakeDataDir::builder()
    ///     .git("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim here")
    ///             .file("data/exrc", "configure vi here")
    ///     })
    ///     .build();
    /// let vim = fake.get_git_repo("vim");
    /// assert_eq!(vim.as_path(), fake.data_dir().join("vim.git"));
    /// ```
    pub fn get_git_repo(&self, name: impl AsRef<str>) -> &GitRepoFixture {
        match self.git_fixtures.get(&self.data_dir.join(format!("{}.git", name.as_ref()))) {
            Some(repo) => repo,
            None => panic!("Git repository not being tracked"),
        }
    }

    /// Get path to fake home directory.
    pub fn home_dir(&self) -> &Path {
        self.home_dir.as_path()
    }

    /// Get path to fake configuration data directory.
    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
}

pub struct FakeDataDirBuilder {
    home_dir: FakeHomeDir,
    data_dir: PathBuf,
    git_fixtures: HashMap<PathBuf, GitRepoFixture>,
}

impl FakeDataDirBuilder {
    /// Construct new fake configuration data directory builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeDataDirBuilder;
    ///
    /// let builder = FakeDataDirBuilder::new();
    /// ```
    pub fn new() -> Self {
        let home_dir = FakeHomeDir::new();
        let data_dir = home_dir.as_path().join("ricer");
        err_check!(create_dir(&data_dir));
        Self { home_dir, data_dir, git_fixtures: HashMap::default() }
    }

    /// Add regular git repository fixture.
    ///
    /// Git repository fixture will have the git directory and working directory
    /// be the same as each other, and will not be bare or _fake bare_. Just a
    /// normal git repository will be created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeDataDirBuilder;
    ///
    /// let builder = FakeDataDirBuilder::new()
    ///     .git("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim!")
    ///             .file("exrc", "configure vi!")
    ///     });
    /// ```
    pub fn git<F>(mut self, name: impl AsRef<str>, callback: F) -> Self
    where
        F: FnOnce(DotfileBuilder) -> DotfileBuilder,
    {
        let git_dir = self.data_dir.join(format!("{}.git", name.as_ref()));
        let repo = GitRepoFixture::init(git_dir.as_path());
        let mut dotfile = DotfileBuilder::new().root(git_dir.as_path());
        dotfile = callback(dotfile);
        dotfile.build();
        repo.add_all();
        repo.commit("Initial commit");
        self.git_fixtures.insert(repo.as_path().into(), repo);
        self
    }

    /// Add _fake bare_ git repository fixture that uses.
    ///
    /// The Git repository will seem like a bare repository, but retain a
    /// working tree.  Git directory will point to the repository itself in the
    /// fake configuration data directory, while the working directory will
    /// point to the fake home directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeDataDirBuilder;
    ///
    /// let builder = FakeDataDirBuilder::new()
    ///     .git_fake_bare("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim!")
    ///             .file("exrc", "configure vi!")
    ///     });
    /// ```
    pub fn git_fake_bare<F>(mut self, name: impl AsRef<str>, callback: F) -> Self
    where
        F: FnOnce(DotfileBuilder) -> DotfileBuilder,
    {
        let git_dir = self.data_dir.join(format!("{}.git", name.as_ref()));
        let work_dir = self.home_dir.as_path();
        let repo = GitRepoFixture::init_fake_bare(git_dir.as_path(), work_dir);
        let mut dotfile = DotfileBuilder::new().root(work_dir);
        dotfile = callback(dotfile);
        dotfile.build();
        repo.add_all();
        repo.commit("Initial commit");
        self.git_fixtures.insert(repo.as_path().into(), repo);
        self
    }

    /// Build new [`FakeDataDir`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fakes::FakeDataDirBuilder;
    ///
    /// let fake = FakeDataDirBuilder::new()
    ///     .git("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim here")
    ///             .file("data/exrc", "configure vi here")
    ///     })
    ///     .git_fake_bare("vim", |dotfiles| {
    ///         dotfiles
    ///             .file("vimrc", "configure vim here")
    ///             .file("data/exrc", "configure vi here")
    ///     })
    ///     .build();
    /// ```
    pub fn build(self) -> FakeDataDir {
        FakeDataDir {
            home_dir: self.home_dir,
            data_dir: self.data_dir,
            git_fixtures: self.git_fixtures,
        }
    }
}
