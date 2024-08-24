// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use log::{debug, error, info, trace};
use run_script::{run_script, ScriptOptions};
use std::env;
use std::path::PathBuf;

use crate::config::dir::ConfigDirManager;
use crate::config::file::hooks_section::CommandHookEntry;
use crate::config::file::ConfigFileManager;
use crate::config::ConfigManager;
use crate::context::Context;
use crate::error::RicerResult;

pub trait CommandHookManager {
    fn run_pre(&self) -> RicerResult<()>;
    fn run_post(&self) -> RicerResult<()>;
}

pub struct DefaultCommandHookManager<'cfg, D: ConfigDirManager, F: ConfigFileManager> {
    cfg_mgr: &'cfg ConfigManager<D, F>,
    ctx: &'cfg Context,
}

impl<'cfg, D: ConfigDirManager, F: ConfigFileManager> DefaultCommandHookManager<'cfg, D, F> {
    pub fn new(cfg_mgr: &'cfg ConfigManager<D, F>, ctx: &'cfg Context) -> Self {
        trace!("Construct new command hook manager");
        Self { cfg_mgr, ctx }
    }

    fn setup_script_options(&self, repo: Option<&String>) -> RicerResult<ScriptOptions> {
        let mut opts = ScriptOptions::new();
        if let Some(repo) = &repo {
            let (path, repo_entry) = self.cfg_mgr.get_repo(repo)?;
            let home_dir = PathBuf::from(env::var("HOME")?);
            let work_dir = repo_entry.target.as_ref().map(|target| {
                match target.home.unwrap_or_default() {
                    true => home_dir,
                    false => path,
                }
            });

            opts.working_directory = work_dir;
        }

        Ok(opts)
    }
}

impl<D: ConfigDirManager, F: ConfigFileManager> CommandHookManager
    for DefaultCommandHookManager<'_, D, F>
{
    fn run_pre(&self) -> RicerResult<()> {
        let cmd_hook =
            self.cfg_mgr.file_manager().get_cmd_hook(self.ctx.to_string()).ok().unwrap_or_default();
        for hook in cmd_hook.hooks.iter() {
            let pre = if let Some(pre) = &hook.pre { pre } else { continue };
            let data = self.cfg_mgr.dir_manager().get_cmd_hook(pre)?;
            let args: Vec<String> = pre.split_whitespace().skip(1).map(|s| s.to_string()).collect();
            let opts = self.setup_script_options(hook.repo.as_ref())?;
            let (code, output, error) = run_script!(data, args, opts)?;
            if error.is_empty() {
                info!("Script '{}' (exit code: {}) stdout: {}", pre, code, output);
            } else {
                error!("Script '{}' failed (exit code: {}): {}", pre, code, error);
            }
        }

        Ok(())
    }

    fn run_post(&self) -> RicerResult<()> {
        todo!();
    }
}
