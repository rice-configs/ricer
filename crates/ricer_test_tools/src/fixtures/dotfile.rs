// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use crate::fixtures::FileFixtureBuilder;

/// Basic dotfile fixture.
///
/// Provides simple way to construct dotfiles to be used for unit and
/// integration testing.
pub struct Dotfile {
    root: PathBuf,
}

impl Dotfile {
    /// Build a new dotfile fixture.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::Dotfile;
    ///
    /// let dotfile = Dotfile::builder()
    ///     .root(".vim")
    ///     .file("vimrc", "configure vim right here!")
    ///     .build();
    /// ```
    pub fn builder() -> DotfileBuilder {
        DotfileBuilder::new()
    }

    /// Root path to dotfile configuration fixture.
    pub fn root(&self) -> &Path {
        self.root.as_path()
    }
}

/// Builder for [`Dotfile`].
#[derive(Debug, Default, Clone)]
pub struct DotfileBuilder {
    root: PathBuf,
    files: Vec<FileFixtureBuilder>,
}

impl DotfileBuilder {
    /// Construct new dotfile fixture builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::DotfileBuilder;
    ///
    /// let builder = DotfileBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set root location of dotfile fixture.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::DotfileBuilder;
    ///
    /// let builder = DotfileBuilder::new().root("foo");
    /// ```
    pub fn root(mut self, root: impl AsRef<Path>) -> Self {
        self.root = root.as_ref().into();
        self
    }

    /// Add file fixture.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::DotfileBuilder;
    ///
    /// let builder = DotfileBuilder::new().file(".vimrc", "configure vim here!");
    /// ```
    pub fn file(mut self, name: impl AsRef<Path>, data: impl AsRef<str>) -> Self {
        let fixture =
            FileFixtureBuilder::new().path(self.root.join(name.as_ref())).data(data.as_ref());
        self.files.push(fixture);
        self
    }

    /// Build dotfile fixture.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer_test_tools::fixtures::DotfileBuilder;
    ///
    /// let dotfile = DotfileBuilder::new()
    ///     .root(".vim")
    ///     .file("vimrc", "configure vim right here!")
    ///     .build();
    /// ```
    pub fn build(mut self) {
        for file in self.files.iter_mut() {
            file.write();
        }
    }
}
