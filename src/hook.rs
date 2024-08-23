// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::config::dir::{ConfigDirManager, DefaultConfigDirManager};
use crate::config::file::{ConfigFileManager, DefaultConfigFileManager};
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
    pub fn new(cfg_mgr: &'cfg ConfigManager<D, F>, ctx: &'cfg Context) -> Self
    {
        Self { cfg_mgr, ctx }
    }
}

impl<D: ConfigDirManager, F: ConfigFileManager> CommandHookManager for DefaultCommandHookManager<'_, D, F> {
    fn run_pre(&self) -> RicerResult<()> {
        todo!();
    }

    fn run_post(&self) -> RicerResult<()> {
        todo!();
    }
}
