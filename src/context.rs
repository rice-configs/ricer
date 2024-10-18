// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use clap::ValueEnum;

use crate::ui::{CmdSet, Cli, SharedOpts};

#[derive(Debug)]
pub enum Context {

}

impl From<Cli> for Context {
    fn from(opts: Cli) -> Self {
        match opts.cmd_set {
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct BootstrapCtx {
    pub config: Option<String>,
    pub from: Option<String>,
    pub only: Option<Vec<String>>,
    pub shared: SharedCtx,
}

impl From<Cli> for BootstrapCtx {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CmdSet::Bootstrap(opts) => opts,
            _ => unreachable!("This should never happen. The command is not 'bootstrap'!"),
        };

        Self {
            config: cmd_set.config,
            from: cmd_set.from,
            only: cmd_set.only,
            shared: shared_opts.into(),
        }
    }
}

#[derive(Debug)]
pub struct SharedCtx {
    pub run_hook: HookAction,
}

impl From<SharedOpts> for SharedCtx {
    fn from(opts: SharedOpts) -> Self {
        Self { run_hook: opts.run_hook }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum HookAction {
    Always,

    #[default]
    Prompt,

    Never,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum FixupAction {
    #[default]
    Amend,

    Reword,
}
