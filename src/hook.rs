// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Manage user-defined hooks.
//!
//! The user can define hooks to execute in Ricer's configuration file. This
//! module provides a standard way to handle these hook definitions.
//!
//! The user can have Ricer prompt hooks for execution, always run a command
//! hook, or never run a command hook. Hook definitions can also change the
//! working directory for a given hook script by targeting a specific repository
//! definition.

use log::{debug, info, trace, warn};
use run_script::{run_script, ScriptOptions};
use std::env;
use std::path::PathBuf;

use crate::config::dir::ConfigDirManager;
use crate::config::file::hooks_section::HookEntry;
use crate::config::file::ConfigFileManager;
use crate::config::ConfigManager;
use crate::context::Context;
use crate::error::RicerResult;

/// Representation of a command hook manager.
pub trait CommandHookManager {
    /// Run command hook _before_ command itself.
    fn run_pre(&self) -> RicerResult<()>;

    /// Run command hook _after_ command itself.
    fn run_post(&self) -> RicerResult<()>;
}

/// Default command hook manager implementation.
pub struct DefaultCommandHookManager<'cfg, D, F>
where
    D: ConfigDirManager,
    F: ConfigFileManager,
{
    cfg_mgr: &'cfg ConfigManager<D, F>,
    ctx: &'cfg Context,
}

impl<'cfg, D, F> DefaultCommandHookManager<'cfg, D, F>
where
    D: ConfigDirManager,
    F: ConfigFileManager,
{
    /// Construct new default command hook manager.
    ///
    /// # Postconditions
    ///
    /// 1. Return valid default command hook manager.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use std::ffi::OsString;
    ///
    /// use ricer::cli::RicerCli;
    /// use ricer::config::dir::DefaultConfigDirManager;
    /// use ricer::config::file::DefaultConfigFileManager;
    /// use ricer::config::locator::{DefaultConfigDirLocator, DefaultXdgBaseDirSpec};
    /// use ricer::config::ConfigManager;
    /// use ricer::context::Context;
    /// use ricer::hook::DefaultCommandHookManager;
    ///
    /// let opts = RicerCli::parse_args([OsString::from("some"), OsString::from("command")]);
    /// let ctx = Context::from(opts);
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_mgr = DefaultConfigFileManager::new();
    /// let config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    /// let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(cfg_mgr: &'cfg ConfigManager<D, F>, ctx: &'cfg Context) -> Self {
        trace!("Construct new default command hook manager");
        Self { cfg_mgr, ctx }
    }

    /// Determine working directory of hook script.
    ///
    /// # Postconditions
    ///
    /// 1. Return [`ScriptOptions`] containing working directory.
    ///     - If repository entry sets target [`home`] to true, then working
    ///       directory is the user's home directory.
    ///     - If repository entry sets target [`home`] to false, then working
    ///       directory is the repository itself in
    ///       `$XDG_CONFIG_HOME/ricer/repos`.
    ///     - If repository [`target`] is `None` or [`home`] is `None`, then
    ///       there is no working directory set in [`ScriptOptions`].
    ///
    /// # Errors
    ///
    /// 1. Return [`RicerError::Unrecoverable`] if home directory cannot be
    ///    determined.
    /// 2. May fail if repository entry cannot be obtained due to no entry in
    ///    configuration file or `$XDG_CONFIG_HOME/ricer/repos`.
    ///
    /// # See
    ///
    /// - [`ConfigFileManager`]
    /// - [`ConfigDirManager`]
    ///
    /// [`ScriptOptions`]: https://docs.rs/run_script/latest/run_script/types/struct.ScriptOptions.html
    /// [`home`]: crate::config::file::repos_section::RepoTargetEntry::home
    /// [`target`]: crate::config::file::repos_section::RepoEntry::target
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    fn determine_work_dir(&self, repo: Option<&String>) -> RicerResult<ScriptOptions> {
        let mut opts = ScriptOptions::new();
        if let Some(repo) = repo {
            let (path, repo) = self.cfg_mgr.get_repo(repo)?;
            let home_dir = PathBuf::from(env::var("HOME")?);
            let work_dir =
                repo.target.as_ref().map(|target| match target.home.unwrap_or_default() {
                    true => {
                        debug!("Script targets home directory '{}'", home_dir.display());
                        home_dir
                    }
                    false => {
                        debug!("Script targets repository '{}'", path.display());
                        path
                    }
                });

            opts.working_directory = work_dir;
        } else {
            trace!("Script has not target working directory");
        }

        Ok(opts)
    }

    /// Run a given hook entry in command hook.
    ///
    /// # Postconditions
    ///
    /// 1. Return [`HookStatus::NoHook`] if there is not hook to run.
    /// 2. Return [`HookStatus::HookFailure`] if hook script failed to execute.
    /// 3. Return [`HookStatus::HookSuccess`] if hook succeeded in executing.
    ///
    /// # Errors
    ///
    /// 1. May fail if hook script cannot be read for whatever reason.
    /// 2. May fail if [`determine_work_dir`] fails.
    /// 3. May fail if [`run_script`] fails.
    ///
    /// # See
    ///
    /// - [`ConfigDirManager`]
    ///
    /// [`determine_work_dir`]: #method.determine_work_dir
    /// [`run_script`]: https://docs.rs/run_script/latest/run_script/macro.run_script.html
    fn run_hook_entry(&self, hook: &HookEntry, kind: &HookKind) -> RicerResult<HookStatus> {
        let script = match kind {
            HookKind::Pre => hook.pre.as_ref(),
            HookKind::Post => hook.post.as_ref(),
        };

        let script = match script {
            Some(script) => script,
            None => return Ok(HookStatus::NoHook),
        };

        let data = self.cfg_mgr.dir_manager().get_cmd_hook(script)?;
        let args = script.split_whitespace().skip(1).map(|s| s.to_string()).collect();
        let opts = self.determine_work_dir(hook.repo.as_ref())?;
        let (code, output, error) = run_script!(data, args, opts)?;
        if error.is_empty() {
            info!("Script '{}' (exit code: {}) stdout: {}", script, code, output);
        } else {
            warn!("Script '{}' failed (exit code: {}): {}", script, code, error);
            return Ok(HookStatus::HookFailure);
        }

        Ok(HookStatus::HookSuccess)
    }
}

