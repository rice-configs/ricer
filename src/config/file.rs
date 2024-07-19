// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Format of Ricer's configuration file.
//!
//! Ricer uses a special configuration file named `config.toml` in its base
//! directory in `$XDG_CONFIG_HOME/ricer`. This configuration file uses the
//! [TOML format][toml-spec] so the user can modify it by hand in case they do
//! not want to go through Ricer's command set for whatever reason.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Currently Ricer's configuration file is comprised of two major tables:
/// `repos` and `hooks`. The `repos` table houses configuration information for
/// repositories Ricer is tracking. The `hooks` table is where the user can
/// define their custom command hooks.
///
/// [toml-spec]: https://toml.io/en/v1.0.0
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfigFile {
    /// Current Git repositories Ricer is tracking.
    pub repos: Option<HashMap<String, ReposTable>>,

    /// Current command hooks to execute.
    pub hooks: Option<HooksTable>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReposTable {
    /// Make repository use the user's home directory as the working tree.
    pub target_home: bool,

    /// Branch to use for all commands.
    pub main_branch: String,

    /// Remote to use for all commands.
    pub main_remote: String,
}

/// Command hooks.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HooksTable {
    pub commit: Option<Vec<HookConfig>>,
    pub push: Option<Vec<HookConfig>>,
    pub pull: Option<Vec<HookConfig>>,
    pub init: Option<Vec<HookConfig>>,
    pub clone: Option<Vec<HookConfig>>,
    pub delete: Option<Vec<HookConfig>>,
    pub rename: Option<Vec<HookConfig>>,
    pub status: Option<Vec<HookConfig>>,
    pub list: Option<Vec<HookConfig>>,
    pub enter: Option<Vec<HookConfig>>,
}

/// Hook configuration.
///
/// Hooks are only found and executed from the `hooks` directory in Ricer's
/// base directory. The user only needs to specify the hook by its filename.
/// So if a hook script `$XDG_CONFIG_HOME/ricer/hooks/hook.sh` exists, then the
/// user can specify it as just `hook.sh`.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HookConfig {
    /// Execute hook _before_ running the Ricer command.
    pub pre: Option<String>,

    /// Execute hook _after_ running the Ricer command.
    pub post: Option<String>,

    /// Execute the hook _only_ for a target repository.
    pub repo: Option<String>,
}
