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
        let repo = Repository::init(format!("{}.git", path.as_ref().display()))?;
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

        let repo = Repository::init_opts(format!("{}.git", gitdir.as_ref().display()), &opts)?;
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

    /// Clone existing Git repository from `url` into `path`.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if repository cannot be cloned.
    pub fn clone(url: impl AsRef<str>, into: impl AsRef<Path>) -> Result<Self, GitRepoError> {
        let repo = Repository::clone(url.as_ref(), format!("{}.git", into.as_ref().display()))?;
        Ok(Self { repo })
    }

    pub fn is_fake_bare(&self) -> bool {
        !self.repo.is_bare() && !self.repo.path().ends_with(".git")
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
    use crate::testenv::FixtureHarness;

    use anyhow::Result;
    use rstest::{fixture, rstest};

    #[fixture]
    fn repo_dir() -> Result<FixtureHarness> {
        let harness = FixtureHarness::open()?
            .with_repo("dwm", |repo| {
                repo.stage("config.h", "configure DWM settings here")?
                    .stage("dwm.c", "source code for DWM")?
                    .stage("Makefile", "build DWM binary")
            })?
            .with_fake_bare_repo("vim", |repo| {
                repo.stage("vimrc", "config for vim!")?
                    .stage("indent/c.vim", "indentation settings for C code")
            })?
            .setup()?;
        Ok(harness)
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

    #[rstest]
    fn git_repo_open_return_self(repo_dir: Result<FixtureHarness>) -> Result<()> {
        let repo_dir = repo_dir?;

        let fixture = repo_dir.get_repo("dwm")?;
        let repo = GitRepo::open(fixture.as_path())?;
        assert!(!repo.is_fake_bare());

        let fixture = repo_dir.get_repo("vim")?;
        let repo = GitRepo::open(fixture.as_path())?;
        assert!(repo.is_fake_bare());

        Ok(())
    }

    #[rstest]
    fn git_repo_clone_return_self(repo_dir: Result<FixtureHarness>) -> Result<()> {
        let mut repo_dir = repo_dir?;

        let repo = GitRepo::clone(
            "https://github.com/rice-configs/ricer.git",
            repo_dir.as_path().join("ricer"),
        )?;
        repo_dir.sync_untracked()?;
        let fixture = repo_dir.get_repo("ricer")?;
        assert!(fixture.as_path().exists());
        assert!(!repo.is_fake_bare());

        Ok(())
    }
}
