// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use clap::ValueEnum;

use crate::ui::{CommandSet, Cli, SharedOptions};

#[derive(Debug)]
pub enum Context {
    Bootstrap(BootstrapContext),
}

impl From<Cli> for Context {
    fn from(opts: Cli) -> Self {
        match opts.cmd_set {
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct BootstrapContext {
    pub config: Option<String>,
    pub from: Option<String>,
    pub only: Option<Vec<String>>,
    pub shared: SharedContext,
}

impl From<Cli> for BootstrapContext {
    fn from(opts: Cli) -> Self {
        let Cli { shared_opts, cmd_set, .. } = opts;
        let cmd_set = match cmd_set {
            CommandSet::Bootstrap(opts) => opts,
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
pub struct SharedContext {
    pub run_hook: HookAction,
}

impl From<SharedOptions> for SharedContext {
    fn from(opts: SharedOptions) -> Self {
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
