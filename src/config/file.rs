// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

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

use log::{debug, trace};
use std::fs::{read_to_string, write};
use std::path::Path;
use toml_edit::{DocumentMut, Item, Table};

pub mod hooks_section;
pub mod repos_section;

use crate::error::{RicerError, RicerResult};
use hooks_section::CommandHookEntry;
use repos_section::RepoEntry;

/// Configuration file manager representation.
pub trait ConfigFileManager {
    /// Read from configuration file at provided path.
    fn read(&mut self, path: impl AsRef<Path>) -> RicerResult<()>;

    /// Write to configuration file at provided path.
    fn write(&mut self, path: impl AsRef<Path>) -> RicerResult<()>;

    /// Show current configuration file data in string form.
    fn to_string(&self) -> String;

    /// Deserialize repository entry from parsed configuration file data.
    fn get_repo(&self, repo_name: impl AsRef<str>) -> RicerResult<RepoEntry>;

    /// Serialize repository entry into parsed configuration file data.
    fn add_repo(&mut self, repo_entry: &RepoEntry) -> RicerResult<()>;

    /// Remove repository entry from configuration file data.
    fn remove_repo(&mut self, repo_name: impl AsRef<str>) -> RicerResult<RepoEntry>;

    /// Rename repository entry in configuration file data.
    fn rename_repo(&mut self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()>;

    /// Deserialize command hook envry from parsed configuration file data.
    fn get_cmd_hook(&self, cmd_name: impl AsRef<str>) -> RicerResult<CommandHookEntry>;
}

#[derive(Debug, Default)]
pub struct DefaultConfigFileManager {
    doc: DocumentMut,
}

impl DefaultConfigFileManager {
    pub fn new() -> Self {
        Default::default()
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
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    fn read(&mut self, path: impl AsRef<Path>) -> RicerResult<()> {
        debug!("Read configuration file from '{}'", path.as_ref().display());
        let buffer = read_to_string(path.as_ref())?;
        let doc: DocumentMut = buffer.parse()?;
        self.doc = doc;
        Ok(())
    }

    /// Write to configuration file at provided path.
    fn write(&mut self, path: impl AsRef<Path>) -> RicerResult<()> {
        debug!("Write configuration file to '{}'", path.as_ref().display());
        let buffer = self.doc.to_string();
        write(path.as_ref(), buffer)?;
        Ok(())
    }

    /// Show current configuration file data in string form.
    fn to_string(&self) -> String {
        self.doc.to_string()
    }

    /// Deserialize repository entry from parsed configuration file data.
    fn get_repo(&self, repo_name: impl AsRef<str>) -> RicerResult<RepoEntry> {
        debug!("Get repository '{}' from configuration file", repo_name.as_ref());
        let repos = self.doc.get("repos").ok_or(RicerError::NoReposSection)?;
        let repos = repos.as_table().ok_or(RicerError::ReposSectionNotTable)?;
        let repo = repos
            .get_key_value(repo_name.as_ref())
            .ok_or(RicerError::NoRepoFound { repo_name: repo_name.as_ref().to_string() })?;
        Ok(RepoEntry::from(repo))
    }

    /// Serialize repository entry into parsed configuration file data.
    fn add_repo(&mut self, repo_entry: &RepoEntry) -> RicerResult<()> {
        debug!("Add repository '{}' too configuration file", &repo_entry.name);
        let (repo_name, repo_data) = repo_entry.to_toml();
        if let Some(repos) = self.doc.get_mut("repos") {
            trace!("The 'repos' section exists, add to it");
            let repos = repos.as_table_mut().ok_or(RicerError::ReposSectionNotTable)?;
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
    fn remove_repo(&mut self, repo_name: impl AsRef<str>) -> RicerResult<RepoEntry> {
        debug!("Remove repository '{}' from configuration file", repo_name.as_ref());
        let repos = self.doc.get_mut("repos").ok_or(RicerError::NoReposSection)?;
        let repos = repos.as_table_mut().ok_or(RicerError::ReposSectionNotTable)?;
        let (repo_key, repo_data) = repos
            .remove_entry(repo_name.as_ref())
            .ok_or(RicerError::NoRepoFound { repo_name: repo_name.as_ref().to_string() })?;
        Ok(RepoEntry::from((&repo_key, &repo_data)))
    }

    /// Rename repository entry in configuration file data.
    fn rename_repo(&mut self, from: impl AsRef<str>, to: impl AsRef<str>) -> RicerResult<()> {
        debug!("Rename repository '{}' to '{}' in configuration file", from.as_ref(), to.as_ref());
        let mut repo = self.remove_repo(from.as_ref())?;
        repo.name = to.as_ref().to_string();
        self.add_repo(&repo)?;
        Ok(())
    }

    /// Deserialize command hook envry from parsed configuration file data.
    fn get_cmd_hook(&self, cmd_name: impl AsRef<str>) -> RicerResult<CommandHookEntry> {
        let hooks = self.doc.get("hooks").ok_or(RicerError::NoHooksSection)?;
        let hooks = hooks.as_table().ok_or(RicerError::HooksSectionNotTable)?;
        let hook = hooks
            .get_key_value(cmd_name.as_ref())
            .ok_or(RicerError::NoHookFound { cmd_name: cmd_name.as_ref().to_string() })?;
        Ok(CommandHookEntry::from(hook))
    }
}
