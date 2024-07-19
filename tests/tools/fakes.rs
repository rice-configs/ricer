// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use std::collections::HashMap;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use tempfile::{Builder, TempDir};

use ricer_core::config::ConfigDir;

use crate::tools::stubs::{FileStub, GitRepoStub};

// Create an instance of a fake Ricer configuration directory.
//
// Generally used to decorate `ricer_core::config::Config` with basic fixtures
// for integration testing purposes. This fake configuration directory handler
// generally tries to maintain the structure of Ricer's configuration directory
// for API feedback purposes.
#[derive(Debug)]
pub struct FakeConfigDir {
    // Fake 'base' directory that houses all other fake sub-directories.
    base_dir: TempDir,

    // Fake 'hooks' directory.
    hooks_dir: TempDir,

    // Fake 'repos' directory.
    repos_dir: TempDir,

    // Fake 'ignores' directory.
    ignores_dir: TempDir,

    // Store tracked stub files using their path as the key. HashMap is used for
    // O(1) lookup.
    file_stubs: HashMap<PathBuf, FileStub>,

    // Store tracked stub repositories using their path as the key.
    repo_stubs: HashMap<PathBuf, GitRepoStub>,
}

impl FakeConfigDir {
    // Create an instance of builder to build a new fake configuration directory.
    //
    // Postconditions:
    //
    // 1. Obtain a valid instance of `FakeConfigDirBuilder` to begin building a
    //    fake configuration directory.
    pub fn builder() -> FakeConfigDirBuilder {
        FakeConfigDirBuilder::new()
    }

    // Get stored path to target fake ignore file in 'ignores' directory.
    //
    // Ignore files in Ricer are named after repositories in the `repos`
    // directory with a '.ignore' extension. However, not all repositories will
    // have a corresponding ignore file. Usually, repositories without ignore
    // files are ones who do not target the user's home directory as their
    // working tree.
    //
    // Regardless, to get an absolute path to fake ignore file, the caller
    // just needs to provide the name of the file without the `.ignore`
    // extension.
    //
    // Preconditions:
    //
    // 1. Ignore file is being tracked by fake configuration directory.
    //
    // Postconditions:
    //
    // 1. Get absolute path to fake ignore file.
    //
    // Errors:
    //
    // Panics if ignore file is not being tracked by fake configuration
    // directory.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDir;
    //
    // let config = FakeConfigDir::builder()
    //     .ignore_file("fake_ignore", "/*") // Stored as 'fake_ignore.ignore'
    //     .build();
    // let path = config.path_to_ignore_file("fake_ignore");
    // ```
    pub fn path_to_ignore_file(&self, repo: impl AsRef<Path>) -> &FileStub {
        let ignore_file = format!("{}.ignore", repo.as_ref().display());
        match self.file_stubs.get(&self.ignores_dir().join(&ignore_file)) {
            Some(file) => file,
            None => panic!("Ignore file '{}' is not being tracked by fake directory", &ignore_file),
        }
    }

    // Get path to stored hook script in fake 'hooks' directory.
    //
    // Caller needs to provide full filename of hook to obtain its path.
    //
    // Preconditions:
    //
    // 1. Hook script must be currently tracked by fake configuration directory.
    //
    // Postconditions:
    //
    // 1. Get absolute path to hook script in fake 'hooks' directory.
    //
    // Errors:
    //
    // Panics if named hook script is not being tracked by fake configuration
    // directory.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDir;
    //
    // let config = FakeConfigDir::builder()
    //     .hook_script("hook.sh", "chmod +x blah")
    //     .build();
    // let path = config.path_to_hook_script("hook.sh");
    // ```
    pub fn path_to_hook_script(&self, name: impl AsRef<Path>) -> &FileStub {
        match self.file_stubs.get(&self.hooks_dir().join(name.as_ref())) {
            Some(file) => file,
            None => panic!(
                "Hook script '{}' is not being tracked by fake directory",
                &name.as_ref().display()
            ),
        }
    }

