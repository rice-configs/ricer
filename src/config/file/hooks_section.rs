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
//! be executed _after_ the target command.
//!
//! The `repo` field is really unique. It determines the working directory of
//! the current hook script. If no `repo` field is present in a hook definition,
//! then no working directory is set. If the `repo` field is present, then one
//! of three things can happen depending on how the repository sets up the
//! optional [`target`] field:
//!
//! 1. If [`target`] is `None` or [`target.home`] is `None`, then do not set
//!    the working directory.
//! 2. If [`target.home`] is true, then set the working directory to the user's
//!    home directory.
//! 3. If [`target.home`] is false, then set the working directory to the
//!    repository itself in `$XDG_CONFIG_HOME/ricer/repos`.
//!
//! [`target`]: crate::config::file::repos_section::RepoEntry::target
//! [`target.home`]: crate::config::file::repos_section::RepoTargetEntry::home

use log::trace;
use toml_edit::visit::{visit_inline_table, Visit};
use toml_edit::{InlineTable, Item, Key};

/// Command hook entry definition implementation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandHookEntry {
    /// Name of command to bind hook definition entries too.
    pub cmd: String,

    /// Array of hook entries to execute.
    pub hooks: Vec<HookEntry>,
}

impl CommandHookEntry {
    /// Construct new command hook entry definition.
    ///
    /// # Postconditions
    ///
    /// 1. Return valid instance of command hook entry handler.
    ///
    /// # Examples
    ///
    /// ```
    /// use ricer::config::file::hooks_section::CommandHookEntry;
    ///
    /// let cmd_hook = CommandHookEntry::new("commit");
    /// ```
    pub fn new(cmd_name: impl AsRef<str>) -> Self {
        Self { cmd: cmd_name.as_ref().to_string(), hooks: Default::default() }
    }

    /// Add hook entry into command hook definition.
    ///
    /// # Postconditions
    ///
    /// 1. Add hook entry to [`hooks`] field.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::file::hooks_section::{CommandHookEntry, HookEntry};
    ///
    /// let mut cmd_hook = CommandHookEntry::new("commit");
    /// let hook_entry = HookEntry::builder()
    ///     .pre("hook.sh")
    ///     .post("hook.sh")
    ///     .repo("vim")
    ///     .build();
    /// cmd_hook.add_hook(hook_entry);
    /// ```
    ///
    /// [`hooks`]: #member.hooks
    pub fn add_hook(&mut self, hook: HookEntry) {
        self.hooks.push(hook);
    }
}

impl<'toml> From<(&'toml Key, &'toml Item)> for CommandHookEntry {
    fn from(toml_entry: (&'toml Key, &'toml Item)) -> Self {
        let (key, value) = toml_entry;
        let mut entry = CommandHookEntry::new(key.get());
        entry.visit_item(value);
        entry
    }
}

impl<'toml> Visit<'toml> for CommandHookEntry {
    fn visit_inline_table(&mut self, node: &'toml InlineTable) {
        let pre = if let Some(pre) = node.get("pre") { pre.as_str() } else { None };
        let post = if let Some(post) = node.get("post") { post.as_str() } else { None };
        let repo = if let Some(repo) = node.get("repo") { repo.as_str() } else { None };

        let hook_entry = HookEntry::builder();
        let hook_entry = if let Some(pre) = pre { hook_entry.pre(pre) } else { hook_entry };
        let hook_entry = if let Some(post) = post { hook_entry.post(post) } else { hook_entry };
        let hook_entry = if let Some(repo) = repo { hook_entry.repo(repo) } else { hook_entry };
        self.add_hook(hook_entry.build());

        visit_inline_table(self, node);
    }
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
    /// # Postconditions
    ///
    /// 1. Return hook entry builder instance.
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
    /// # Postconditions
    ///
    /// 1. Return new instance of hook entry builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set pre-script to run _before_ target command.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`pre`] field.
    ///
    /// [`pre`]: #member.pre
    pub fn pre(mut self, script_name: impl Into<String>) -> Self {
        self.pre = Some(script_name.into());
        self
    }

    /// Set post-script to run _before_ target command.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`post`] field.
    ///
    /// [`post`]: #member.post
    pub fn post(mut self, script_name: impl Into<String>) -> Self {
        self.post = Some(script_name.into());
        self
    }

    /// Set repo-script to run _before_ target command.
    ///
    /// # Postconditions
    ///
    /// 1. Set [`repo`] field.
    ///
    /// [`repo`]: #member.repo
    pub fn repo(mut self, repo_name: impl Into<String>) -> Self {
        self.repo = Some(repo_name.into());
        self
    }

    /// Build new [`HookEntry`].
    ///
    /// # Postconditions
    ///
    /// 1. Return new instance of [`HookEntry`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::file::hooks_section::HookEntryBuilder;
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
