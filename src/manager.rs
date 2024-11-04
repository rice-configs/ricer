// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Data manager implementations.
//!
//! This module is responsible for providing the logic needed for Ricer to
//! manage configuration, hook, and repository data provided by the user.

mod error;
mod toml;

#[doc(inline)]
pub use error::*;
pub use toml::*;

use crate::context::{Context, HookAction};
use crate::data_xchg::Toml;
use crate::locate::Locator;
use crate::wizard::HookPager;

use log::{debug, info};
use mkdirp::mkdirp;
use run_script::{run_script, ScriptOptions};
use std::fmt;
use std::fs::{read_to_string, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

/// Configuration file construct.
///
/// Manage configuration file data by selecting which configuration
/// startegy to use, i.e., which configuration category to handle.
/// Expected section to serialize and deserialize will depend on the
/// configuration strategy selected by caller.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
#[derive(Clone, Debug)]
pub struct ConfigManager<'cfg, L, T>
where
    L: Locator,
    T: TomlManager,
{
    doc: Toml,
    config: T,
    locator: &'cfg L,
}

impl<'cfg, L, T> ConfigManager<'cfg, L, T>
where
    L: Locator,
    T: TomlManager,
{
    /// Load new configuration manager.
    ///
    /// If path to configuration file does not exist, then it will be created at
    /// target location. Otherwise, configuration file will be read and parsed
    /// like normal.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileRead`] if target configuration file
    ///    could not be read.
    /// 1. Return [`ConfigManagerError::Toml`] if target configuration file
    ///    could not be parsed into TOML format.
    pub fn load(config: T, locator: &'cfg L) -> Result<Self, ConfigManagerError> {
        let path = config.location(locator);
        debug!("Load new configuration manager from '{}'", path.display());
        let root = path.parent().unwrap();
        mkdirp(root)
            .map_err(|err| ConfigManagerError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|err| ConfigManagerError::FileOpen { source: err, path: path.into() })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| ConfigManagerError::FileRead { source: err, path: path.into() })?;
        let doc: Toml = buffer
            .parse()
            .map_err(|err| ConfigManagerError::Toml { source: err, path: path.into() })?;

        Ok(Self { doc, config, locator })
    }

    /// Save configuration data at expected location.
    ///
    /// If expected configuration file does not exist at location, then it will
    /// be created and written into automatically.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigManagerError::FileWrite`] if target configuration file
    ///    cannot be written into.
    pub fn save(&mut self) -> Result<(), ConfigManagerError> {
        debug!("Save configuration manager data to '{}'", self.as_path().display());
        let root = self.as_path().parent().unwrap();
        mkdirp(root)
            .map_err(|err| ConfigManagerError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(self.as_path())
            .map_err(|err| ConfigManagerError::FileOpen {
                source: err,
                path: self.as_path().into(),
            })?;
        let buffer = self.doc.to_string();
        file.write_all(buffer.as_bytes()).map_err(|err| ConfigManagerError::FileWrite {
            source: err,
            path: self.as_path().into(),
        })?;

        Ok(())
    }

    /// Get configuration entry in deserialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be deserialized.
    pub fn get(&self, key: impl AsRef<str>) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .get(&self.doc, key.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    /// Add new configuration entry in serialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be serialized.
    pub fn add(&mut self, entry: T::Entry) -> Result<Option<T::Entry>, ConfigManagerError> {
        self.config
            .add(&mut self.doc, entry)
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    /// Rename configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be renamed.
    pub fn rename(
        &mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .rename(&mut self.doc, from.as_ref(), to.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    /// Remove configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigManagerError::Toml`] if entry cannot be removed.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Result<T::Entry, ConfigManagerError> {
        self.config
            .remove(&mut self.doc, key.as_ref())
            .map_err(|err| ConfigManagerError::Toml { source: err, path: self.as_path().into() })
    }

    pub fn as_path(&self) -> &Path {
        self.config.location(self.locator)
    }
}

impl<'cfg, L, T> fmt::Display for ConfigManager<'cfg, L, T>
where
    L: Locator,
    T: TomlManager,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

/// Command hook execution management.
///
/// Executes user-defined hooks according to specified hook actions selected by
/// the user through [`Context`]. Hooks can be defined as _pre_ or _post_, i.e.,
/// run _before_ a command, or run _after_ a command. Hooks utilize scripts that
/// must be defined in the special `hooks/` directory at the top-level of
/// Ricer's configuration directory.
///
/// User can set hook actions to _never_, _always_, and _prompt_. Never action
/// means that no hook can be executed no questions asked. Always action means
/// that hooks are executed no questions asked. Finally, prompt action will page
/// the contents of a hook script for the user to review, and prompt them about
/// whether or not they want to execute it.
pub struct CommandHookManager<'cfg, L>
where
    L: Locator,
{
    context: &'cfg Context,
    locator: &'cfg L,
    config: ConfigManager<'cfg, L, CommandHookData>,
    pager: HookPager,
}

impl<'cfg, L> CommandHookManager<'cfg, L>
where
    L: Locator,
{
    pub fn load(context: &'cfg Context, locator: &'cfg L) -> Result<Self, CommandHookManagerError> {
        let config = ConfigManager::load(CommandHookData, locator)?;
        Ok(Self { context, locator, config, pager: Default::default() })
    }

    pub fn set_pager(&mut self, pager: HookPager) {
        self.pager = pager;
    }

    /// Run user-defined hooks.
    ///
    /// Run specific hook kind for given command that was selected through
    /// [`Context`].
    ///
    /// # Errors
    ///
    /// 1. Return [`CommandHookManagerError::GetCmdHook`] if current command
    ///    hook definition cannot be obtained through hook configuration file.
    /// 2. Return [`CommandHookManagerError::HookRead`] if hook script cannot
    ///    be read from `hooks/` directory.
    /// 3. Return [`CommandHookManagerError::RunHook`] if hook script cannot
    ///    be executed for whatever reason.
    /// 4. Return [`CommandHookManagerError::HookPager`] if pager cannot page
    ///    hook script and prompt user.
    pub fn run_hooks(&self, hook_kind: HookKind) -> Result<(), CommandHookManagerError> {
        // INVARIANT: Git command shortcut cannot execute hooks.
        if matches!(self.context, Context::Git(..)) {
            return Ok(());
        }

        let action = self.get_hook_action();
        if action == &HookAction::Never {
            return Ok(());
        }

        let cmd_hook = self.config.get(self.context.to_string())?;
        for hook in cmd_hook.hooks {
            let hook_name = match hook_kind {
                HookKind::Pre => hook.pre.as_ref(),
                HookKind::Post => hook.post.as_ref(),
            };
            let hook_name = match hook_name {
                Some(name) => name,
                None => continue, // Skip this iteration if no hook name is found.
            };

            let hook_path = self.locator.hooks_dir().join(hook_name);
            let hook_data = read_to_string(&hook_path).map_err(|err| {
                CommandHookManagerError::HookRead { source: err, path: hook_path.clone() }
            })?;

            if action == &HookAction::Prompt {
                let workdir = hook.workdir.as_deref().unwrap_or(self.locator.hooks_dir());
                self.pager.page_and_prompt(hook_path.as_path(), workdir, &hook_data)?;
                if !self.pager.choice() {
                    continue; // Skip this iteration if user denied hook script.
                }
            }

            let mut hook_opts = ScriptOptions::new();
            hook_opts.working_directory = hook.workdir;
            let (code, out, err) = run_script!(hook_data, hook_opts)?;
            info!("({code}) {}\nstdout: {out}\nstderr: {err}", hook_path.display());
        }

        Ok(())
    }

    fn get_hook_action(&self) -> &HookAction {
        match self.context {
            Context::Bootstrap(ctx) => &ctx.shared.run_hook,
            Context::Clone(ctx) => &ctx.shared.run_hook,
            Context::Commit(ctx) => &ctx.shared.run_hook,
            Context::Delete(ctx) => &ctx.shared.run_hook,
            Context::Enter(ctx) => &ctx.shared.run_hook,
            Context::Init(ctx) => &ctx.shared.run_hook,
            Context::List(ctx) => &ctx.shared.run_hook,
            Context::Pull(ctx) => &ctx.shared.run_hook,
            Context::Push(ctx) => &ctx.shared.run_hook,
            Context::Rename(ctx) => &ctx.shared.run_hook,
            Context::Status(ctx) => &ctx.shared.run_hook,

            // INVARIANT: Git command shortcut cannot use hooks.
            Context::Git(_) => {
                unreachable!("This should not happen. Git shortcut cannot use hooks")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookKind {
    /// Execute hooks _before_ command.
    Pre,

    /// Execute hooks _after_ command.
    Post,
}
