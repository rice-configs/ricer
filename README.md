<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Ricer

Help user's manage their rice configurations.

Ricer is an experimental command-line tool designed for managing and organizing
[rice][explain-rice] configurations through [Git][git-scm]. Ricer allows the
user to treat their home directory like a regular Git directory. Each
configuration the user has will get stuffed into their own "fake-bare"
repository. The "fake-bare" repository system allows the user to modularize
their configurations for easier deployment across multiple machines.

If Ricer's behavior seems familiar, then that is because Ricer borrows many
concepts from [vcsh][vcsh-repo]. In fact, one could argue that Ricer is the Rust
version of [vcsh][vcsh-repo]. Although Ricer attempts to combine
[vcsh][vcsh-repo] and [mr][mr-repo] under one neat little program in Rust.

## Install

You will need the following pieces of software:

1. Git [>= 2.25.0].
1. Rust [>= 1.74.1].

Clone this repository and use Cargo like so:

```
# git clone https://github.com/rice-configs/ricer.git
# cd ricer
# cargo build --release
# cargo install
```

Make sure that your `$PATH` includes `$HOME/.cargo/bin` in order to execute the
Ricer binary.

Enjoy!

## Usage

Create a new Vim configuration repository through Ricer:

```
# ricer init vim
```

Add `.vimrc` into the new Vim repository:

```
# ricer vim add ~/.vimrc
```

Commit the newly added file:

```
# ricer vim commit -m 'Initial commit of my Vim config'
```

Add a remote to Vim configuration:

```
# ricer vim remote add origin https://url/to/remote/vim.git
```

Push changes to the remote:

```
# ricer vim push -u origin main
```

Now this Vim configuration can be cloned to a new machine:

```
# ricer clone https://url/to/remote/vim.git vim
```

Ricer does not offer the full Git command set due to the limitations of the
[git2-rs][libgit2-rs] library. Thus, if you need more advanced functionality
like sparse checkout, submodules, etc, then Ricer can place you directly into a
target repository to run the Git binary itself like so:

```
# ricer enter vim
```

Or you can use the following shortcut as well:

```
# ricer vim
```

For more details about using Ricer, then reference the provided manual page for
the project.

## Acknowledgements

Richard Hartmann's [vcsh][vcsh-repo] inspired Ricer's creation, and provided a
basic foundation for Ricer's command set and functionality.

The [git2-rs][libgit2-rs] library for offering a more idomatic way of
integrating Git into Ricer. Originally, Ricer was just going to wrap Git with
`std::process::Command` all over the place.

## Contributing

The Ricer coding project is open to the following forms of contribution:

1. Improvements or additions to production code.
1. Improvements or additions to test code.
1. Improvements or additions to build system.
1. Improvements or additions to documentation.
1. Improvements or additions to CI/CD pipelines.

See the [contribution guidelines][contrib-guide] for more information about
contributing to the Ricer project.

## Copyright and Licensing

The Ricer coding project uses a few different licenses to cover different
portions of the codebase for various reasons. However, Ricer should be
considered free software that uses the GNU GPL version 2 license with a few
extensions.

This project uses the [REUSE version 3 specification][reuse-v3-spec] to make it
easier to determine who owns the copyright and licensing of any given file in
the codebase with SPDX identifiers. Ricer also employs the [Developer
Certificate of Origin version 1.1][linux-dco] to ensure that any contributions
made have the right to be merged into the project, and can be distributed with
the project under its main license.

### Main License

Copyright (C) 2024 Jason Pena \<<jasonpena@awkless.com>\>

The Ricer program is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the Free
Software Foundation; either version 2 of the License, or (at your option) any
later version.

This program also uses the GPL Cooperation Commitment version 1.0 to give itself
the cure and reinstatement clauses offered by the GNU GPL version 3 to avoid
instant termination of its GPL license for any reported violations.

This program is distributed in the hope that it will be useful, but __WITHOUT
ANY WARRANTY__; without even the implied warranty of __MERCHANTABILITY__ or
__FITNESS FOR A PARTICULAR PURPOSE__. See the GNU General Public License for
more details.

You should have received a copy of the GNU General Public License and the
Cooperation Commitment along with Ricer; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

### Documentation License

The Ricer coding project uses the Creative Commons Attribute-ShareAlike 4.0
International license to cover its public documentation files. Hopefully, this
license will make it easier to distribute Ricer's documentation over the
internet.

### Public Domain License

Some portions of the codebase are either too small or too generic to claim
copyright over. Thus, these portions will be placed into the public domain
through the Creative Commons CC0 1.0 Unversial license. This license was
selected to for countries that legally have no concept of a public domain.

[explain-ricing]: pesos.github.io/2020/07/14/what-is-ricing.html
[git-scm]: https://git-scm.com
[vcsh-repo]: https://github.com/RichiH/vcsh
[mr-repo]: https://github.com/RichiH/myrepos
[libgit2-rs]: https://github.com/rust-lang/git2-rs
[contrib-guide]: CONTRIBUTING.md
[reuse-v3-spec]: https://reuse.software/spec-3.0/
[linux-dco]: https://en.wikipedia.org/wiki/Developer_Certificate_of_Origin
