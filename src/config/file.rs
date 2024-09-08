// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration file management.
//!
//! This module provides a simple interface to manipulate and manage Ricer's
//! configuration file. Ricer uses a special configuration file named
//! `config.toml` in its base directory in `$XDG_CONFIG_HOME/ricer`. This
//! configuration file uses the [TOML format[toml-spec] so the user can modify
//! it by hand in case they do not want to go through Ricer's command set for
//! whatever reason.
//!
//! [toml-spec]: https://toml.io/en/v1.0.0

use anyhow::anyhow;
use log::{debug, trace, warn};
use std::fmt::{Display, Formatter, Result};
use std::fs::{read_to_string, write};
use std::path::Path;
use toml_edit::{DocumentMut, Item, Key, Table};

pub mod hooks_section;
pub mod repos_section;

use crate::error::RicerResult;
use hooks_section::CommandHookEntry;
use repos_section::RepoEntry;

/// Configuration file manager representation.
pub trait ConfigFileManager: Display {
    /// Read from configuration file at provided path.
    fn read(&mut self, path: impl AsRef<Path>) -> RicerResult<()>;

    /// Write to configuration file at provided path.
    fn write(&mut self, path: impl AsRef<Path>) -> RicerResult<()>;

    /// Deserialize repository entry from parsed configuration file data.
    fn get_repo(&self, repo_name: impl AsRef<str>) -> RicerResult<RepoEntry>;

    /// Serialize repository entry into parsed configuration file data.
    fn add_repo(&mut self, repo_entry: &RepoEntry) -> RicerResult<()>;

    /// Remove repository entry from configuration file data.
    fn remove_repo(&mut self, repo_name: impl AsRef<str>) -> RicerResult<()>;

    /// Rename repository entry in configuration file data.
    fn rename_repo(&mut self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()>;

    /// Deserialize command hook entry from parsed configuration file data.
    fn get_cmd_hook(&self, cmd_name: impl AsRef<str>) -> RicerResult<CommandHookEntry>;
}

/// Default implementation of configuration file manager.
///
/// # Invariants
///
/// 1. Preserve original formatting and comments of user's configuration file.
#[derive(Debug, Default)]
pub struct DefaultConfigFileManager {
    doc: DocumentMut,
}

impl DefaultConfigFileManager {
    /// Construct new default configuration file manager.
    ///
    /// # Postconditions
    ///
    /// 1. Obtain new valid instance of configuration file manager.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::file::DefaultConfigFileManager;
    ///
    /// let cfg_file_mgr = DefaultConfigFileManager::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Get section of configuration file as a table.
    ///
    /// # Preconditions
    ///
    /// 1. Section exists in configuration file.
    /// 2. Section is actually defined as a table.
    ///
    /// # Postconditions
    ///
    /// 1. Return target section as a table.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if section could not be found,
    ///    or was not defined as a table.
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    #[rustfmt::skip]
    fn get_section(&self, name: impl AsRef<str>) -> RicerResult<&Table> {
        let repos = self.doc.get(name.as_ref()).ok_or(anyhow!(
            "Configuration file does not define a '{}' section",
            name.as_ref()
        ))?;
        let repos = repos.as_table().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            name.as_ref()
        ))?;

        Ok(repos)
    }

    /// Get section of configuration file as a mutable table.
    ///
    /// # Preconditions
    ///
    /// 1. Section exists in configuration file.
    /// 2. Section is actually defined as a table.
    ///
    /// # Postconditions
    ///
    /// 1. Return target section as a mutable table.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if section could not be found,
    ///    or was not defined as a table.
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    #[rustfmt::skip]
    fn get_section_mut(&mut self, name: impl AsRef<str>) -> RicerResult<&mut Table> {
        let repos = self.doc.get_mut(name.as_ref()).ok_or(anyhow!(
            "Configuration file does not define a '{}' section",
            name.as_ref()
        ))?;
        let repos = repos.as_table_mut().ok_or(anyhow!(
            "Configuration file does not define '{}' section as a table",
            name.as_ref()
        ))?;

        Ok(repos)
    }
}

