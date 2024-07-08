<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Contributing to Ricer

Thanks for taking the time to contribute! Remember that the information stored
in this document only provides basic _guidelines_. Thus, all contributors are
expected to use their best judgement!

## Expected Forms of Contribution

This project is open to the following forms of contribution:

1. Improvements or additions to production code.
1. Improvements or additions to test code.
1. Improvements or additions to build system.
1. Improvements or additions to documentation.
1. Improvements or additions to CI/CD pipelines.

Use the provided templates for bug reports, feature requests, and pull requests.
Please only use the bug tracker for reporting bug reports, or feature request
submissions.

Pull request submissions must occur on a separate branch, and compared by the
current state of the `main` branch. Keep commit history linear. Linear commit
history makes it easier to perform rebase merging. This project does not like
merge commits.

## Coding Style

The Ricer project uses the [Rust][rust-lang] programming langauge. Rust already
comes with a general [style and coding standard][rust-style] that should be
followed. To make development easier, use the `rustfmt` tool to automaically
format any piece of code.

Document all functions regardless if they are public or private via rustdocs.
Be sure to include any preconditions, postconditions, invariants, and side
effects a given piece of code may have. If applicable, then also provide
examples on how to use the code via rustdocs. Do not use rustdocs to document
implementation details, because that should be expressed in the code.

## Commit Style

All commits in Ricer must abide by the [Conventional Commits 1.0.0][cc1.0.0]
specification. Here are the following valid types for commits accepted by this
project:

- __chore__: General change that does not affect production code.
- __feat__: Change that implements a feature in production code.
- __fix__: Change that fixes a bug in production code.
- __doc__: Improvements or fixes to documentation.
- __style__: Improvements or fixes to the formatting of code.
- __ref__: Changes involve refactoring the code base.
- __rev__: A set of commits were revereted.
- __test__: Test functionality of code base.
- __perf__: Improvements to performance of code base.

Try keep the subject, body, and trailer of your commits below 80 characters. Try
to keep commit history linear. Make sure that large changes get represented as a
series of commits rather than one massive commit.

Make sure that your commits are clear and descriptive, because they might be
used in changelog of the project. If you make refactors based on the results
provided by clippy, then be sure to use the _clippy_ scope when commiting those
refactors. By using the _clippy_ scope, you ensure that those commits do not get
added. User's will only really care about refactors that change the way Ricer's
interface or commands work, which is always a breaking change anyway.  Also, if
you use the _style_ tag for commits, then they will be skipped too, because
user's do not care about how the code looks!

The Ricer project uses the [Developer Certificate of Origin version
1.1][linux-dco]. All commits need to have the following trailer:

```
Signed-off-by: <name> <email>
```

## Rules of Licensing and Copyright

This project abides by the [REUSE 3.0][reuse3] specification to determine the
licensing and copyright of files in the code base. Thus, all files must have the
proper SPDX copyright and licensing tags at the top always. Contributors can
Use the [reuse tool][reuse-tool] to determine if their changes are REUSE 3.0
compliant.

Ricer uses a number of different licenses to cover different sections of the
codebase. Regardless, Ricer's main license is the GNU GPL v2+ license with the
GPL Cooperation Commitment version 1.0. Thus, all major pieces of source code
will need the following SPDX license identifier tag: `GPL-2.0-or-later WITH
GPL-CC-1.0`.

If you add a file that mainly provides documentation, then you will need to use
the Creative Commons Attribution-ShareAlike 4.0 International license, because
the GPL only covers source code. Thus, all documentation files will need the
following SPDX license identifier tag: `CC-BY-SA-4.0`.

Finally, if you add files that are either too small or too generic to place
copyright over, or you just want the file to exist in the public domain, then
use the Creative Commons CC0 1.0 Unviersal license via this SPDX license
identifier: `CC0-1.0`

Do not forget to include the following SPDX copyright identifier at the top of
any file you create along with the SPDX copyright identifier:

```
SPDX-FileCopyrightText: <year> <name> <email>
```

[rust-lang]: doc.rust-lang.org
[rust-style]: doc.rust-lang.org/beta/style-guide/index.html
[linux-dco]: https://en.wikipedia.org/wiki/Developer_Certificate_of_Origin
[reuse3]: https://reuse.software/spec/
[reuse-tool]: https://reuse.software/tutorial/
