// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use minus::error;

#[derive(Debug, thiserror::Error)]
pub enum HookPagerError {
    #[error("Minus pager failed because '{source}'")]
    Minus { source: error::MinusError },
}

impl From<error::MinusError> for HookPagerError {
    fn from(err: error::MinusError) -> Self {
        HookPagerError::Minus { source: err }
    }
}
