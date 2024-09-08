// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use git2::{IndexAddOption, Repository, RepositoryInitMode, RepositoryInitOptions};
use std::path::{Path, PathBuf};

use crate::util::err_check;

/// Basic git repository fixture.
///
/// Allows caller to create a basic git repository fixture, or a git repository
/// that uses the _fake bare_ technique. A fake bare repository is a repository
/// does not have a ".git" directory similar to a bare repository, but still
/// retains a working tree that is different from the git directory itself.
pub struct GitRepoFixture {
    git_dir: PathBuf,
    repo: Repository,
}

impl GitRepoFixture {
    /// Create a new regular git repository.
    ///
    /// # Panics
    ///
    /// Will panic if git repository cannot be created and initialized.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::GitRepoFixture;
    ///
    /// let repo = GitRepoFixture::init("vim");
    /// ```
    pub fn init(path: impl AsRef<Path>) -> Self {
        let repo = err_check!(Repository::init(path.as_ref()));
        Self { git_dir: path.as_ref().into(), repo }
    }

    /// Create a new _fake bare_ git repository.
    ///
    /// # Panics
    ///
    /// Will panic if repository cannot be created with "fake bare" technique.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::GitRepoFixture;
    /// use ricer_test_tools::fakes::FakeHomeDir;
    ///
    /// let fake_home = FakeHomeDir::new();
    /// let repo = GitRepoFixture::init_fake_bare(fake_home.as_path().join("vim"), fake_home.as_path());
    /// ```
    pub fn init_fake_bare(path: impl AsRef<Path>, work_dir: impl AsRef<Path>) -> Self {
        let mut opts = RepositoryInitOptions::new();
        opts.workdir_path(work_dir.as_ref());
        opts.bare(false);
        opts.mode(RepositoryInitMode::SHARED_UMASK);
        opts.no_dotgit_dir(true);
        let repo = err_check!(Repository::init_opts(path.as_ref(), &opts));
        Self { git_dir: path.as_ref().into(), repo }
    }

    /// Add all currently unstaged files in repository.
    ///
    /// # Panics
    ///
    /// Will panic if any issues are encountered with adding unstaged files.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::write;
    ///
    /// use ricer_test_tools::fixtures::GitRepoFixture;
    /// use ricer_test_tools::fakes::FakeHomeDir;
    /// use ricer_test_tools::util::err_check;
    ///
    /// let fake_home = FakeHomeDir::new();
    /// let repo = GitRepoFixture::init_fake_bare(fake_home.as_path().join("vim"), fake_home.as_path());
    /// err_check!(write(fake_home.as_path().join(".vimrc"), "configuring vim!"));
    /// repo.add_all();
    /// ```
    pub fn add_all(&self) {
        let mut index = err_check!(self.repo.index());
        err_check!(index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None));
        err_check!(index.write());
    }

    /// Commit changes staged changes.
    ///
    /// # Panics
    ///
    /// Will panic if staged panics cannot be committed for whatever reason.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::write;
    ///
    /// use ricer_test_tools::fixtures::GitRepoFixture;
    /// use ricer_test_tools::fakes::FakeHomeDir;
    /// use ricer_test_tools::util::err_check;
    ///
    /// let fake_home = FakeHomeDir::new();
    /// let repo = GitRepoFixture::init_fake_bare(fake_home.as_path().join("vim"), fake_home.as_path());
    /// err_check!(write(fake_home.as_path().join(".vimrc"), "configuring vim!"));
    /// repo.add_all();
    /// repo.commit("Initial commit");
    /// ```
    pub fn commit(&self, msg: impl AsRef<str>) {
        let mut index = err_check!(self.repo.index());
        let tree_id = err_check!(index.write_tree());
        let sig = err_check!(self.repo.signature());
        let mut parents = Vec::new();
        if let Some(parent) = self.repo.head().ok().map(|h| h.target().unwrap()) {
            parents.push(err_check!(self.repo.find_commit(parent)));
        }
        let parents = parents.iter().collect::<Vec<_>>();
        err_check!(self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            msg.as_ref(),
            &self.repo.find_tree(tree_id).expect("Failed to find tree"),
            &parents,
        ));
    }

    /// Get current hash id of HEAD.
    ///
    /// # Panics
    ///
    /// Will panic if hash id of HEAD cannot be obtained for whatever reason.
    pub fn rev_parse_head(&self) -> String {
        err_check!(self.repo.revparse_single("HEAD")).id().to_string()
    }

    /// Get path to git repository.
    pub fn as_path(&self) -> &Path {
        self.git_dir.as_path()
    }
}
