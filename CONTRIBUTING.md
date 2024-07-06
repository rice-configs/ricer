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

Make sure that your commits are clear and descriptive, because they will be used
in changelog of the project.

The Ricer project uses the [Developer Certificate of Origin version
1.1][linux-dco]. All commits need to have the following trailer:

```
Signed-off-by: <name> <email>
```

## Rules of Licensing and Copyright

__TODO__

[rust-lang]: doc.rust-lang.org
[rust-style]: doc.rust-lang.org/beta/style-guide/index.html
[linux-dco]: https://en.wikipedia.org/wiki/Developer_Certificate_of_Origin
