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

use log::trace;

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

impl HookEntry {
    /// Build a new hook entry definition.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return hook entry builder instance.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    pub fn builder() -> HookEntryBuilder {
        HookEntryBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct HookEntryBuilder {
    pre: Option<String>,
    post: Option<String>,
    repo: Option<String>,
}

impl HookEntryBuilder {
    /// Construct new hook entry builder.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return new instance of hook entry builder.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set pre-script to run _before_ target command.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`pre`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`pre`]: #member.pre
    pub fn pre(mut self, script_name: impl AsRef<str>) -> Self {
        self.pre = Some(script_name.as_ref().to_string());
        self
    }

    /// Set post-script to run _before_ target command.
    ///
    /// # postconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`post`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`post`]: #member.post
    pub fn post(mut self, script_name: impl AsRef<str>) -> Self {
        self.post = Some(script_name.as_ref().to_string());
        self
    }

    /// Set repo-script to run _before_ target command.
    ///
    /// # repoconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`repo`] field.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// [`repo`]: #member.repo
    pub fn repo(mut self, repo_name: impl AsRef<str>) -> Self {
        self.repo = Some(repo_name.as_ref().to_string());
        self
    }

    /// Build new [`HookEntry`].
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return new instance of [`HookEntry`].
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::file::hooks_section::HookEntry;
    ///
    /// let hook_entry = HookEntryBuilder::new()
    ///     .pre("hook.sh")
    ///     .post("hook.sh")
    ///     .repo("vim")
    ///     .build();
    /// println!("{:#?}", hook_entry);
    /// ```
    pub fn build(self) -> HookEntry {
        trace!("Build new hook entry definition");
        HookEntry { pre: self.pre, post: self.post, repo: self.repo }
    }
}
