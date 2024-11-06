// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::config;
use crate::wizard;

use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CommandHookManagerError {
    #[error("Failed to load command hook configuration file")]
    LoadConfig { source: config::ConfigManagerError },

    #[error("Failed to get command hook data")]
    GetCmdHook { source: config::TomlError },

    #[error("Failed to read hook '{path}'")]
    HookRead { source: io::Error, path: PathBuf },

    #[error("Failed to run hook")]
    RunHook { source: run_script::ScriptError },

    #[error("Failed to run pager")]
    HookPager { source: wizard::HookPagerError },
}

impl From<config::ConfigManagerError> for CommandHookManagerError {
    fn from(err: config::ConfigManagerError) -> Self {
        CommandHookManagerError::LoadConfig { source: err }
    }
}

impl From<config::TomlError> for CommandHookManagerError {
    fn from(err: config::TomlError) -> Self {
        CommandHookManagerError::GetCmdHook { source: err }
    }
}

impl From<run_script::ScriptError> for CommandHookManagerError {
    fn from(err: run_script::ScriptError) -> Self {
        CommandHookManagerError::RunHook { source: err }
    }
}

impl From<wizard::HookPagerError> for CommandHookManagerError {
    fn from(err: wizard::HookPagerError) -> Self {
        CommandHookManagerError::HookPager { source: err }
    }
}
