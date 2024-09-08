// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Welcome to Ricer's internal API!
//!
//! If you plan on contributing to Ricer, then you came to the right place! The
//! documentation stored here explains the _what_ and _why_. However, if you
//! want to find out about the _how_, then you need to look at the code itself!
//! Visit <https://github.com/rice-configs/ricer> for the code.
//!
//! # What is Ricer?
//!
//! Ricer is an experimental command-line tool designed for managing and
//! organizing [rice][explain-rice] configurations through [Git][git-scm]. Ricer
//! allows the user to treat their home directory like a regular Git directory.
//! Each configuration the user has will get stuffed into their own "fake-bare"
//! repository. The "fake-bare" repository system allows the user to modularize
//! their configurations for easier deployment across multiple machines.
//!
//! If Ricer's behavior seems familiar, then that is because Ricer borrows many
//! concepts from [vcsh][vcsh-repo]. In fact, one could argue that Ricer is the
//! Rust version of [vcsh][vcsh-repo]. Although Ricer attempts to combine
//! [vcsh][vcsh-repo] and [mr][mr-repo] under one neat little program in Rust.
//!
//! # Contributing
//!
//! The Ricer coding project is open to the following forms of contribution:
//!
//! 1. Improvements or additions to production code.
//! 1. Improvements or additions to test code.
//! 1. Improvements or additions to build system.
//! 1. Improvements or additions to documentation.
//! 1. Improvements or additions to CI/CD pipelines.
//!
//! See the [contribution guidelines][contrib-guide] for more information about
//! contributing to the Ricer project.
//!
//! [explain-ricing]: pesos.github.io/2020/07/14/what-is-ricing.html
//! [git-scm]: https://git-scm.com
//! [vcsh-repo]: https://github.com/RichiH/vcsh
//! [mr-repo]: https://github.com/RichiH/myrepos
//! [contrib-guide]: CONTRIBUTING.md

use shadow_rs::shadow;
shadow!(build);

pub mod error;

pub mod cli;

pub mod context;

pub mod config;

pub mod hook;

#[cfg(test)]
mod tests;
