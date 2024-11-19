// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Command hook management.
//!
//! Manage command hook definitions and hook scripts to be executed for Ricer.
//! Command hooks only apply to Ricer's unique command set, and does not effect
//! regular Git commands. Thus, the command hook to execute is determined by
//! the parsed command-line arguments the user gave to Ricer, which is obtained
//! through [`Context`].
//!
//! Hooks are defined in Ricer's special command hook configuration file.
//! Commands can have multiple hook definitions stuffed into an array. Each
//! hook definition specifies one or two hook scripts to be executed from the
//! special `hooks/` directory. Ricer will only look for hook scripts in the
//! `hooks/` directory.
//!
//! Hooks can come in two forms: _pre_ and _post_. Pre hooks are meant to be
//! executed _before_ a given Ricer command, and post hooks are meant to execute
//! _after_. The user can control whether or not a hook script can be executed
//! in three ways: _always_ execute the hook no questions asked, _never_ execute
//! the hook no questions asked, or page the hooks contents and _prompt_ the
//! user about executing it.

use crate::{
    config::{CmdHookConfig, ConfigFile, ConfigFileError, TomlError},
    context::{Context, HookAction},
    locate::Locator,
};

use log::info;
use minus::{
    error::MinusError,
    input::{HashedEventRegister, InputEvent},
    page_all, ExitStrategy, LineNumbers, Pager,
};
use run_script::{run_script, ScriptError, ScriptOptions};
use shellexpand::{full as expand_var, LookupError};
use std::{
    env::VarError,
    fs::read_to_string,
    hash::RandomState,
    io::Error as IoError,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};

/// Error types for [`CmdHook`].
#[derive(Debug, thiserror::Error)]
pub enum CmdHookError {
    #[error("Failed to load command hook configuration file")]
    LoadConfig { source: ConfigFileError },

    #[error("Failed to get command hook data")]
    GetCmdHook { source: TomlError },

    #[error("Failed to read hook '{path}'")]
    HookRead { source: IoError, path: PathBuf },

    #[error("Failed to run hook")]
    RunHook { source: ScriptError },

    #[error("Failed to run pager")]
    HookPager { source: HookPagerError },

    #[error("Failed to expand hook work directory path")]
    ExpandPath { source: LookupError<VarError> },
}

impl From<ConfigFileError> for CmdHookError {
    fn from(err: ConfigFileError) -> Self {
        CmdHookError::LoadConfig { source: err }
    }
}

impl From<TomlError> for CmdHookError {
    fn from(err: TomlError) -> Self {
        CmdHookError::GetCmdHook { source: err }
    }
}

impl From<ScriptError> for CmdHookError {
    fn from(err: ScriptError) -> Self {
        CmdHookError::RunHook { source: err }
    }
}

impl From<HookPagerError> for CmdHookError {
    fn from(err: HookPagerError) -> Self {
        CmdHookError::HookPager { source: err }
    }
}

/// Error types for [`HookPager`].
#[derive(Debug, thiserror::Error)]
pub enum HookPagerError {
    #[error("Minus pager failed because '{source}'")]
    Minus { source: MinusError },
}

impl From<MinusError> for HookPagerError {
    fn from(err: MinusError) -> Self {
        HookPagerError::Minus { source: err }
    }
}

/// Command hook execution handler.
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
#[derive(Debug)]
pub struct CmdHook<'cfg, L>
where
    L: Locator,
{
    context: &'cfg Context,
    locator: &'cfg L,
    config: ConfigFile<'cfg, CmdHookConfig, L>,
    pager: HookPager,
}

