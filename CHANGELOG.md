<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: MIT
-->

# Changelog


All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### [0.2.0] - 2024-10-20

### Added

- Add `ricer::error::ExitCode` to handle exit code types for Ricer binary.
- Add `ricer::error::RicerError` to handle internal error types.
    - Currently `ricer::error::RicerError::Unrecoverable` is used by caller to
      indcate that all methods of errory recovery failed, or were deemed to
      expensive to perform.
- Add `ricer::error::RicerResult` to indicate that caller is responsible for
  error recovery.
    - Now will use `anyhow::Result` to indicate to the caller that they do not
      need to perform error recover even if there is a way to do so.
- Add `ricer::ui::Cli` to perform command-line argument parsing, and represent
  Ricer's command-line interface.
- Add `ricer::context::Context` to provide context as an abstraction layer
  between Ricer's CLI and command set implementations.
    - Now changes to Ricer's CLI can be done freely without worry of braking
      existing command set implementations.
- Add rstest dependency to improve and simplify unit test cases.
- Add thiserror dependency to make it easier to define custom internal error
  types for `ricer::error::RicerError`.
- Configure rustfmt through `.rustfmt.toml` file.

### [0.1.0] - 2024-10-15

### Added

- Place Ricer under MIT license.
- Add CC0-1.0 license to place some stuff in public domain.
- Add `README.md` file.
- Add `CONTRIBUTING.md` file.
- Add `CODE_OF_CONDUCT.md` file.
- Add `SECURITY.md` file.
- Setup Cargo build system.
- Define `main` for Ricer binary.
- Add CI code quality check.
- Add CI REUSE v3.0 compliance check.
- Define default textual attributes in `.gitattributes`.
- Ignore `target/*` in `.gitignore`.
- Provide pull request template.
- Provide bug report template.
- Provide feature request template.
- Make @awkless main code owner of Ricer.

[Unreleased]: https://github.com/rice-configs/ricer/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/rice-configs/ricer/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rice-configs/ricer/releases/tag/v0.1.0
