// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::data_xchg;
use crate::wizard;

use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ConfigManagerError {
    #[error("Failed to make parent directory '{path}'")]
    MakeDirP { source: io::Error, path: PathBuf },

    #[error("Failed to open '{path}'")]
    FileOpen { source: io::Error, path: PathBuf },

    #[error("Failed to read '{path}'")]
    FileRead { source: io::Error, path: PathBuf },

    #[error("Failed to write '{path}'")]
    FileWrite { source: io::Error, path: PathBuf },

    #[error("Failed to parse '{path}'")]
    Toml { source: data_xchg::TomlError, path: PathBuf },
}

#[derive(Debug, thiserror::Error)]
pub enum CommandHookManagerError {
    #[error("Failed to load command hook configuration file")]
    LoadConfig { source: ConfigManagerError },

    #[error("Failed to get command hook data")]
    GetCmdHook { source: data_xchg::TomlError },

    #[error("Failed to read hook '{path}'")]
    HookRead { source: io::Error, path: PathBuf },

    #[error("Failed to run hook")]
    RunHook { source: run_script::ScriptError },

    #[error("Failed to run pager")]
    HookPager { source: wizard::HookPagerError },
}

impl From<ConfigManagerError> for CommandHookManagerError {
    fn from(err: ConfigManagerError) -> Self {
        CommandHookManagerError::LoadConfig { source: err }
    }
}

impl From<data_xchg::TomlError> for CommandHookManagerError {
    fn from(err: data_xchg::TomlError) -> Self {
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
