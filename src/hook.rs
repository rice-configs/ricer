// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

use crate::error::RicerResult;

pub trait CommandHookManager {
    fn run_pre(&self) -> RicerResult<()>;
    fn run_post(&self) -> RicerResult<()>;
}
