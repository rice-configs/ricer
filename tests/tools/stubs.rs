// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use std::fs::{metadata, set_permissions, write};
use std::path::{PathBuf, Path};

pub struct StubFile {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl StubFile {
    pub fn builder() -> StubFileBuilder {
        StubFileBuilder::new()
    }

    pub fn as_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    pub fn is_executable(&self) -> bool {
        self.executable
    }
}

pub struct StubFileBuilder {
    path: PathBuf,
    data: String,
    executable: bool,
}

impl StubFileBuilder {
    pub fn new() -> Self {
        Self { path: PathBuf::default(), data: String::default(), executable: false }
    }

    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = path.as_ref().to_path_buf();
        self
    }

    pub fn data(mut self, data: impl AsRef<str>) -> Self {
        self.data = data.as_ref().to_string();
        self
    }

    pub fn executable(mut self, flag: bool) -> Self {
        self.executable = flag;
        self
    }

    pub fn build(self) -> StubFile {
        write(&self.path, &self.data).unwrap_or_else(|error| {
            panic!("Failed to create file '{}': {}", self.path.display(), error)
        });

        #[cfg(unix)]
        if self.executable {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = metadata(&self.path).unwrap().permissions();
            let mode = perms.mode();

            perms.set_mode(mode | 0o111);
            set_permissions(&self.path, perms).unwrap();
        }

        StubFile { path: self.path, data: self.data, executable: self.executable }
    }
}
