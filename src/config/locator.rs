// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Locate configuration directory.
//!
//! Basic way to locate Ricer's configuration directory at an expected area in
//! the user's environment. This logic remains seperate from configuration
//! directory management logic to make it easier to change the expected location
//! of Ricer's configuration directory at any time if need be.
//!
//! By default Ricer expects its configuration directory to be at
//! `$XDG_CONFIG_HOME/ricer`, i.e., `$HOME/.config/ricer`. Thus, the
//! [`XdgConfigDirLocator`] handler is expected to be used. However, if the
//! location of the configuration directory needs to change for whatever reason,
//! then simply implement a new locator with [`ConfigDirLocator`] trait.
