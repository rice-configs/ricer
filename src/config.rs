// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration file management.
//!
//! This module houses objects that allow for the manipulation of Ricer's
//! configuration file data.

use anyhow::Result;
use log::trace;
use std::fmt;
use std::path::PathBuf;

mod parser;
mod hook;
mod repo;

#[doc(inline)]
pub use parser::*;
pub use hook::*;
pub use repo::*;


/// Manage repository configuration file.
#[derive(Clone, Debug, Default)]
pub struct RepoConfig {
    path: PathBuf,
    toml: TomlParser,
}

impl RepoConfig {
    /// Construct new repository configuration file manager
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::RepoConfig;
    ///
    /// let config = RepoConfig::new("/path/to/repos.toml");
    /// ```
    pub fn new(path: impl Into<PathBuf>) -> Self {
        trace!("Construct new repository configuration file manager");
        Self { path: path.into(), toml: TomlParser::new() }
    }

    /// Read repository configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if path to repository configuration file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::RepoConfig;
    ///
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.read()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&mut self) -> Result<()> {
        self.toml.read(&self.path)?;
        Ok(())
    }

    /// Write repository configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if path to repository configuration file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::RepoConfig;
    ///
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.write()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write(&mut self) -> Result<()> {
        self.toml.write(&self.path)?;
        Ok(())
    }

    /// Add repository definition entry into configuration file.
    ///
    /// Inserts repository entry into `repos` section. If the `repos` section
    /// does not exist, then a new `repos` section will be created with the
    /// repository entry inserted.
    ///
    /// Will return previous repository entry if target repository entry
    /// replaced it in the configuration file. Otherwise, will return `None`
    /// if provided repository entry was a new addition to configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if existing `repos` section was not defined as a table.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(repo)?;
    /// let expect = indoc! {r#"
    ///     [repos.vim]
    ///     branch = "main"
    ///     remote = "origin"
    ///     workdir_home = true
    /// "#};
    /// let result = config.to_string();
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_repo(&mut self, entry: RepoEntry) -> Result<Option<RepoEntry>> {
        let (key, item) = entry.to_toml();
        let old_entry = self
            .toml
            .add_entry("repos", (key, item))?
            .map(|(key, item)| RepoEntry::from((&key, &item)));
        Ok(old_entry)
    }

    /// Get repository entry from configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, or `repos` section was not
    /// defined as a table. Will also fail if repository entry does not exist
    /// in `repos` section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let expect = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(expect.clone())?;
    /// let result = config.get_repo("vim")?;
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_repo(&self, name: impl AsRef<str>) -> Result<RepoEntry> {
        let entry = self.toml.get_entry("repos", name.as_ref())?;
        Ok(RepoEntry::from(entry))
    }

    /// Remove repository entry from configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, or `repos` section was not
    /// defined as a table. Will also fail if repository entry does not exist
    /// in `repos` section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let expect = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(expect.clone())?;
    /// let result = config.remove_repo("vim")?;
    /// assert_eq!(expect, result);
    /// assert_eq!("", config.to_string());
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_repo(&mut self, name: impl AsRef<str>) -> Result<RepoEntry> {
        let (key, item) = self.toml.remove_entry("repos", name.as_ref())?;
        Ok(RepoEntry::from((&key, &item)))
    }

    /// Rename repository entry in configuration file.
    ///
    /// Provides previous repository entry in deserialized form.
    ///
    /// # Errors
    ///
    /// Will fail if `repos` section does not exist, or `repos` section was not
    /// defined as a table. Will also fail if repository entry does not exist
    /// in `repos` section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use ricer::config::{RepoConfig, RepoEntry};
    ///
    /// let expect = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .workdir_home(true)
    ///     .build();
    /// let mut config = RepoConfig::new("/path/to/repos.toml");
    /// config.add_repo(expect.clone())?;
    /// config.rename_repo("vim", "neovim")?;
    ///
    /// let expect = indoc! {r#"
    ///     [repos.neovim]
    ///     branch = "main"
    ///     remote = "origin"
    ///     workdir_home = true
    /// "#};
    /// let result = config.to_string();
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn rename_repo<S>(&mut self, from: S, to: S) -> Result<RepoEntry>
    where
        S: AsRef<str>,
    {
        let (key, item) = self.toml.rename_entry("repos", from.as_ref(), to.as_ref())?;
        Ok(RepoEntry::from((&key, &item)))
    }
}

impl fmt::Display for RepoConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toml)
    }
}

/// Command hook configuration file manager.
#[derive(Clone, Debug, Default)]
pub struct CmdHookConfig {
    path: PathBuf,
    toml: TomlParser,
}

impl CmdHookConfig {
    /// Construct command hook configuration file manager.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::CmdHookConfig;
    ///
    /// let config = CmdHookConfig::new("/path/to/hooks.toml");
    /// ```
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), toml: TomlParser::new() }
    }

    /// Read from command hook configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if path to command hook configuration file does not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::CmdHookConfig;
    ///
    /// let mut config = CmdHookConfig::new("/path/to/hooks.toml");
    /// config.read()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&mut self) -> Result<()> {
        self.toml.read(&self.path)?;
        Ok(())
    }

    /// Get command hook entry from configuration file.
    ///
    /// # Errors
    ///
    /// Will fail if `hooks` section does not exist, or `hooks` section was not
    /// defined as a table. Will also fail if hooksitory entry does not exist
    /// in `hooks` section.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::{CmdHookConfig, CmdHookEntry};
    ///
    /// let mut expect = CmdHookEntry::new("commit");
    /// expect.add_hook(HookEntry::builder().pre("hook.sh").build());
    /// expect.add_hook(HookEntry::builder().post("hook.sh").workdir("/some/path").build());
    ///
    /// let mut config = CmdHookConfig::new("/path/to/hooks/toml");
    /// config.add_cmd_hook(expect.clone())?;
    ///
    /// let result = config.get_cmd_hook("commit")?;
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// ```
    pub fn get_cmd_hook(&self, name: impl AsRef<str>) -> Result<CmdHookEntry> {
        let entry= self.toml.get_entry("hooks", name.as_ref())?;
        Ok(CmdHookEntry::from(entry))
    }
}