    // Get path to stored Git repository in fake 'repos' directory.
    //
    // Caller needs to provide full filename of hook to obtain its path.
    //
    // Preconditions:
    //
    // 1. Git repository must be currently tracked by fake configuration
    //    directory.
    //
    // Postconditions:
    //
    // 1. Get absolute path to Git repository in fake 'repos' directory.
    //
    // Errors:
    //
    // Panics if named Git repository is not being tracked by fake configuration
    // directory.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDir;
    //
    // let config = FakeConfigDir::builder()
    //     .git_repo("fake_repo")
    //     .build();
    // let path = config.path_to_hook_script("hook.sh");
    // ```
    pub fn path_to_git_repo(&self, name: impl AsRef<Path>) -> &GitRepoStub {
        let git_repo = format!("{}.git", name.as_ref().display());
        match self.repo_stubs.get(&self.repos_dir().join(&git_repo)) {
            Some(repo) => repo,
            None => panic!("Repository '{}' is not being tracked by fake directory", &git_repo),
        }
    }
}

impl ConfigDir for FakeConfigDir {
    fn base_dir(&self) -> &Path {
        self.base_dir.path()
    }

    fn hooks_dir(&self) -> &Path {
        self.hooks_dir.path()
    }

    fn repos_dir(&self) -> &Path {
        self.repos_dir.path()
    }

    fn ignores_dir(&self) -> &Path {
        self.ignores_dir.path()
    }
}

impl Drop for FakeConfigDir {
    fn drop(&mut self) {
        self.file_stubs.clear();
        remove_dir_all(self.ignores_dir.path()).expect("Failed to close 'ignores/' fixture");
        remove_dir_all(self.repos_dir.path()).expect("Failed to close 'repos/' fixture");
        remove_dir_all(self.hooks_dir.path()).expect("Failed to close 'hooks/' fixture");
        remove_dir_all(self.base_dir.path()).expect("Failed to close base directory fixture");
    }
}

#[derive(Debug)]
pub struct FakeConfigDirBuilder {
    base_dir: TempDir,
    hooks_dir: TempDir,
    repos_dir: TempDir,
    ignores_dir: TempDir,
    file_stubs: HashMap<PathBuf, FileStub>,
    repo_stubs: HashMap<PathBuf, GitRepoStub>,
}

impl FakeConfigDirBuilder {
    // Construct new instance of fake configuration directory builder.
    //
    // Postconditions:
    //
    // 1. Get valid instance of fake configuration directory builder.
    //
    // Invariants:
    //
    // 1. Do not leave any fields uninitialized without a sane default value.
    //
    // Errors:
    //
    // Panics if it cannot create the directory structure needed to fake Ricer's
    // configuration directory.
    //
    // Note:
    //
    // Caller should use `FakeConfigDir::builder()` instead of directly calling
    // this method. That way they can use `FileStub` more directly. Unless they
    // need the file stub instance separate from their file stub builder
    // instance for whatever reason (unlikely but possible).
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDirBuilder;
    //
    // let builder = FakeConfigDirBuilder::new();
    // ```
    pub fn new() -> Self {
        let base_dir =
            Builder::new().prefix("ricer").tempdir().expect("Failed to create base directory");

        let hooks_dir = Builder::new()
            .prefix("hooks")
            .tempdir_in(base_dir.path())
            .expect("Failed to create 'hooks' directory");

        let repos_dir = Builder::new()
            .prefix("repos")
            .tempdir_in(base_dir.path())
            .expect("Failed to create 'repos' directory");

        let ignores_dir = Builder::new()
            .prefix("ignores")
            .tempdir_in(base_dir.path())
            .expect("Failed to create 'ignores' directory");

        Self {
            base_dir,
            hooks_dir,
            repos_dir,
            ignores_dir,
            file_stubs: HashMap::default(),
            repo_stubs: HashMap::default(),
        }
    }

