<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Changelog

All notable changes to this project will be documented in this file. See
[contributing guidelines](CONTRIBUTING.md) for commit guidelines.

## [0.2.0](https://github.com/rice-configs/ricer/compare/v0.1.1..0.2.0) - 2024-07-10

### Documentation

- **(contrib)** Detail what commits will be included in changelog - ([ea83245](https://github.com/rice-configs/ricer/commit/ea8324559957f9415d85ec64b11a029df0e40172)) - Jason Pena
- **(lib)** Document Ricer crate - ([02edaee](https://github.com/rice-configs/ricer/commit/02edaee5db3ca64e4d371b12a0f14795c71604fc)) - Jason Pena
- **(lib)** Module `ricer::cli` documents itself now - ([dd435fb](https://github.com/rice-configs/ricer/commit/dd435fb5d0a6418896aef8f4994232911a63aa37)) - Jason Pena
- **(ricer::cli)** Improve documentation of module - ([ea4d169](https://github.com/rice-configs/ricer/commit/ea4d169de6dd5de7cb878499623347a726d1d975)) - Jason Pena

### Features

- **(bin)** [**breaking**] Make `main` handle command execute - ([39fa17f](https://github.com/rice-configs/ricer/commit/39fa17f45d2e166a22a414160933005e97ea3668)) - Jason Pena
- **(lib)** Connect ricer::cli module to API - ([5ed867b](https://github.com/rice-configs/ricer/commit/5ed867bac19f513e5878d82a9b6f66edd2986f87)) - Jason Pena
- **(main)** [**breaking**] Setup `main` to use `rally::cli::Cli` - ([90cf6e4](https://github.com/rice-configs/ricer/commit/90cf6e43a356d19f04cadfa31404fc20fb3a8127)) - Jason Pena
- **(ricer::cli)** Setup basic skeleton of Ricer's cli - ([f2dc3f4](https://github.com/rice-configs/ricer/commit/f2dc3f41b945849cb4d6e4b3f6674c584ce38489)) - Jason Pena
- **(ricer::cli)** Implement `Cli::new_run` - ([39f3ee5](https://github.com/rice-configs/ricer/commit/39f3ee51e02515d9e726552bd9659f3f6a808c53)) - Jason Pena
- **(ricer::cli)** [**breaking**] Add skeleton of full command set and options - ([589d3c4](https://github.com/rice-configs/ricer/commit/589d3c4d312599336af0c9de0f638d8b49806c64)) - Jason Pena
- **(ricer::cli)** Add options for `Add` command - ([868067c](https://github.com/rice-configs/ricer/commit/868067c2d629c6b4c88c08122f4289cbfe7aa195)) - Jason Pena
- **(ricer::cli)** Setup options for commit command - ([25e6ff8](https://github.com/rice-configs/ricer/commit/25e6ff821d2d0c3cafbea8cd82424dc0ab877707)) - Jason Pena
- **(ricer::cli)** Setup options for push command - ([fa6fd28](https://github.com/rice-configs/ricer/commit/fa6fd28ae72304402bf19711d188378eca10aa95)) - Jason Pena
- **(ricer::cli)** Setup options for pull command - ([4a68273](https://github.com/rice-configs/ricer/commit/4a6827371054340e44faf7f5da8fbc3739076283)) - Jason Pena
- **(ricer::cli)** Setup options for init command - ([9983c4f](https://github.com/rice-configs/ricer/commit/9983c4f946e773c962f435444bea2d67ccaa9ed9)) - Jason Pena
- **(ricer::cli)** Setup options for clone command - ([7f6c75a](https://github.com/rice-configs/ricer/commit/7f6c75af04ce102c06b00e57a000537ec41dcd8f)) - Jason Pena
- **(ricer::cli)** Setup options for delete command - ([dcb3ad7](https://github.com/rice-configs/ricer/commit/dcb3ad7a15bd95b64b40c8f28ad983e1e72554ae)) - Jason Pena
- **(ricer::cli)** Setup options for rename command - ([ff1e73e](https://github.com/rice-configs/ricer/commit/ff1e73e8d430cb599f6607c00015e2f1b89a4e38)) - Jason Pena
- **(ricer::cli)** Setup options of status command - ([4bd9826](https://github.com/rice-configs/ricer/commit/4bd9826db9a0721e67f1554da19ff411af6c4bd9)) - Jason Pena
- **(ricer::cli)** Setup options for list command - ([018443a](https://github.com/rice-configs/ricer/commit/018443ace78dc36315c9990d0da32886880d8b54)) - Jason Pena
- **(ricer::cli)** Setup options for enter command - ([a2fb977](https://github.com/rice-configs/ricer/commit/a2fb9774848dd56e0a415b4f7c5713439994283c)) - Jason Pena
- **(ricer::cli)** [**breaking**] Use external command for using Git binary on a repository - ([380d56b](https://github.com/rice-configs/ricer/commit/380d56b2af0d2680873020108398b646a9130d6c)) - Jason Pena
- **(ricer::cli)** Add GPL boilerplate for long version info - ([2e80074](https://github.com/rice-configs/ricer/commit/2e8007434be69afc58fe121f419be105d988fa33)) - Jason Pena
- **(ricer::cli)** Make help document external subcommand usage - ([cb857ee](https://github.com/rice-configs/ricer/commit/cb857ee28b29941cd7ee407262da80fc7a093759)) - Jason Pena

### Miscellaneous Chores

- **(cargo)** [**breaking**] Bump crate version to 0.2.0 - ([e4c4f85](https://github.com/rice-configs/ricer/commit/e4c4f85d4707b8f4cdeb9b2a6985da2ab73d9775)) - Jason Pena
- **(cargo)** [**breaking**] Add indoc, shadow-rs, and const_format dependencies - ([502dc78](https://github.com/rice-configs/ricer/commit/502dc78ab724a21db46f671461102bae297f784a)) - Jason Pena
- **(cargo)** [**breaking**] Setup shadow-rs to obtain project info - ([ccafa20](https://github.com/rice-configs/ricer/commit/ccafa20012e9df757ad48a70ff4fdba0496c9170)) - Jason Pena
- **(cargo)** [**breaking**] Replace envy and shellexpand with directories - ([34567be](https://github.com/rice-configs/ricer/commit/34567beb40c0f49ee83bb804d46269340bc2e079)) - Jason Pena
- **(cargo)** [**breaking**] Give library and binary separate names - ([9bab23e](https://github.com/rice-configs/ricer/commit/9bab23e1f98b44969a8eba7647b3d10a75c17a07)) - Jason Pena
- **(ci/cd)** Generate documentation for Ricer binary - ([e50451c](https://github.com/rice-configs/ricer/commit/e50451c7725c85d64b5dd5b43c2a8ecea5ec19f0)) - Jason Pena
- **(cliff)** Ignore style commits and clippy scope commits - ([a0b93f0](https://github.com/rice-configs/ricer/commit/a0b93f08f56161c5669ad1802a00db56cb2c4eb9)) - Jason Pena
- **(cliff)** Skip changelog updates - ([1d55b1b](https://github.com/rice-configs/ricer/commit/1d55b1bb46689fadd0ee3c24ae896097085b8af4)) - Jason Pena

### Refactoring

- **(ricer::cli)** Thanks clippy - ([5efc296](https://github.com/rice-configs/ricer/commit/5efc2969367d7df70903e704ea4b665cfa01769d)) - Jason Pena
- **(ricer::cli)** [**breaking**] Remove `RicerCli::new_run` in favor for `RicerCli::parse_args` - ([2c92284](https://github.com/rice-configs/ricer/commit/2c922844000e17bafa35954ec5884a7f6358f6e0)) - Jason Pena
- **(ricer::cli)** [**breaking**] Separate hook execution into individual options - ([412a18d](https://github.com/rice-configs/ricer/commit/412a18db91e864f5394bcae89ce3f7be59e5af65)) - Jason Pena

### Tests

- **(ricer::cli)** Verify that CLI works as expected - ([a178c30](https://github.com/rice-configs/ricer/commit/a178c300f916520109e3c9fda948abd5ef00e4bb)) - Jason Pena

## [0.1.1](https://github.com/rice-configs/ricer/compare/v0.1.0..v0.1.1) - 2024-07-07

### Documentation

- **(readme)** Add status badges - ([d6643fd](https://github.com/rice-configs/ricer/commit/d6643fd344907fd668566716acb495808d013cf1)) - Jason Pena

### Miscellaneous Chores

- **(cargo)** Bump crate version to 0.1.1 - ([39b9e58](https://github.com/rice-configs/ricer/commit/39b9e58072de58ffae81fb2300a0e9cc9b4c6925)) - Jason Pena
- **(changelog)** Document new changes in 0.1.1 - ([fa3f5a2](https://github.com/rice-configs/ricer/commit/fa3f5a25db488395b9237fd64b72585763275b88)) - Jason Pena
- **(cliff)** [**breaking**] Remove Unreleased link - ([4599eba](https://github.com/rice-configs/ricer/commit/4599eba258f867d11ff44ef8a9a85d31e4793e11)) - Jason Pena
- **(rustfmt)** Configure rustfmt - ([8cee725](https://github.com/rice-configs/ricer/commit/8cee72586d926427db61bd970f5447cb7e78eaa6)) - Jason Pena

## [0.1.0](https://github.com/rice-configs/ricer/releases/tag/v0.1.0) - 2024-07-07

### Documentation

- **(changelog)** Document version 0.1.0 - ([6c731a6](https://github.com/rice-configs/ricer/commit/6c731a67ba7eaeee83c7c41bb258b00869be0043)) - Jason Pena
- **(coc)** Use Contributor Covenant as main COC - ([8ae9e54](https://github.com/rice-configs/ricer/commit/8ae9e540b2acaa395efc8ba576f97a58fdc02690)) - Jason Pena
- **(contrib)** Setup skeleton of `CONTRIBUTING.md` - ([8a50476](https://github.com/rice-configs/ricer/commit/8a50476ceeac411fe03f19734ccbabf48dc581db)) - Jason Pena
- **(contrib)** State expected forms of contribution - ([33c1027](https://github.com/rice-configs/ricer/commit/33c10278d5c3ec11be67e9c66531348a3e368145)) - Jason Pena
- **(contrib)** Provide coding style - ([4995271](https://github.com/rice-configs/ricer/commit/4995271770fbed2841224df9c12348b54c7b8da6)) - Jason Pena
- **(contrib)** Provide commit style - ([3a5bf60](https://github.com/rice-configs/ricer/commit/3a5bf606d369db398691ab50a18d0d7a2541179c)) - Jason Pena
- **(contrib)** Provide rules for copyright and licensing - ([6afea09](https://github.com/rice-configs/ricer/commit/6afea09e2fcb9c259c2b92eff08cb5e239cadfff)) - Jason Pena
- **(git)** Ignore `target/` directory - ([d917b02](https://github.com/rice-configs/ricer/commit/d917b024192c6cd48084807ceb68a7c25c7cca0a)) - Jason Pena
- **(github)** Provide bug report template - ([57bd706](https://github.com/rice-configs/ricer/commit/57bd706b6a169cae24197bba9764158f0e89c9c1)) - Jason Pena
- **(github)** Provide feature request template - ([d790781](https://github.com/rice-configs/ricer/commit/d79078185bc2a861f997ba99c51b0122e0d49314)) - Jason Pena
- **(license)** Place Ricer under GNU GPL v2+ - ([01a065c](https://github.com/rice-configs/ricer/commit/01a065c7e98952787f13b0d7018bc7a3634fbf60)) - Jason Pena
- **(license)** Add GPL Cooperation Commitment 1.0 - ([f9ebbcd](https://github.com/rice-configs/ricer/commit/f9ebbcd590904a7e01114e0b4db46a9c96e23e0f)) - Jason Pena
- **(license)** Add CC-BY-SA-4.0 for documentation - ([8a593e2](https://github.com/rice-configs/ricer/commit/8a593e2036991b7e8252e9afa3b369a11d01f56b)) - Jason Pena
- **(license)** Add CC0-1.0 for uncopyrightable work - ([81bc13c](https://github.com/rice-configs/ricer/commit/81bc13c9c20e81871f93c8e527a74d514e2510ad)) - Jason Pena
- **(license)** Provide pull request template - ([e9fcc20](https://github.com/rice-configs/ricer/commit/e9fcc20daadb15f02036296b1b63fdd32bfaf9e4)) - Jason Pena
- **(readme)** Setup skeleton of `README.md` - ([6811c0b](https://github.com/rice-configs/ricer/commit/6811c0bb43126096bfefe5bf8473606d152f7f50)) - Jason Pena
- **(readme)** Provide installation instructions - ([bc9abf5](https://github.com/rice-configs/ricer/commit/bc9abf527e464c982aaeb25c7640f6f08923458d)) - Jason Pena
- **(readme)** State expected forms of contribution - ([d4bcbfa](https://github.com/rice-configs/ricer/commit/d4bcbfa21091949937061677afc031cf48fe4b09)) - Jason Pena
- **(readme)** State licensing and copyright - ([93a75b9](https://github.com/rice-configs/ricer/commit/93a75b9833ea03a85141a12ad90d9e79b8efa942)) - Jason Pena
- **(readme)** Describe what Ricer is - ([9d2873b](https://github.com/rice-configs/ricer/commit/9d2873bff53ee1c4cae7fcd2e5ce8597c73f568f)) - Jason Pena
- **(readme)** Provide acknowledgement section - ([d7225f7](https://github.com/rice-configs/ricer/commit/d7225f7629b3808ac0822534ddae56d67a2318c2)) - Jason Pena
- **(readme)** Provide usage example - ([4403dd4](https://github.com/rice-configs/ricer/commit/4403dd4fb69ada441cb68b6a4ce947cba2e6f6e0)) - Jason Pena
- **(security)** Provide security policy - ([17ffc25](https://github.com/rice-configs/ricer/commit/17ffc2582d6b873ffeb9d19412beed5984af5210)) - Jason Pena

### Features

- **(lib)** Setup internal library - ([cdaeb04](https://github.com/rice-configs/ricer/commit/cdaeb042fee5a1f5d318f29d83d31d552fedb94c)) - Jason Pena
- **(main)** Setup `main` - ([2c0fd88](https://github.com/rice-configs/ricer/commit/2c0fd88c971ea4a04ac30851d194cd63b733a9b8)) - Jason Pena

### Miscellaneous Chores

- **(cargo)** Setup Cargo to build project - ([275eac8](https://github.com/rice-configs/ricer/commit/275eac8fc9e8b46ed17c4277143b99ecea99df5b)) - Jason Pena
- **(ci/cd)** Setup REUSE 3.0 compliance check - ([f19c19d](https://github.com/rice-configs/ricer/commit/f19c19d5ae1a6b31b2241eb87b5eb693ba1b1ae0)) - Jason Pena
- **(ci/cd)** Setup quality check gauntlet for Rust code - ([289e6ad](https://github.com/rice-configs/ricer/commit/289e6adbd3beef0d70ed3ba91d21bbb18c8aac48)) - Jason Pena
- **(ci/cd)** Publish Ricer API documentation to GitHub pages - ([e25fcb3](https://github.com/rice-configs/ricer/commit/e25fcb30b9a9522540b3c164a06157b7662711ca)) - Jason Pena
- **(cliff)** Setup git cliff to generate changelog - ([a94310d](https://github.com/rice-configs/ricer/commit/a94310d9f343936a54bd27c8513b13b9cfd47a38)) - Jason Pena
- **(git)** Define default textual attributes - ([e0fbddb](https://github.com/rice-configs/ricer/commit/e0fbddb58ee0797d7f52630ca6ba444dfd29bd74)) - Jason Pena
- **(github)** Make @awkless main code owner - ([549a525](https://github.com/rice-configs/ricer/commit/549a525161bbd81042a84c6c81d7e05ade062ac6)) - Jason Pena

<!-- generated by git-cliff -->
