// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! General utilities.
//!
//! Basic utilities to make writing tests easier.

use std::fmt::Write;

/// Check a `Result` with a useful panic message.
///
/// # Examples
///
/// ```should_panic
/// use ricer_test_tools::util::err_check;
///
/// err_check!(std::fs::read_to_string("config.toml"));
/// ```
#[macro_export]
macro_rules! err_check {
    ($expr:expr) => {
        match $expr {
            Ok(data) => data,
            Err(err) => {
                $crate::util::anyhow_panic(&format!("Failed running {}", stringify!($expr)), err)
            }
        }
    };
}

pub use err_check;

/// Panic with [`anyhow::Error`] context.
///
/// # Examples
///
/// ```should_panic
/// use ricer_test_tools::util::anyhow_panic;
///
/// match std::fs::read_to_string("config.toml") {
///     Ok(data) => todo!(),
///     Err(err) => anyhow_panic("Failed to read config.toml", err),
/// }
/// ```
///
/// [`anyhow::Error`]: https://docs.rs/anyhow/latest/anyhow/struct.Error.html
#[track_caller]
pub fn anyhow_panic(what: &str, err: impl Into<anyhow::Error>) -> ! {
    let err = err.into();
    let mut result = format!("{}\nerror: {}\n", what, err);
    for cause in err.chain().skip(1) {
        let _ = writeln!(result, "Cause by: {}", cause);
    }
    panic!("\n{}", result);
}
