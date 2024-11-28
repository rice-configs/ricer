// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use git2::{BranchType, Commit, Error as Git2Error, Oid, Repository, RepositoryInitOptions};
use log::info;
use std::{ffi::OsStr, io::Error as IoError, path::Path, process::Command};

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

    /// Commit staged changes.
    ///
    /// Will return Git OID of commit.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if commit cannot be created.
    pub fn commit(&self, msg: impl AsRef<str>) -> Result<Oid, GitRepoError> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let sig = self.repo.signature()?;
        let mut parents = Vec::new();

        if let Some(parent) = self.repo.head().ok().map(|h| h.target().unwrap()) {
            parents.push(self.repo.find_commit(parent)?);
        }
        let parents = parents.iter().collect::<Vec<_>>();

        let oid = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            msg.as_ref(),
            &self.repo.find_tree(tree_id).expect("Failed to find tree"),
            &parents,
        )?;

        Ok(oid)
    }

    pub fn find_commit(&self, oid: Oid) -> Result<Commit<'_>, GitRepoError> {
        let commit = self.repo.find_commit(oid)?;
        Ok(commit)
    }

    pub fn push(
        &self,
        remote: impl AsRef<str>,
        branch: impl AsRef<str>,
    ) -> Result<(), GitRepoError> {
        let mut remote = self.repo.find_remote(remote.as_ref())?;
        let branch = self.repo.find_branch(branch.as_ref(), BranchType::Local)?;
        remote.push(&[branch.into_reference().name().unwrap_or("master")], None)?;
        Ok(())
    }

    pub fn syscall(
        &self,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Result<(), GitRepoError> {
        let output = Command::new("git")
            .args([
                "--git-dir",
                self.repo.path().to_str().unwrap(),
                "--work-tree",
                self.repo.workdir().unwrap().to_str().unwrap(),
            ])
            .args(args)
            .output()?;

        if !output.status.success() {
            let msg = String::from_utf8_lossy(output.stderr.as_slice()).into_owned();
            return Err(GitRepoError::GitBin { msg });
        }

        let msg = String::from_utf8_lossy(output.stdout.as_slice()).into_owned();
        info!("Git binary success: {msg}");

        Ok(())
    }

    pub fn is_fake_bare(&self) -> bool {
        !self.repo.is_bare() && !self.repo.path().ends_with(".git")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitRepoError {
    #[error("Failed to perform libgit2 operation")]
    LibGit2 { source: Git2Error },

    #[error("Failed to call Git binary")]
    Syscall { source: IoError },

    #[error("Git binary failure: {msg}")]
    GitBin { msg: String },
}

impl From<Git2Error> for GitRepoError {
    fn from(err: Git2Error) -> Self {
        GitRepoError::LibGit2 { source: err }
    }
}

impl From<IoError> for GitRepoError {
    fn from(err: IoError) -> Self {
        GitRepoError::Syscall { source: err }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testenv::{FileFixture, FileKind, FixtureHarness};

    use anyhow::Result;
    use pretty_assertions::assert_eq;
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
            .with_bare_repo("github")?
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

    #[rstest]
    fn git_repo_commit_return_oid(repo_dir: Result<FixtureHarness>) -> Result<()> {
        let mut repo_dir = repo_dir?;
        let fixture = repo_dir.get_repo_mut("dwm")?;
        let new_file = FileFixture::new(fixture.as_path().join("new.c"))
            .with_data("some new data")
            .with_kind(FileKind::Normal);
        new_file.write()?;
        fixture.add("new.c")?;

        let repo = GitRepo::open(fixture.as_path())?;
        let oid = repo.commit("Add new.c")?;
        let result = repo.find_commit(oid)?;
        fixture.sync()?;
        let expect = fixture.find_commit(oid)?;
        assert_eq!(result.message(), expect.message());

        Ok(())
    }

    #[rstest]
    fn git_repo_push_return_ok(
        repo_dir: Result<FixtureHarness>,
        #[values("vim", "dwm")] repo: &str,
    ) -> Result<()> {
        let repo_dir = repo_dir?;
        let remote = repo_dir.get_repo("github")?;
        let local = repo_dir.get_repo(repo)?;
        let repo = GitRepo::open(local.as_path())?;
        repo.syscall([
            "remote",
            "add",
            "origin",
            format!("file://{}", remote.as_path().display()).as_str(),
        ])?;
        let result = repo.push("origin", "main");
        assert!(result.is_ok());
        Ok(())
    }

    #[rstest]
    fn git_repo_syscall_return_ok(
        repo_dir: Result<FixtureHarness>,
        #[values("vim", "dwm")] repo: &str,
    ) -> Result<()> {
        let repo_dir = repo_dir?;
        let fixture = repo_dir.get_repo(repo)?;
        let repo = GitRepo::open(fixture.as_path())?;
        let result = repo.syscall(["status"]);
        assert!(result.is_ok());
        Ok(())
    }

    #[rstest]
    fn git_repo_syscall_return_err(
        repo_dir: Result<FixtureHarness>,
        #[values("vim", "dwm")] repo: &str,
    ) -> Result<()> {
        let repo_dir = repo_dir?;
        let fixture = repo_dir.get_repo(repo)?;
        let repo = GitRepo::open(fixture.as_path())?;
        let result = repo.syscall(["non-existent-cmd"]);
        assert!(result.is_err());
        Ok(())
    }
}
