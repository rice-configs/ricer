// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use log::{error, info, trace};
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
            let (path, repo_entry) = self.cfg_mgr.get_repo(repo)?;
            let home_dir = PathBuf::from(env::var("HOME")?);
            let work_dir =
                repo_entry.target.as_ref().map(|target| match target.home.unwrap_or_default() {
                    true => home_dir,
                    false => path,
                });

            opts.working_directory = work_dir;
        }

        Ok(opts)
    }

    fn run_hook_entry<'run>(
        &self,
        hook: &'run HookEntry,
        kind: &'run HookKind,
    ) -> RicerResult<Option<(&'run String, i32, String, String)>> {
        let script = match kind {
            HookKind::Pre => hook.pre.as_ref(),
            HookKind::Post => hook.post.as_ref(),
        };

        let script = match script {
            Some(script) => script,
            None => return Ok(None),
        };

        let data = self.cfg_mgr.dir_manager().get_cmd_hook(script)?;
        let args = script.split_whitespace().skip(1).map(|s| s.to_string()).collect();
        let opts = self.setup_script_options(hook.repo.as_ref())?;
        let (code, output, error) = run_script!(data, args, opts)?;
        Ok(Some((script, code, output, error)))
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
            let (script, code, output, error) = match self.run_hook_entry(hook, &HookKind::Pre)? {
                Some(result) => result,
                None => continue,
            };

            if error.is_empty() {
                info!("Script '{}' (exit code: {}) stdout: {}", script, code, output);
            } else {
                error!("Script '{}' failed (exit code: {}): {}", script, code, error);
            }
        }

        Ok(())
    }

    fn run_post(&self) -> RicerResult<()> {
        let cmd_hook =
            self.cfg_mgr.file_manager().get_cmd_hook(self.ctx.to_string()).ok().unwrap_or_default();
        for hook in cmd_hook.hooks.iter() {
            let (script, code, output, error) = match self.run_hook_entry(hook, &HookKind::Post)? {
                Some(result) => result,
                None => continue,
            };

            if error.is_empty() {
                info!("Script '{}' (exit code: {}) stdout: {}", script, code, output);
            } else {
                error!("Script '{}' failed (exit code: {}): {}", script, code, error);
            }
        }

        Ok(())
    }
}

enum HookKind {
    Pre,
    Post,
}