    // Write fake configuration file in fake base directory.
    //
    // Postconditions:
    //
    // 1. Write a fake configuration file in the fake base directory retaining
    //    file stub data.
    //
    // Errors:
    //
    // Panics if it cannot create fake configuration file.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDirBuilder;
    //
    // let builder = FakeConfigDirBuilder::new()
    //     .config_file("[hooks]");
    // ```
    pub fn config_file(mut self, data: impl AsRef<str>) -> Self {
        let config_stub = FileStub::builder()
            .path(self.base_dir.path().join("config.toml"))
            .data(data.as_ref())
            .executable(false)
            .build();

        self.file_stubs.insert(config_stub.as_path().to_path_buf(), config_stub);
        self
    }

    // Write fake ignore file in fake 'ignores' directory.
    //
    // Postconditions:
    //
    // 1. Write a fake ignore file in the fake 'ignores' directory retaining
    //    file stub data.
    //
    // Errors:
    //
    // Panics if it cannot create fake ignore file in 'ignores' directory.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDirBuilder;
    //
    // let builder = FakeConfigDirBuilder::new()
    //     .ignore_file("fake_ignore", "/*");
    // ```
    pub fn ignore_file(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let file_stub = FileStub::builder()
            .path(self.ignores_dir.path().join(format!("{}.ignore", name.as_ref())))
            .data(data.as_ref())
            .executable(false)
            .build();

        self.file_stubs.insert(file_stub.as_path().to_path_buf(), file_stub);
        self
    }

    // Create executable fake hook script in the fake 'hooks' directory.
    //
    // Postconditions:
    //
    // 1. Create executable hook script in fake 'hooks' directory retaining
    //    file stub data.
    //
    // Errors:
    //
    // Panics if it cannot create executable hook script for whatever reason.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDirBuilder;
    //
    // let builder = FakeConfigDirBuilder::new()
    //     .hook_script("fake_hook", "chmod +x somefile.txt");
    // ```
    pub fn hook_script(mut self, name: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let file_stub = FileStub::builder()
            .path(self.hooks_dir.path().join(name.as_ref()))
            .data(data.as_ref())
            .executable(true)
            .build();

        self.file_stubs.insert(file_stub.as_path().to_path_buf(), file_stub);
        self
    }

    // Create Git repository in 'repos' directory.
    //
    // Postconditions:
    //
    // 1. Create Git repository in 'repos' directory retaining repo stub data.
    //
    // Errors:
    //
    // Panics if it cannot create the Git repository.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDirBuilder;
    //
    // let builder = FakeConfigDirBuilder::new()
    //     .git_repo("fake_repo")
    // ```
    pub fn git_repo(mut self, name: impl AsRef<str>) -> Self {
        let repo = format!("{}.git", name.as_ref());
        let repo_stub = GitRepoStub::new(self.repos_dir.path().join(repo));
        self.repo_stubs.insert(repo_stub.as_path().to_path_buf(), repo_stub);
        self
    }

    // Build fake configuration directory instance.
    //
    // Postconditions:
    //
    // 1. Provide `FakeConfigDir` instance with what was built.
    //
    // Examples:
    //
    // ```
    // use crate::tools::fakes::FakeConfigDirBuilder;
    //
    // let config = FakeConfigDirBuilder::new()
    //     .hook_script("fake_hook", "chmod +x somefile.txt")
    //     .ignore_file("fake_ignore", "/*")
    //     .build();
    // ```
    pub fn build(self) -> FakeConfigDir {
        FakeConfigDir {
            base_dir: self.base_dir,
            hooks_dir: self.hooks_dir,
            repos_dir: self.repos_dir,
            ignores_dir: self.ignores_dir,
            file_stubs: self.file_stubs,
            repo_stubs: self.repo_stubs,
        }
    }
}