impl<'cfg, L> CmdHook<'cfg, L>
where
    L: Locator,
{
    /// Load new command hook handler.
    ///
    /// Will load the contents of the command hook configuration file based
    /// on the path provided by `locator`. Will also load user selected actions
    /// from `context`.
    ///
    /// # Errors
    ///
    /// 1. Return [`CmdHookError::LoadConfig`] if configuration file cannot be
    ///    read and parsed for some reason.
    ///
    /// # See also
    ///
    /// - [`ConfigFile`]
    /// - [`Locator`]
    pub fn load(context: &'cfg Context, locator: &'cfg L) -> Result<Self, CmdHookError> {
        let config = ConfigFile::load(CmdHookConfig, locator)?;
        Ok(Self { context, locator, config, pager: Default::default() })
    }

    /// Run user-defined hooks.
    ///
    /// Run specific hook kind for given command that was selected through
    /// [`Context`].
    ///
    /// # Errors
    ///
    /// 1. Return [`CmdHookError::GetCmdHook`] if current command hook
    ///    definition cannot be obtained through hook configuration file.
    /// 2. Return [`CmdHookError::HookRead`] if hook script cannot be read
    ///    from `hooks/` directory.
    /// 3. Return [`CmdHookError::RunHook`] if hook script cannot be executed
    ///    for whatever reason.
    /// 4. Return [`CmdHookError::HookPager`] if pager cannot page hook script
    ///    and prompt user.
    pub fn run_hooks(&self, hook_kind: HookKind) -> Result<(), CmdHookError> {
        // INVARIANT: Git command shortcut cannot execute hooks.
        if matches!(self.context, Context::Git(..)) {
            return Ok(());
        }

        let action = self.get_hook_action();
        if action == &HookAction::Never {
            return Ok(());
        }

        let cmd_hook = match self.config.get(self.context.to_string()) {
            Ok(entry) => entry,
            // INVARIANT: Ricer commands are allowed not to have hooks.
            Err(ConfigFileError::Toml { source: TomlError::EntryNotFound { .. }, .. }) => {
                return Ok(())
            }
            Err(err) => return Err(err.into()),
        };

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
            let hook_data = read_to_string(&hook_path)
                .map_err(|err| CmdHookError::HookRead { source: err, path: hook_path.clone() })?;
            // INVARIANT: all working directory paths must be shell expanded.
            let hook_dir = self.expand_workdir(hook.workdir)?;

            if action == &HookAction::Prompt {
                self.pager.page_and_prompt(hook_path.as_path(), &hook_dir, &hook_data)?;
                if !self.pager.choice() {
                    continue; // Skip this iteration if user denied hook script.
                }
            }

            let mut hook_opts = ScriptOptions::new();
            hook_opts.working_directory = hook_dir;
            let (code, out, err) = run_script!(hook_data, hook_opts)?;
            info!("({code}) {}\nstdout: {out}\nstderr: {err}", hook_path.display());
        }

        Ok(())
    }

    /// Perform shell expansion on working directory path.
    ///
    /// Provides the following forms of expansion:
    ///
    /// - Tilde expansion, e.g., `~/some/path`.
    /// - Environment expansion like `$A` or `${B}` or `${C:32}`, e.g.,
    ///   `$HOME/some/path`.
    ///
    /// # Errors
    ///
    /// - Return [`CmdHookError::ExpandPath`] if path expansion failed for some reason.
    fn expand_workdir(&self, workdir: Option<PathBuf>) -> Result<Option<PathBuf>, CmdHookError> {
        match workdir {
            Some(workdir) => {
                let workdir = workdir.to_string_lossy().into_owned();
                let workdir = expand_var(&workdir)
                    .map_err(|err| CmdHookError::ExpandPath { source: err })?
                    .into_owned();
                Ok(Some(PathBuf::from(workdir)))
            }
            None => Ok(None),
        }
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

/// Hook type to execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookKind {
    /// Execute hooks _before_ command.
    Pre,

    /// Execute hooks _after_ command.
    Post,
}

/// Pager for hook scripts.
///
/// Basic static pager that shows the current contents of a given hook script,
/// and prompts the user about whether or not they want to execute it. User
/// can accept or deny hook script by pressing "a" or "d".
///
/// # See also
///
/// - [Minus](https://docs.rs/minus/latest/minus/)
#[derive(Debug, Default)]
pub struct HookPager {
    choice: Arc<AtomicBool>,
}

impl HookPager {
    pub fn new() -> Self {
        Self { choice: Arc::new(AtomicBool::default()) }
    }

    pub fn choice(&self) -> bool {
        self.choice.load(Ordering::Relaxed)
    }

    /// Page hook script and prompt user about running it.
    ///
    /// # Errors
    ///
    /// - Return [`HookPagerError::Minus`] for any issues encountered with
    ///   [Minus](https://docs.rs/minus/latest/minus/).
    pub fn page_and_prompt(
        &self,
        file_name: &Path,
        workdir: &Option<PathBuf>,
        file_data: &str,
    ) -> Result<(), HookPagerError> {
        let pager = Pager::new();
        let workdir = match workdir {
            Some(path) => path.clone(),
            None => PathBuf::from("./"),
        };

        pager.set_prompt(format!(
            "Run '{}' at '{}'? [a]ccept/[d]eny",
            file_name.display(),
            workdir.display(),
        ))?;
        pager.show_prompt(true)?;
        pager.set_run_no_overflow(true)?;
        pager.set_line_numbers(LineNumbers::Enabled)?;
        pager.push_str(file_data)?;
        pager.set_input_classifier(self.generate_key_bindings())?;
        pager.set_exit_strategy(ExitStrategy::PagerQuit)?;
        page_all(pager)?;

        Ok(())
    }