impl ConfigFileManager for DefaultConfigFileManager {
    /// Read from configuration file at provided path.
    ///
    /// # Preconditions
    ///
    /// 1. Configuration file exists at provided path.
    /// 2. Configuration file contains valid TOML formatting.
    ///
    /// # Postconditions
    ///
    /// 1. Parse TOML data for future manipulation.
    ///
    /// # Errors
    ///
    /// 1. Returns [`RicerError::Unrecoverable`] if configuration file does not
    ///    exist at provided path, or it contains invalid TOML formatting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.read("/path/to/config.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn read(&mut self, path: impl AsRef<Path>) -> RicerResult<()> {
        debug!("Read configuration file from '{}'", path.as_ref().display());
        let buffer = read_to_string(path.as_ref())?;
        let doc: DocumentMut = buffer.parse()?;
        self.doc = doc;
        Ok(())
    }

    /// Write to configuration file at provided path.
    ///
    /// # Preconditions
    ///
    /// 1. Full path to configuration file exists, i.e., no sub-directories are
    ///    _not_ missing.
    ///
    /// # Postconditions
    ///
    /// 1. If file does not exist, but all sub-directories do exist, then create
    ///    it and write to it.
    /// 2. Preserve original formatting and comments that existed before
    ///    writing.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if sub-directories in provided
    ///    path do not exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.write("/path/to/config.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn write(&mut self, path: impl AsRef<Path>) -> RicerResult<()> {
        debug!("Write configuration file to '{}'", path.as_ref().display());
        let buffer = self.doc.to_string();
        write(path.as_ref(), buffer)?;
        Ok(())
    }

    /// Deserialize repository entry from parsed configuration file data.
    ///
    /// # Preconditions
    ///
    /// 1. The `repos` section exists.
    /// 2. The `repos` section is defined as a table.
    /// 3. Repository definition exists in `repos` section.
    ///
    /// # Postconditions
    ///
    /// 1. Return deserialized [`RepoEntry`].
    /// 2. Will not modify configuration file data.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if target repository does not
    ///    exist, 'repos' section does not exist, or 'repos' section was not
    ///    defined as a table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.read("/path/to/config.toml")?;
    /// let repo = cfg_file_mgr.get_repo("vim")?;
    /// println!("{:#?}", repo);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RepoEntry`]: crate::config::file::repos_section::RepoEntry
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn get_repo(&self, name: impl AsRef<str>) -> RicerResult<RepoEntry> {
        debug!("Get repository '{}' from configuration file", name.as_ref());
        let repos = self.get_section("repos")?;
        let repo = repos.get_key_value(name.as_ref()).ok_or(anyhow!(
            "Repository '{}' does not exist in 'repos' section of configuration file",
            name.as_ref()
        ))?;
        Ok(RepoEntry::from(repo))
    }

    /// Serialize repository entry into parsed configuration file data.
    ///
    /// # Preconditions
    ///
    /// 1. The `repos` section is defined as a table.
    ///
    /// # Postconditions
    ///
    /// 1. Add repository definition into configuration file data.
    ///     - Will add repository entry to existing `repos` section.
    ///     - Will create `repos` section and add repository entry  if and only
    ///       if `repos` section did not exist before.
    ///
    /// # Invariants
    ///
    /// 1. Preserve original comments and formatting that existed beforehand.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if `repos` section is
    ///    not defined as a table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    /// use ricer::config::file::repos_section::RepoEntry;
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.read("/path/to/config.toml")?;
    /// let repo = RepoEntry::builder("vim")
    ///     .branch("main")
    ///     .remote("origin")
    ///     .url("https://github.com/awkless/vim.git")
    ///     .build();
    /// cfg_file_mgr.add_repo(&repo)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn add_repo(&mut self, repo_entry: &RepoEntry) -> RicerResult<()> {
        debug!("Add repository '{}' too configuration file", &repo_entry.name);
        let (repo_name, repo_data) = repo_entry.to_toml();
        if let Some(repos) = self.doc.get_mut("repos") {
            trace!("The 'repos' section exists, add to it");
            let repos = repos.as_table_mut().ok_or(anyhow!(
                "The 'repos' section in configuration file not defined as a table"
            ))?;
            repos.insert(repo_name.get(), repo_data);
        } else {
            trace!("The 'repos' section does not exist, set it up and add to it");
            let mut repos = Table::new();
            repos.insert(repo_name.get(), repo_data);
            repos.set_implicit(true);
            self.doc.insert("repos", Item::Table(repos));
        }

        Ok(())
    }

    /// Remove repository entry from configuration file data.
    ///
    /// # Preconditions
    ///
    /// 1. The `repos` section exists.
    /// 2. The `repos` section is defined as a table.
    /// 3. Repository definition exists in `repos` section.
    ///
    /// # Postconditions
    ///
    /// 1. Remove target repository definition from configuration file data.
    /// 2. Return removed target repository definition as a [`RepoEntry`].
    ///
    /// # Invariants
    ///
    /// 1. Preserve original comments and formatting that existed beforehand.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if target repository does not
    ///    exist, 'repos' section does not exist, or 'repos' section was not
    ///    defined as a table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.read("/path/to/config.toml")?;
    /// let repo = cfg_file_mgr.remove_repo("vim")?;
    /// println!("{:#?}", repo);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RepoEntry`]: crate::config::file::repos_section::RepoEntry
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn remove_repo(&mut self, repo_name: impl AsRef<str>) -> RicerResult<()> {
        debug!("Remove repository '{}' from configuration file", repo_name.as_ref());
        let repos = self.get_section_mut("repos")?;
        let repo = repos.remove_entry(repo_name.as_ref());
        if repo.is_some() {
            warn!("Repository '{}' does not exist in configuration file", repo_name.as_ref());
        }

        Ok(())
    }

    /// Rename repository entry in configuration file data.
    ///
    /// # Preconditions
    ///
    /// 1. The `repos` section exists.
    /// 2. The `repos` section is defined as a table.
    /// 3. Repository definition exists in `repos` section.
    ///
    /// # Postconditions
    ///
    /// 1. Rename target repository with provided new name.
    ///
    /// # Invariants
    ///
    /// 1. Preserve original formatting and comments that existed beforehand.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if 'repos' section was not
    ///    defined as a table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.read("/path/to/config.toml")?;
    /// cfg_file_mgr.rename_repo("vi", "vim")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn rename_repo(&mut self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()> {
        debug!("Rename repository '{}' to '{}' in configuration file", from.as_ref(), to.as_ref());
        let repos = self.get_section_mut("repos")?;
        let (key, value) = repos.remove_entry(from.as_ref()).ok_or(anyhow!(
            "Repository '{}' does not exist in 'repos' section of configuration file",
            from.as_ref()
        ))?;

        // Preserve decor (comments and formatting) from original key...
        let key = Key::new(to.as_ref()).with_leaf_decor(key.leaf_decor().clone());
        repos.insert_formatted(&key, value);
        Ok(())
    }

    /// Deserialize command hook entry from parsed configuration file data.
    ///
    /// # Preconditions
    ///
    /// 1. The `hooks` section exists.
    /// 2. The `hooks` section is defined as a table.
    /// 3. Command hook definition exists in `hooks` section.
    ///
    /// # Postconditions
    ///
    /// 1. Return deserialized [`CommandHookEntry`].
    /// 2. Will not modify configuration file data.
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if target command hook does not
    ///    exist, 'hooks' section does not exist, or 'hooks' section was not
    ///    defined as a table.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::file::{ConfigFileManager, DefaultConfigFileManager};
    ///
    /// let mut cfg_file_mgr = DefaultConfigFileManager::new();
    /// cfg_file_mgr.read("/path/to/config.toml")?;
    /// let hook = cfg_file_mgr.get_cmd_hook("commit")?;
    /// println!("{:#?}", hook);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`CommandHookEntry`]: crate::config::file::hooks_section::CommandHookEntry
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn get_cmd_hook(&self, cmd_name: impl AsRef<str>) -> RicerResult<CommandHookEntry> {
        let hooks = self.get_section("hooks")?;
        let hook = hooks.get_key_value(cmd_name.as_ref()).ok_or(anyhow!(
            "Command hook '{}' does not exist in 'hooks' section of configuration file",
            cmd_name.as_ref()
        ))?;
        Ok(CommandHookEntry::from(hook))
    }
}

impl Display for DefaultConfigFileManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.doc)
    }
}
