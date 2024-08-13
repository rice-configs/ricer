// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Manage command hook definitions.
//!
//! A command hook definition in Ricer is stored in the `hooks` section of the
//! configuration file. Currently, command hooks follow this general formatting:
//!
//! ```markdown
//! [hooks]
//! cmd = [
//!     { pre = "hook.sh", post = "hook.sh", repo = "vim" }
//!     ...
//! ]
//! ```
//!
//! The `cmd` field is the name of the Ricer command to bind the hook
//! definitions too. The `pre` field is the hook script that will be executed
//! _before_ the target command. The `post` field is the hook script that will
//! be executed _after_ the target command. The `repo` command is the target
//! repository to execute the current hook entry on only.

/// Command hook entry definition implementation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandHookEntry {
    /// Name of command to bind hook definition entries too.
    pub cmd: String,

    /// Array of hook entries to execute.
    pub hooks: Vec<HookEntry>
}

/// Hook entry definition to be added into array of command hook definition.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HookEntry {
    /// Execute hook script _before_ command itself.
    pub pre: Option<String>,

    /// Execute hook script _after_ command itself.
    pub post: Option<String>,

    /// Execute hook script _only_ for a target repository definition.
    pub repo: Option<String>,
}
