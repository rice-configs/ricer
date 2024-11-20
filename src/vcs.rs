// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use git2::{Error as Git2Error, Repository};
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
        todo!();
    }

    pub fn is_fake_bare(&self) -> bool {
        todo!();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitRepoError {
    #[error("Failed to perform libgit2 operation")]
    LibGit2 { source: Git2Error },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testenv::FakeDir;

    use anyhow::Result;
    use rstest::{fixture, rstest};

    #[fixture]
    fn repo_dir() -> Result<FakeDir> {
        let fake = FakeDir::open()?;
        Ok(fake)
    }

    #[rstest]
    fn git_repo_init_return_self(repo_dir: Result<FakeDir>) -> Result<()> {
        let repo_dir = repo_dir?;
        let repo = GitRepo::init(repo_dir.as_path().join("foo"))?;
        assert!(!repo.is_fake_bare());
        Ok(())
    }
}
