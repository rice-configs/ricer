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

As an example of using Ricer, lets create a new Vim editor configuration. First
we need to initialize a new repository under Ricer:

```
# ricer init vim --workdir_home
```

The above command will initialize a new repository named "vim" for Ricer to keep
track of. The `--workdir_home` flag tells Ricer that the "vim" repository will
use our home directory as the working directory, which will make the repository
bare, and allow us to manage it through our home directory.

Now in our home directory we have a `.vimrc` file full of configuration
information for Vim. We will add it to the "vim" repository so Ricer can keep
track of our changes:

```
# ricer vim add .vimrc
```

Now we will commit our changes through Ricer:

```
# ricer vim commit -m "inital commit to vim"
```

Lets specify the remote and push our changes to it:

```
# ricer vim remote add origin https://url/to/vim/remote.git
# ricer push
```

Now, lets setup a hook that will install the plug.vim plugin manager. First we
need a hook script that must be defined at `$XDG_CONFIG_HOME/ricer/hooks/`. Lets
call the script `$XDG_CONFIG_HOME/ricer/hooks/vim_plug.sh`. It will contain the
following shell code:

```
#!/bin/sh

curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
```

Secondly, we need to specify the hook script to execute _after_ a given Ricer
command in the `$XDG_CONFIG_HOME/ricer/hooks.toml` file. Lets use the
__bootstrap__ command hook like so:

```
[hooks]
bootstrap = [
    { post = "vim_plug.sh" }
]
```

Now, whenever we execute the bootstrap command, this new hook we created will be
executed _after_ the command has finished running.

Finally, lets specify bootstrap options for the "vim" repository so we can
quickly obtain our new Vim configuration across different machines:

```
# ricer bootstrap --config vim
```

The above command will make Ricer display its bootstrap option configuration
wizard. All we have to do is follow the wizard's instructions.

Now on a new machine, we can quickly obtain a valid instance of our "vim"
repository like so:

```
# ricer bootstrap "https://url/to/remote/we/told/wizard/to/use.git
```

The above command will boostrap the "vim" repository, and execute the special
hook we specified for it. For more information about using Ricer, refer to its
help menu via `--help` flag.

## Contributing

The Ricer coding project is open to the following forms of contribution:

1. Improvements or additions to production code.
1. Improvements or additions to test code.
1. Improvements or additions to build system.
1. Improvements or additions to documentation.
1. Improvements or additions to CI/CD pipelines.

See the [contribution guidelines][contrib-guide] for more information about
contributing to the Ricer project.

## Acknowledgements

1. Richard Hartmann's [vcsh][vcsh-repo] inspired Ricer's creation, and provided a
   basic foundation for Ricer's command set and functionality.

1. The [git2-rs][libgit2-rs] library for offering a more idomatic way of
   integrating Git into Ricer. Originally, Ricer was just going to wrap Git with
   `std::process::Command` all over the place.

1. The [Arch Linux Wiki][arch-wiki] provides a very detailed explaination of
   dotfiles and ricing. It also gives a good explaination of how to use Git for
   dotfile management through bare repositories. This unique way of using Git
   provided a basic idea on how to implement Ricer's core functionality.

## Copyright and Licensing

The Ricer coding project uses the MIT license as its main license for its
source code and documentation. Ricer also uses the CC0-1.0 license to place
files in the public domain that are to small or generic to place copyright over.

This project uses the [REUSE version 3 specification][reuse-v3-spec] to make it
easier to determine who owns the copyright and licensing of any given file in
the codebase with SPDX identifiers. Ricer also employs the [Developer
Certificate of Origin version 1.1][linux-dco] to ensure that any contributions
made have the right to be merged into the project, and can be distributed with
the project under its main license.

[build-status]: https://img.shields.io/github/actions/workflow/status/rice-configs/ricer/quality_check.yaml?style=for-the-badge&label=Quality%20Check
[reuse-compliance]: https://img.shields.io/github/actions/workflow/status/rice-configs/ricer/reuse.yaml?style=for-the-badge&label=REUSE%203.0
[version]: https://img.shields.io/github/v/tag/rice-configs/ricer?style=for-the-badge
[explain-ricing]: pesos.github.io/2020/07/14/what-is-ricing.html
[git-scm]: https://git-scm.com/downloads
[rust-lang]: https://www.rust-lang.org/learn/get-started
[contrib-guide]: CONTRIBUTING.md
[vcsh-repo]: https://github.com/RichiH/vcsh
[libgit2-rs]: https://github.com/rust-lang/git2-rs
[arch-wiki]: https://wiki.archlinux.org/title/Dotfiles#Tracking_dotfiles_directly_with_Git
[reuse-v3-spec]: https://reuse.software/spec-3.0/
[linux-dco]: https://en.wikipedia.org/wiki/Developer_Certificate_of_Origin