impl<D, F> CommandHookManager for DefaultCommandHookManager<'_, D, F>
where
    D: ConfigDirManager,
    F: ConfigFileManager,
{
    /// Run pre hook entries in command hook.
    ///
    /// # Postconditions
    ///
    /// 1. Run all pre hook entries and report status on each one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use std::ffi::OsString;
    ///
    /// use ricer::cli::RicerCli;
    /// use ricer::config::dir::DefaultConfigDirManager;
    /// use ricer::config::file::DefaultConfigFileManager;
    /// use ricer::config::locator::{DefaultConfigDirLocator, DefaultXdgBaseDirSpec};
    /// use ricer::config::ConfigManager;
    /// use ricer::context::Context;
    /// use ricer::hook::{CommandHookManager, DefaultCommandHookManager};
    ///
    /// let opts = RicerCli::parse_args([OsString::from("some"), OsString::from("command")]);
    /// let ctx = Context::from(opts);
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_mgr = DefaultConfigFileManager::new();
    /// let config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    /// let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    /// hook_mgr.run_pre()?;
    /// # Ok(())
    /// # }
    /// ```
    fn run_pre(&self) -> RicerResult<()> {
        trace!("Run pre hooks");
        let cmd_hook = match self.cfg_mgr.file_manager().get_cmd_hook(self.ctx.to_string()) {
            Ok(cmd_hook) => cmd_hook,
            Err(err) => {
                debug!("{}", err);
                return Ok(());
            }
        };

        for hook in cmd_hook.hooks.iter() {
            match self.run_hook_entry(hook, &HookKind::Pre)? {
                HookStatus::HookSuccess => info!("Pre hook success"),
                HookStatus::HookFailure => warn!("Pre hook failure! Address reported issues"),
                HookStatus::NoHook => {
                    trace!("No pre hook to run");
                    continue;
                }
            };
        }

        Ok(())
    }

    /// Run post hook entries in command hook.
    ///
    /// # Postconditions
    ///
    /// 1. Run all post hook entries and report status on each one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use std::ffi::OsString;
    ///
    /// use ricer::cli::RicerCli;
    /// use ricer::config::dir::DefaultConfigDirManager;
    /// use ricer::config::file::DefaultConfigFileManager;
    /// use ricer::config::locator::{DefaultConfigDirLocator, DefaultXdgBaseDirSpec};
    /// use ricer::config::ConfigManager;
    /// use ricer::context::Context;
    /// use ricer::hook::{CommandHookManager, DefaultCommandHookManager};
    ///
    /// let opts = RicerCli::parse_args([OsString::from("some"), OsString::from("command")]);
    /// let ctx = Context::from(opts);
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = DefaultConfigDirLocator::new_locate(&xdg_spec)?;
    /// let cfg_dir_mgr = DefaultConfigDirManager::new(&locator);
    /// let cfg_file_mgr = DefaultConfigFileManager::new();
    /// let config = ConfigManager::new(cfg_dir_mgr, cfg_file_mgr);
    /// let hook_mgr = DefaultCommandHookManager::new(&config, &ctx);
    /// hook_mgr.run_post()?;
    /// # Ok(())
    /// # }
    /// ```
    fn run_post(&self) -> RicerResult<()> {
        trace!("Run post hooks");
        let cmd_hook = match self.cfg_mgr.file_manager().get_cmd_hook(self.ctx.to_string()) {
            Ok(cmd_hook) => cmd_hook,
            Err(err) => {
                debug!("{}", err);
                return Ok(());
            }
        };

        for hook in cmd_hook.hooks.iter() {
            match self.run_hook_entry(hook, &HookKind::Post)? {
                HookStatus::HookSuccess => info!("Post hook success"),
                HookStatus::HookFailure => warn!("Post hook failure! Address reported issues"),
                HookStatus::NoHook => {
                    trace!("No post hook to run");
                    continue;
                }
            };
        }

        Ok(())
    }
}

/// The hook kind to be executed.
enum HookKind {
    /// Run hook _before_ command.
    Pre,

    /// Run hook _after_ command.
    Post,
}

/// Status of hook after trying to execute it.
enum HookStatus {
    /// There was not hook to execute, i.e., `pre` or `post` were `None`.
    NoHook,

    /// Hook failed to execute for some reason, e.g., invalid script syntax,
    /// invalid permissions, etc.
    HookFailure,

    /// Hook succeeded to execute.
    HookSuccess,
}
