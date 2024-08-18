// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Error system.
//!
//! This module contains all error types that a given module of the Ricer code
//! base may produce. Some of these errors are meant to be recoverable, others
//! are not. It all depends upon the API of a given module.

use anyhow::anyhow;
use std::path::PathBuf;

/// Specify result type that uses [`RicerError`] for `Err`.
pub type RicerResult<T> = Result<T, RicerError>;

/// All possible error types in Ricer's internal API.
///
/// Do note that the documented description of each error type only provides
/// the _expectation_ on recoverability. Please refer to the module API that
/// uses these error types for what is really going to happen if Ricer
/// encounters these errors.
#[derive(Debug, thiserror::Error)]
pub enum RicerError {
    /// Configuration directory located, but does not exist.
    ///
    /// Recoverable as the location is known, so a new configuration directory
    /// can simply be created at that location.
    #[error("Configuration directory located at expected path, but does not exist")]
    NoConfigDir(#[source] anyhow::Error),

    /// Configuration file does not exist at expected path.
    ///
    /// Recoverable since the path is known, and can just be created at any
    /// time.
    #[error("Configuration file does not exist at '{path}'")]
    NoConfigFile { path: PathBuf },

    /// Git repository does not exist at expected path.
    ///
    /// Recoverable since the path is known, and can just be created at any
    /// time.
    #[error("Git repository does not exist at '{path}'")]
    NoGitRepo { path: PathBuf },

    /// Hook script does not exist at expected path.
    ///
    /// Recoverable since the path is known, and can just be created at any
    /// time.
    #[error("Hook script does not exist at '{path}'")]
    NoHookScript { path: PathBuf },

    /// Ignore file does not exist at expected path.
    ///
    /// Recoverable since the path is known, and can just be created at any
    /// time.
    #[error("Ignore file does not exist at '{path}'")]
    NoIgnoreFile { path: PathBuf },

    /// Configuration file does not have a `repos` section.
    ///
    /// Recoverability depends on context. If user is adding a new repository to
    /// the configuration file, then just create the `repos` section. However,
    /// if the user is trying to obtain repository data from the configuration
    /// file with the `repos` section not existing, then this error is deemed
    /// unrecoverable as the user _needs to add_ a repository definition, or
    /// create the `repos` section themselves.
    #[error("Configuration file does not have a 'repos' section")]
    NoReposSection,

    /// Configuration file does not define `repos` section as a table.
    ///
    /// The user can define `repos` as a key/value pair, which is not what Ricer
    /// expects. This one is currently considered unrecoverable, requiring the
    /// user to remove their key/value pair definition of `repos`.
    #[error("Configuration file does not define 'repos' section as a table")]
    ReposSectionNotTable,

    /// Configuration file does not contain repository definition in `repos` section.
    ///
    /// This is currently considered unrecoverable. The user is expected to
    /// _add_ a repository definition in order to later retieve it.
    #[error("Configuration file does not have repository '{repo_name}' in 'repos' section")]
    NoRepoFound { repo_name: String },

    /// Configuration file does not have a `hooks` section.
    ///
    /// Recoverability depends on context. If the user is adding a command hook
    /// definition, then just create the `hooks` section. If the user is trying
    /// to retrieve a command hook definition without a `hooks` section, then
    /// this error is unrecoverable, because the user first needs to _add_ a
    /// command _hook_ or `hooks` section to avoid this error.
    #[error("Configuration file does not have a 'hooks' section")]
    NoHooksSection,

    /// Configuration file does not define `hooks` section as a table.
    ///
    /// The user can define `hooks` as a key/value pair, which is not what Ricer
    /// expects. This one is currently considered unrecoverable, requiring the
    /// user to remove their key/value pair definition of `hooks`.
    #[error("Configuration file does not define 'hooks' section as a table")]
    HooksSectionNotTable,

    /// Configuration file does not contain command hook definition in `hooks`
    /// section.
    ///
    /// This is currently considered unrecoverable. The user is expected to
    /// _add_ a command hook definition in order to later retieve it.
    #[error("Configuration file does not have command hook '{cmd_name}' in 'hooks' section")]
    NoHookFound { cmd_name: String },

    /// External error is not recoverable.
    ///
    /// Mainly used to convert external error types into a valid [`RicerError`]
    /// variant.
    #[error(transparent)]
    Unrecoverable(#[from] anyhow::Error),
}

impl From<std::io::Error> for RicerError {
    fn from(err: std::io::Error) -> RicerError {
        RicerError::Unrecoverable(anyhow!("{}", err))
    }
}

impl From<toml_edit::TomlError> for RicerError {
    fn from(err: toml_edit::TomlError) -> RicerError {
        RicerError::Unrecoverable(anyhow!("{}", err))
    }
}
