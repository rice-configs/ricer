// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use log::trace;
use std::path::PathBuf;
use toml_edit::visit::{visit_inline_table, Visit};
use toml_edit::{Array, InlineTable, Item, Key, Value};

/// Command hook entry definition.
///
/// An intermediary structure to help deserialize command hook entries from
/// Ricer's command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CmdHookEntry {
    /// Name of command to bind hook definition entries too.
    pub cmd: String,

    /// Array of hook entries to execute.
    pub hooks: Vec<HookEntry>,
}

impl CmdHookEntry {
    /// Construct new command hook entry definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::CmdHookEntry;
    ///
    /// let hook = CmdHookEntry::new("commit");
    /// ```
    pub fn new(cmd: impl Into<String>) -> Self {
        trace!("Construct new command hook entry definition");
        Self { cmd: cmd.into(), hooks: Default::default() }
    }

    /// Add hook entry into command hook definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::{CmdHookEntry, HookEntry};
    ///
    /// let mut cmd_hook = CmdHookEntry::new("commit");
    /// let hook = HookEntry::builder()
    ///     .pre("hook.sh")
    ///     .post("hook.sh")
    ///     .workdir("/path/to/work/dir")
    ///     .build();
    /// cmd_hook.add_hook(hook);
    /// ```
    pub fn add_hook(&mut self, hook: HookEntry) {
        self.hooks.push(hook);
    }

    /// Serialize command hook entry definition into a TOML item.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use indoc::indoc;
    /// use pretty_assertions::assert_eq;
    /// use ricer::config::{CmdHookEntry, HookEntry};
    /// use toml_edit::DocumentMut;
    ///
    /// let mut cmd_hook = CmdHookEntry::new("commit");
    /// cmd_hook.add_hook(HookEntry::builder().pre("hook.sh").build());
    /// cmd_hook.add_hook(HookEntry::builder().post("hook.sh").workdir("/some/path").build());
    /// let (key, value) = cmd_hook.to_toml();
    ///
    /// let mut toml_doc: DocumentMut = "[hooks]".parse()?;
    /// let hooks_table = toml_doc.get_mut("hooks").unwrap();
    /// let hooks_table = hooks_table.as_table_mut().unwrap();
    /// hooks_table.insert(&key, value);
    /// hooks_table.set_implicit(true);
    /// let expect = indoc! {r#"
    ///     [hooks]
    ///     commit = [
    ///         { pre = "hook.sh" },
    ///         { post = "hook.sh", workdir = "/some/path" }
    ///     ]
    /// "#};
    /// let result = toml_doc.to_string();
    /// assert_eq!(expect, result);
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_toml(&self) -> (Key, Item) {
        let mut tables = Array::new();
        let mut iter = self.hooks.iter().enumerate().peekable();
        while let Some((_, hook)) = iter.next() {
            let mut inline = InlineTable::new();
            let decor = inline.decor_mut();
            decor.set_prefix("\n    ");

            if iter.peek().is_none() {
                decor.set_suffix("\n");
            }

            if let Some(pre) = &hook.pre {
                inline.insert("pre", Value::from(pre));
            }

            if let Some(post) = &hook.post {
                inline.insert("post", Value::from(post));
            }

            if let Some(workdir) = &hook.workdir {
                inline.insert("workdir", Value::from(String::from(workdir.to_string_lossy())));
            }

            tables.push_formatted(Value::from(inline));
        }

        let key = Key::new(&self.cmd);
        let value = Item::Value(Value::from(tables));
        (key, value)
    }
}

impl<'toml> From<(&'toml Key, &'toml Item)> for CmdHookEntry {
    fn from(toml_entry: (&'toml Key, &'toml Item)) -> Self {
        let (key, value) = toml_entry;
        let mut hook = CmdHookEntry::new(key.get());
        hook.visit_item(value);
        hook
    }
}

impl<'toml> Visit<'toml> for CmdHookEntry {
    fn visit_inline_table(&mut self, node: &'toml InlineTable) {
        let pre = if let Some(pre) = node.get("pre") { pre.as_str() } else { None };
        let post = if let Some(post) = node.get("post") { post.as_str() } else { None };
        let workdir = if let Some(workdir) = node.get("workdir") { workdir.as_str() } else { None };

        let hook = HookEntry::builder();
        let hook = if let Some(pre) = pre { hook.pre(pre) } else { hook };
        let hook = if let Some(post) = post { hook.post(post) } else { hook };
        let hook = if let Some(workdir) = workdir { hook.workdir(workdir) } else { hook };
        self.add_hook(hook.build());

        visit_inline_table(self, node);
    }
}

/// Hook entry definition.
///
/// An intermediary structure to help deserialize hook entries for command hook
/// definitions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HookEntry {
    /// Execute hook script _before_ command itself.
    pub pre: Option<String>,

    /// Execute hook script _after_ command itself.
    pub post: Option<String>,

    /// Set working directory of hook script.
    pub workdir: Option<PathBuf>,
}

impl HookEntry {
    /// Build new hook entry definition.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookEntry;
    ///
    /// let hook = HookEntry::builder()
    ///     .pre("hook.sh")
    ///     .pre("hook.sh")
    ///     .workdir("/path/to/work/dir/")
    ///     .build();
    /// ```
    pub fn builder() -> HookEntryBuilder {
        HookEntryBuilder::new()
    }
}

/// Builder for [`HookEntry`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HookEntryBuilder {
    pre: Option<String>,
    post: Option<String>,
    workdir: Option<PathBuf>,
}

impl HookEntryBuilder {
    /// Construct new hook entry builder.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookEntryBuilder;
    ///
    /// let builder = HookEntryBuilder::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set hook to run _before_ command.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookEntryBuilder;
    ///
    /// let builder = HookEntryBuilder::new().pre("hook.sh");
    /// ```
    pub fn pre(mut self, script: impl Into<String>) -> Self {
        self.pre = Some(script.into());
        self
    }

    /// Set hook to run _after_ command.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookEntryBuilder;
    ///
    /// let builder = HookEntryBuilder::new().post("hook.sh");
    /// ```
    pub fn post(mut self, script: impl Into<String>) -> Self {
        self.post = Some(script.into());
        self
    }

    /// Set working directory for script.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookEntryBuilder;
    ///
    /// let builder = HookEntryBuilder::new().workdir("/path/to/work/dir");
    /// ```
    pub fn workdir(mut self, path: impl Into<PathBuf>) -> Self {
        self.workdir = Some(path.into());
        self
    }

    /// Build new [`HookEntry`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ricer::config::HookEntryBuilder;
    ///
    /// let hook = HookEntryBuilder::new()
    ///     .pre("hook.sh")
    ///     .pre("hook.sh")
    ///     .workdir("/path/to/work/dir/")
    ///     .build();
    /// ```
    pub fn build(self) -> HookEntry {
        trace!("Build new hook entry definition");
        HookEntry { pre: self.pre, post: self.post, workdir: self.workdir }
    }
}
