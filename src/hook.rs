// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

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

pub trait CommandHookManager {
    fn run_pre(&self) -> RicerResult<()>;
    fn run_post(&self) -> RicerResult<()>;
}

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
    pub fn new(cfg_mgr: &'cfg ConfigManager<D, F>, ctx: &'cfg Context) -> Self {
        trace!("Construct new command hook manager");
        Self { cfg_mgr, ctx }
    }

    fn setup_script_options(&self, repo: Option<&String>) -> RicerResult<ScriptOptions> {
        let mut opts = ScriptOptions::new();
        if let Some(repo) = &repo {
            let (path, repo) = self.cfg_mgr.get_repo(repo)?;
            let home_dir = PathBuf::from(env::var("HOME")?);
            let work_dir = repo.target.as_ref().map(|target| match target.home.unwrap_or_default() {
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

    fn run_hook_entry<'run>(
        &self,
        hook: &'run HookEntry,
        kind: &'run HookKind,
    ) -> RicerResult<HookStatus> {
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
        let opts = self.setup_script_options(hook.repo.as_ref())?;
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
    fn run_pre(&self) -> RicerResult<()> {
        let cmd_hook =
            self.cfg_mgr.file_manager().get_cmd_hook(self.ctx.to_string()).ok().unwrap_or_default();
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

    fn run_post(&self) -> RicerResult<()> {
        let cmd_hook =
            self.cfg_mgr.file_manager().get_cmd_hook(self.ctx.to_string()).ok().unwrap_or_default();
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

enum HookKind {
    Pre,
    Post,
}

enum HookStatus {
    NoHook,
    HookFailure,
    HookSuccess,
}
