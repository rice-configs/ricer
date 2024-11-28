// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use git2::{
    build::CheckoutBuilder, AnnotatedCommit, AutotagOption, BranchType, Commit, Error as Git2Error,
    FetchOptions, Oid, Reference, Remote, RemoteCallbacks, Repository, RepositoryInitOptions,
};
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

    /// Find a commit from object ID.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if commit cannot be found from OID.
    pub fn find_commit(&self, oid: Oid) -> Result<Commit<'_>, GitRepoError> {
        let commit = self.repo.find_commit(oid)?;
        Ok(commit)
    }

    /// Pull changes from Git repository remote and branch.
    ///
    /// Performs a fetch and then merges any changes. Will perform a fast-forward
    /// merge if `branch` has not diverged from `remote`. Will perform a commit
    /// merge is `branch` does diverge from `remote`.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::LibGit2`] if pull cannot be performed.
    pub fn pull(
        &self,
        remote: impl AsRef<str>,
        branch: impl AsRef<str>,
    ) -> Result<(), GitRepoError> {
        let mut remote = self.repo.find_remote(remote.as_ref())?;
        let fetch = self.fetch(&[branch.as_ref()], &mut remote)?;
        self.full_merge(branch.as_ref(), fetch)?;
        Ok(())
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

    /// Use Git binary directly on this repository.
    ///
    /// Useful to gain access to full Git binary for functionality not offered
    /// by libgit2.
    ///
    /// # Errors
    ///
    /// - Return [`GitRepoError::Syscall`] if system call to Git binary failed.
    /// - Return [`GitRepoError::GitBin`] if Git binary itself fails.
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

    pub(crate) fn fetch(
        &self,
        refs: &[&str],
        remote: &mut Remote,
    ) -> Result<AnnotatedCommit, GitRepoError> {
        let mut cb = RemoteCallbacks::new();

        // Print transfer progress...
        cb.transfer_progress(|stats| {
            if stats.received_objects() == stats.total_objects() {
                info!("Resolving deltas {}/{}", stats.indexed_deltas(), stats.total_deltas(),);
            } else if stats.total_objects() > 0 {
                info!(
                    "Received {}/{} objects ({}) in {} bytes",
                    stats.received_objects(),
                    stats.total_objects(),
                    stats.indexed_objects(),
                    stats.received_bytes(),
                );
            }
            true
        });

        let mut opts = FetchOptions::new();
        opts.remote_callbacks(cb);
        opts.download_tags(AutotagOption::All);
        info!("Fetching {} for repo", remote.name().unwrap_or("origin"));
        remote.fetch(refs, Some(&mut opts), None)?;

        let stats = remote.stats();
        if stats.local_objects() > 0 {
            info!(
                "Received {}/{} objects in {} bytes (used {} local objects)",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes(),
                stats.local_objects(),
            );
        } else {
            info!(
                "Received {}/{} objects in {} bytes",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes(),
            );
        }

        let head = self.repo.find_reference("FETCH_HEAD")?;
        let commit = self.repo.reference_to_annotated_commit(&head)?;
        Ok(commit)
    }

    pub(crate) fn fast_forward(
        &self,
        lb: &mut Reference,
        rc: &AnnotatedCommit,
    ) -> Result<(), GitRepoError> {
        let name = match lb.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
        };

        let msg = format!("Fast-forward: settings {} to id: {}", name, rc.id());
        info!("{msg}");
        lb.set_target(rc.id(), &msg)?;
        self.repo.set_head(&name)?;
        self.repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
        Ok(())
    }

    pub(crate) fn normal_merge(
        &self,
        local: &AnnotatedCommit,
        remote: &AnnotatedCommit,
    ) -> Result<(), GitRepoError> {
        let local_tree = self.repo.find_commit(local.id())?.tree()?;
        let remote_tree = self.repo.find_commit(remote.id())?.tree()?;
        let ancestor =
            self.repo.find_commit(self.repo.merge_base(local.id(), remote.id())?)?.tree()?;
        let mut idx = self.repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

        if idx.has_conflicts() {
            info!("Merge conflicts detected...");
            self.repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }

        let result_tree = self.repo.find_tree(idx.write_tree_to(&self.repo)?)?;
        let msg = format!("Merge: {} into {}", remote.id(), local.id());
        let sig = self.repo.signature()?;
        let local_commit = self.repo.find_commit(local.id())?;
        let remote_commit = self.repo.find_commit(remote.id())?;
        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;

        self.repo.checkout_head(None)?;
        Ok(())
    }

    pub(crate) fn full_merge(
        &self,
        branch: &str,
        fetch: AnnotatedCommit,
    ) -> Result<(), GitRepoError> {
        let analysis = self.repo.merge_analysis(&[&fetch])?;

        if analysis.0.is_fast_forward() {
            info!("Doing a fast-forward");
            let refname = format!("refs/heads/{}", branch);
            match self.repo.find_reference(&refname) {
                Ok(mut rc) => {
                    self.fast_forward(&mut rc, &fetch)?;
                }
                Err(_) => {
                    self.repo.reference(
                        &refname,
                        fetch.id(),
                        true,
                        &format!("Setting {} to {}", branch, fetch.id()),
                    )?;
                    self.repo.set_head(&refname)?;
                    self.repo.checkout_head(Some(
                        CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            let head = self.repo.reference_to_annotated_commit(&self.repo.head()?)?;
            self.normal_merge(&head, &fetch)?;
        } else {
            info!("Nothing to do!");
        }
        Ok(())
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
