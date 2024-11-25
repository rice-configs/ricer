// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use git2::{Error as Git2Error, Repository, RepositoryInitOptions};
use std::path::Path;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    /// Create new Git repository at `path`.
    ///
    /// Will create any necessary directories to repository.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if repository cannot be created.
    pub fn init(path: impl AsRef<Path>) -> Result<Self, GitRepoError> {
        let repo = Repository::init(path.as_ref())?;
        Ok(Self { repo })
    }

    /// Create new Git repository that uses fake bare technique at `path`.
    ///
    /// Will create any necessary directories to fake bare repository.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if repository cannot be created.
    pub fn init_fake_bare(
        gitdir: impl AsRef<Path>,
        workdir: impl AsRef<Path>,
    ) -> Result<Self, GitRepoError> {
        let mut opts = RepositoryInitOptions::new();
        opts.bare(false);
        opts.no_dotgit_dir(true);
        opts.workdir_path(workdir.as_ref());

        let repo = Repository::init_opts(gitdir.as_ref(), &opts)?;
        Ok(Self { repo })
    }

    /// Open existing Git repository at `path`.
    ///
    /// Will open both normal, bare, and fake bare repositories.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if repository cannot be opened.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, GitRepoError> {
        let repo = Repository::open(path.as_ref())?;
        Ok(Self { repo })
    }

    pub fn is_fake_bare(&self) -> bool {
        !self.repo.is_bare() && !self.repo.path().join(".git").exists()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitRepoError {
    #[error("Failed to perform libgit2 operation")]
    LibGit2 { source: Git2Error },
}

impl From<Git2Error> for GitRepoError {
    fn from(err: Git2Error) -> Self {
        GitRepoError::LibGit2 { source: err }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testenv::{FixtureHarness, FixtureKind};

    use anyhow::Result;
    use indoc::indoc;
    use rstest::{fixture, rstest};

    #[fixture]
    fn repo_dir() -> Result<FixtureHarness> {
        let repo_dir = FixtureHarness::open()?;
        Ok(repo_dir)
    }

    #[rstest]
    fn git_repo_init_return_self(repo_dir: Result<FixtureHarness>) -> Result<()> {
        let repo_dir = repo_dir?;
        let repo = GitRepo::init(repo_dir.as_path().join("foo"))?;
        assert!(!repo.is_fake_bare());
        Ok(())
    }

    #[rstest]
    fn git_repo_init_fake_bare_return_self(repo_dir: Result<FixtureHarness>) -> Result<()> {
        let repo_dir = repo_dir?;
        let repo = GitRepo::init_fake_bare(repo_dir.as_path().join("foo"), repo_dir.as_path())?;
        assert!(repo.is_fake_bare());
        Ok(())
    }

    // #[rstest]
    // fn git_repo_open_return_self(repo_dir: Result<FixtureHarness>) -> Result<()> {
    //     let repo_dir = repo_dir?;

    //     let fixture = repo_dir.repo("dwm.git")?;
    //     let repo = GitRepo::open(fixture.path())?;
    //     assert!(!repo.is_fake_bare());

    //     let fixture = repo_dir.repo("vim.git")?;
    //     let repo = GitRepo::open(fixture.path())?;
    //     assert!(repo.is_fake_bare());

    //     Ok(())
    // }
}
