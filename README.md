<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: MIT
-->

![GitHub Actions Workflow Status][build-status] ![REUSE 3.0 Compliance][reuse-compliance] ![GitHub Tag][version]

# Ricer

Ricer is an experimental dotfile management tool designed to allow the user to
treat their home directory like a Git repository. The goal of this tool is to
provide the user a way to distribute their custom [rice][explain-ricing]
configurations without the need to copy, move, or symlink them into their home
directory. Through Ricer, the user can modularize their configurations via
multiple Git repositories. Where each repository can be configured to use the
user's home directory as the working directory, or can be left as a regular
self-contained Git repository that Ricer needs to keep track of. Finally, Ricer
allows the user to further enhance their configurations through special command
hooks that can be configured to execute _before_ or _after_ a given Ricer
command.

Ricer tries to provide a thin layer of abstraction over Git. The idea is to
simplify interactions between the user and Git for dotfile management. However,
the user is still given the ability to directly use Git itself when they need
to. Ricer's command set itself resembles Git's command set in order to make it
easier to pick up for anyone familiar with Git in the first place.

## Requirements

Ricer requires the following to build:

- [Git][git-scm] [>= 2.25.0]
- [Rust][rust-lang] [>= 1.74.1]

Git is needed due to Ricer basing its core functionality around it. Rust is
required, because this project uses it as the primary programming language.
Ricer also uses Cargo to manage its dependencies.

## Installation

Ricer is available through \<<https://crates.io>\> as a lib+bin crate. Thus,
anyone can obtain a functioning release through Cargo like so:

```
# cargo install ricer
```

The above method of installation will only provide the latest release published
to \<<https://crates.io>\>. However, if the latest changes to the project are
desired, then build Ricer through a clone of the project repository:

```
# git clone https://github.com/rice-configs/ricer.git
# cd ricer
# cargo build
# cargo install
```

It is recommended to install release versions of Ricer rather than directly
installing the latest changes made to the project repository. The clone and
build method previously shown should generally be used by those who intend to
contribute to the project.

## Usage

__TODO__

## Acknowledgements

__TODO__

## Contributing

__TODO__

## Copyright and Licensing

__TODO__

[build-status]: https://img.shields.io/github/actions/workflow/status/rice-configs/ricer/quality_check.yaml?style=for-the-badge&label=Quality%20Check
[reuse-compliance]: https://img.shields.io/github/actions/workflow/status/rice-configs/ricer/reuse.yaml?style=for-the-badge&label=REUSE%203.0
[version]: https://img.shields.io/github/v/tag/rice-configs/ricer?style=for-the-badge
[explain-ricing]: pesos.github.io/2020/07/14/what-is-ricing.html
[git-scm]: https://git-scm.com/downloads
[rust-lang]: https://www.rust-lang.org/learn/get-started
