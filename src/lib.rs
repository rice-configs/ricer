// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Welcome to Ricer's internal API!
//!
//! This internal API is employeed by Ricer to manage implementation details of
//! its command set and various features. It is not recommended to utilize this
//! API outside of the Ricer codebase, because it is only designed to meet the
//! needs of the Ricer binary.
//!
//! It is recommended to look through the codebase itself to better understand
//! what the internal API is doing, and how it is doing it. Developers can use
//! rustdoc comments to explain _why_ something was coded the way it is, and to
//! give a general overview of what is going on in the codebase. However, the
//! code itself should be self documenting.
//!
//! # What is Ricer?
//!
//! Ricer is an experimental dotfile management tool designed to allow the user to
//! treat their home directory like a Git repository. The goal of this tool is to
//! provide the user a way to distribute their custom [rice][explain-ricing]
//! configurations without the need to copy, move, or symlink them into their home
//! directory. Through Ricer, the user can modularize their configurations via
//! multiple Git repositories. Where each repository can be configured to use the
//! user's home directory as the working directory, or can be left as a regular
//! self-contained Git repository that Ricer needs to keep track of. Finally, Ricer
//! allows the user to further enhance their configurations through special command
//! hooks that can be configured to execute _before_ or _after_ a given Ricer
//! command.
//!
//! Ricer tries to provide a thin layer of abstraction over Git. The idea is to
//! simplify interactions between the user and Git for dotfile management. However,
//! the user is still given the ability to directly use Git itself when they need
//! to. Ricer's command set itself resembles Git's command set in order to make it
//! easier to pick up for anyone familiar with Git in the first place.
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
//! [contrib-guide]: https://github.com/rice-configs/ricer/blob/main/CONTRIBUTING.md

pub mod cli;
pub mod config;
pub mod context;
pub mod hook;
pub mod locate;

#[cfg(test)]
pub(crate) mod test_tools;

#[cfg(test)]
mod tests;
