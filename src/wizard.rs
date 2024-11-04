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
use std::hash::RandomState;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

#[derive(Default)]
pub struct HookPager {
    choice: Arc<AtomicBool>,
}

#[cfg_attr(test, automock)]
impl HookPager {
    pub fn new() -> Self {
        Self { choice: Arc::new(AtomicBool::default()) }
    }

    pub fn choice(&self) -> bool {
        self.choice.load(Ordering::Relaxed)
    }

    pub fn page_and_prompt(
        &self,
        file_name: &Path,
        workdir: &Path,
        file_data: &str,
    ) -> Result<(), HookPagerError> {
        let pager = Pager::new();
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