    fn generate_key_bindings(&self) -> Box<HashedEventRegister<RandomState>> {
        let mut input = HashedEventRegister::default();

        let response = self.choice.clone();
        input.add_key_events(&["a"], move |_, _| {
            response.store(true, Ordering::Relaxed);
            InputEvent::Exit
        });

        let response = self.choice.clone();
        input.add_key_events(&["d"], move |_, _| {
            response.store(false, Ordering::Relaxed);
            InputEvent::Exit
        });

        Box::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use crate::context::Context;
    use crate::locate::MockLocator;
    use crate::testenv::{FakeDir, FixtureKind};

    use anyhow::Result;
    use indoc::{formatdoc, indoc};
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn config_dir() -> Result<FakeDir> {
        let fake_dir = FakeDir::open()?;
        let top_level = fake_dir.as_path().to_path_buf();
        let fake_dir = fake_dir
            .with_file(
                "hooks.toml",
                indoc! {r#"
                    [hooks]
                    bootstrap = [
                        { pre = "pre_hook.sh" },
                        { post = "post_hook.sh" },
                    ]
                "#},
                FixtureKind::NormalFile,
            )
            .with_file(
                "hooks/pre_hook.sh",
                formatdoc! {r#"
                    #!/bin/sh

                    echo "hello from pre hook" > {}/out.txt
                    exit 0
                "#, top_level.display()},
                FixtureKind::ScriptFile,
            )
            .with_file(
                "hooks/post_hook.sh",
                formatdoc! {r#"
                    #!/usr/bin/env bash

                    echo "hello from post hook" > {}/out.txt
                    exit 0
                "#, top_level.display()},
                FixtureKind::ScriptFile,
            )
            .with_file("bad_hooks.toml", "should 'fail'", FixtureKind::NormalFile)
            .write()?;
        Ok(fake_dir)
    }

    #[rstest]
    fn cmd_hook_load_parses_config_file(config_dir: Result<FakeDir>) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_fixture("hooks.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_hooks_config().return_const(fixture.as_path().into());
        locator.expect_hooks_dir().return_const(config_dir.as_path().join("hooks"));

        let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "bootstrap"])?);
        let cmd_hook = CmdHook::load(&ctx, &locator)?;
        assert_eq!(fixture.as_str(), cmd_hook.config.to_string());
        Ok(())
    }

    #[rstest]
    fn cmd_hook_load_return_err_config_file(config_dir: Result<FakeDir>) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_fixture("bad_hooks.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_hooks_config().return_const(fixture.as_path().into());
        locator.expect_hooks_dir().return_const(config_dir.as_path().join("hooks"));

        let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "bootstrap"])?);
        let result = CmdHook::load(&ctx, &locator);
        assert!(matches!(result.unwrap_err(), CmdHookError::LoadConfig { .. }));

        Ok(())
    }

    #[rstest]
    #[case::pre_hooks(HookKind::Pre, "hello from pre hook\n")]
    #[case::post_hooks(HookKind::Post, "hello from post hook\n")]
    fn cmd_hook_run_hooks_execute_pre_and_post_hooks(
        config_dir: Result<FakeDir>,
        #[case] hook_kind: HookKind,
        #[case] expect: &str,
    ) -> Result<()> {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_fixture("hooks.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_hooks_config().return_const(fixture.as_path().into());
        locator.expect_hooks_dir().return_const(config_dir.as_path().join("hooks"));

        let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "bootstrap"])?);
        let cmd_hook = CmdHook::load(&ctx, &locator)?;
        cmd_hook.run_hooks(hook_kind)?;
        config_dir.sync()?;
        let result = config_dir.get_fixture("out.txt")?;
        assert_eq!(result.as_str(), expect);

        Ok(())
    }

    #[rstest]
    #[case::pre_hooks(HookKind::Pre)]
    #[case::post_hooks(HookKind::Post)]
    fn cmd_hook_run_hooks_ignore_git_shortcut(
        config_dir: Result<FakeDir>,
        #[case] hook_kind: HookKind,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_fixture("hooks.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_hooks_config().return_const(fixture.as_path().into());
        locator.expect_hooks_dir().return_const(config_dir.as_path().join("hooks"));

        let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "vim", "commit"])?);
        let cmd_hook = CmdHook::load(&ctx, &locator)?;
        assert!(cmd_hook.run_hooks(hook_kind).is_ok());

        Ok(())
    }

    #[rstest]
    #[case::pre_hooks(HookKind::Pre)]
    #[case::post_hooks(HookKind::Post)]
    fn cmd_hook_run_hooks_ignore_no_entry_for_cmd(
        config_dir: Result<FakeDir>,
        #[case] hook_kind: HookKind,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_fixture("hooks.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_hooks_config().return_const(fixture.as_path().into());
        locator.expect_hooks_dir().return_const(config_dir.as_path().join("hooks"));

        let ctx = Context::from(Cli::parse_args(["ricer", "--run-hook=always", "commit"])?);
        let cmd_hook = CmdHook::load(&ctx, &locator)?;
        assert!(cmd_hook.run_hooks(hook_kind).is_ok());

        Ok(())
    }
}
