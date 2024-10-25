// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

/// Command hook settings.
///
/// An intermediary structure to help deserialize and serialize command hook
/// from Ricer's command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandHook {
    /// Name of command to bind hook definitions too.
    pub cmd: String,

    /// Array of hook definitions to execute.
    pub hooks: Vec<Hook>,
}
