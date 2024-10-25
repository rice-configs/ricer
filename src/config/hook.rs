// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use toml_edit::{Key, Item, InlineTable};
use toml_edit::visit::{Visit, visit_inline_table};

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

impl CommandHook {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self { cmd: cmd.into(), hooks: Default::default() }
    }

    pub fn add_hook(&mut self, hook: Hook) {
        self.hooks.push(hook);
    }
}

fn from_toml<'toml>(entry: (&'toml Key, &'toml Item)) -> CommandHook {
    let (key, value) = entry;
    let mut cmd_hook = CommandHook::new(key.get());
    cmd_hook.visit_item(value);
    cmd_hook
}

impl<'toml> From<(&'toml Key, &'toml Item)> for CommandHook {
    fn from(entry: (&'toml Key, &'toml Item)) -> Self {
        from_toml(entry)
    }
}

impl From<(Key, Item)> for CommandHook {
    fn from(entry: (Key, Item)) -> Self {
        let (key, value) = entry;
        from_toml((&key, &value))
    }
}

impl<'toml> Visit<'toml> for CommandHook {
    fn visit_inline_table(&mut self, node: &'toml InlineTable) {
        let pre = if let Some(pre) = node.get("pre") { pre.as_str() } else { None };
        let post = if let Some(post) = node.get("post") { post.as_str() } else { None };
        let workdir = if let Some(workdir) = node.get("workdir") { workdir.as_str() } else { None };

        let hook = Hook::new();
        let hook = if let Some(pre) = pre { hook.pre(pre) } else { hook };
        let hook = if let Some(post) = post { hook.post(post) } else { hook };
        let hook = if let Some(workdir) = workdir { hook.workdir(workdir) } else { hook };
        self.add_hook(hook);

        visit_inline_table(self, node);
    }
}

/// Hook definition settings.
///
/// An intermediary structure to help deserialize and serialize hook entries
/// for command hook settings in command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Hook {
    /// Execute hook script _before_ command itself.
    pub pre: Option<String>,

    /// Execute hook script _after_ command itself.
    pub post: Option<String>,

    /// Set working directory of hook script.
    pub workdir: Option<PathBuf>,
}

impl Hook {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pre(mut self, script: impl Into<String>) -> Self {
        self.pre = Some(script.into());
        self
    }

    pub fn post(mut self, script: impl Into<String>) -> Self {
        self.post = Some(script.into());
        self
    }

    pub fn workdir(mut self, path: impl Into<PathBuf>) -> Self {
        self.workdir = Some(path.into());
        self
    }
}
