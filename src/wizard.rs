// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Wizard implementations.
//!
//! This module implements helpful wizard functionality to make Ricer's CLI more
//! user friendly.

mod error;

#[doc(inline)]
pub use error::*;

use minus::input::{HashedEventRegister, InputEvent};
use minus::{page_all, ExitStrategy, LineNumbers, Pager};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub trait PagerPrompt {
    fn page_and_prompt(&self, file_name: &str, file_data: &str) -> Result<bool, HookWizardError>;
}

#[derive(Debug, Default)]
pub struct HookWizard;

impl PagerPrompt for HookWizard {
    fn page_and_prompt(&self, file_name: &str, file_data: &str) -> Result<bool, HookWizardError> {
        let choice = Arc::new(AtomicBool::default());
        let mut input = HashedEventRegister::default();
        let response = choice.clone();
        input.add_key_events(&["a"], move |_, _| {
            response.store(true, Ordering::Relaxed);
            InputEvent::Exit
        });
        let response = choice.clone();
        input.add_key_events(&["d"], move |_, _| {
            response.store(false, Ordering::Relaxed);
            InputEvent::Exit
        });

        let pager = Pager::new();
        pager.set_prompt(format!("Do you want to execute '{}'? [A]ccept/[D]eny", file_name))?;
        pager.show_prompt(true)?;
        pager.set_run_no_overflow(true)?;
        pager.set_line_numbers(LineNumbers::Enabled)?;
        pager.push_str(file_data)?;
        pager.set_input_classifier(Box::new(input))?;
        pager.set_exit_strategy(ExitStrategy::PagerQuit)?;
        page_all(pager)?;
        Ok(choice.load(Ordering::Relaxed))
    }
}
