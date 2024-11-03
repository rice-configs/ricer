// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use minus::error;

#[derive(Debug, thiserror::Error)]
pub enum HookWizardError {
    #[error("Minus pager failed because '{source}'")]
    Minus { source: error::MinusError }
}

impl From<error::MinusError> for  HookWizardError {
    fn from(err: error::MinusError) -> Self {
        HookWizardError::Minus { source: err }
    }
}
