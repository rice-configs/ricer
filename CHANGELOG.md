<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Changelog

All notable changes to this project will be documented in this file. See
[contributing guidelines](CONTRIBUTING.md) for commit guidelines.

## [0.4.0](https://github.com/rice-configs/ricer/compare/v0.3.0..0.4.0) - 2024-08-23

### Bug Fixes

- **(ricer)** Print context of error logs - ([54a1dbc](https://github.com/rice-configs/ricer/commit/54a1dbc78bc3bcd88dd1957fca3ff78853e8fa68)) - Jason Pena
- **(ricer::config::dir)** Don't run doctest for `DefaultConfigDirManager::new` - ([47d0071](https://github.com/rice-configs/ricer/commit/47d0071a0b487af62db503dd9bc76a7dfc187895)) - Jason Pena
- **(ricer::config::dir)** Fix tests for `remove_repo` - ([d116e23](https://github.com/rice-configs/ricer/commit/d116e233163ff160cac23dbbcdcbc80a43fdb90d)) - Jason Pena
- **(ricer::config::file)** Make `add_repo` use implicit for new `repos` section - ([732b372](https://github.com/rice-configs/ricer/commit/732b372d9cdd10ff6f04fb4e4638723820729cfa)) - Jason Pena
- **(ricer::config::file)** Make `rename_repo` preserve comments and formatting - ([bd8687e](https://github.com/rice-configs/ricer/commit/bd8687eb04ed750d57e8159107183dac0fd7f1e2)) - Jason Pena
- **(ricer::config::file)** Thanks clippy - ([2e6b875](https://github.com/rice-configs/ricer/commit/2e6b8751da09b6dc5ddef60bbe6647cbc03a9745)) - Jason Pena
- **(ricer::config::file::repos_section)** Only add `targets` if it exists - ([0c270bd](https://github.com/rice-configs/ricer/commit/0c270bd3f9e41195b92b24242f0505208082c124)) - Jason Pena
- **(ricer::config::locator)** Fix examples in documentation - ([ad6a5e6](https://github.com/rice-configs/ricer/commit/ad6a5e69197c0a69d1cb549afcdb2d2174bc8ad0)) - Jason Pena
- **(ricer_test_tools)** Make root directory in tempdir - ([3077078](https://github.com/rice-configs/ricer/commit/30770782822bfb927ce924b9e1a669f94d88ec72)) - Jason Pena

### Documentation

- **(contrib)** Update coding style to follow rustdoc book conventions - ([fbf31e2](https://github.com/rice-configs/ricer/commit/fbf31e28d230e39de5f1483e92577b17f7c7fcbd)) - Jason Pena
- **(ricer)** Document preconditions, postconditions, and invariants when needed - ([44276c5](https://github.com/rice-configs/ricer/commit/44276c5fce6a86ee65df44a4766f7e7aeda8c702)) - Jason Pena
- **(ricer::config)** Improve rustdocs for `ConfigManager` API - ([859d3c2](https://github.com/rice-configs/ricer/commit/859d3c2e1cf7c04b594e8e854105c5d842ee66a2)) - Jason Pena
- **(ricer::config)** Improve API documentation for `ConfigManager` - ([abae8a5](https://github.com/rice-configs/ricer/commit/abae8a5d70ad27f82f08e0502bb8e38cb8b2f46e)) - Jason Pena
- **(ricer::config::dir)** Document getters of `DefaultConfigDirManager` - ([47b8dea](https://github.com/rice-configs/ricer/commit/47b8dea87a581f1f152f64c63262a76816d77989)) - Jason Pena
- **(ricer::config::file::repos_section)** Document `RepoEntry::to_toml` - ([4e90843](https://github.com/rice-configs/ricer/commit/4e90843a28f12417003c9b0783a06e1ddb70b631)) - Jason Pena
- **(ricer::config::locator)** State design contract of API - ([e16e786](https://github.com/rice-configs/ricer/commit/e16e786b6facfe42998f5d72c99cbd798bfbfdec)) - Jason Pena
- **(ricer_core::config)** Document `DefaultConfigDir::try_new` - ([709151c](https://github.com/rice-configs/ricer/commit/709151cccac00ae432866267fcd250dd587ad474)) - Jason Pena
- **(ricer_core::error)** Document error module - ([62541ca](https://github.com/rice-configs/ricer/commit/62541ca3d814352c0b963d4ec6e291496383c8e1)) - Jason Pena

### Features

- **(bin)** Use `ricer::config::locator` module - ([c0d86d4](https://github.com/rice-configs/ricer/commit/c0d86d4c4c5ed16e6df58da3a530265f52056ebf)) - Jason Pena
- **(bin)** Use `ricer::config::locator` API - ([15fcd4d](https://github.com/rice-configs/ricer/commit/15fcd4d14207b6c22f4f17d4d134c1fcff91fb7c)) - Jason Pena
- **(bin)** Use `ricer::config::dir::DefaultConfigDirManager` - ([558c239](https://github.com/rice-configs/ricer/commit/558c2390bc3041bed068baab830f7da7e09ecbdf)) - Jason Pena
- **(ricer)** Add `ricer_core::config::Config` into `run_ricer` - ([7f10177](https://github.com/rice-configs/ricer/commit/7f101772d3e7c21e10a12a9b472f130ddaa92b47)) - Jason Pena
- **(ricer::config)** [**breaking**] Setup `locator` submodule - ([2ffef01](https://github.com/rice-configs/ricer/commit/2ffef01926ba8099b998eb60d51204535719b463)) - Jason Pena
- **(ricer::config)** Add `ConfigManager::new` - ([3c7f776](https://github.com/rice-configs/ricer/commit/3c7f776f6026a3080520fa147de9cdb6a1c463e2)) - Jason Pena
- **(ricer::config)** Add `ConfigManager::read_config_file` - ([b18e02b](https://github.com/rice-configs/ricer/commit/b18e02b1c44873720292162fd54dbb8237f23a98)) - Jason Pena
- **(ricer::config)** Add getters - ([b10e68d](https://github.com/rice-configs/ricer/commit/b10e68dfe64931aa27271098c25e12d83b32c7c0)) - Jason Pena
- **(ricer::config)** Add `ConfigManager::write_config_file` - ([a1f9804](https://github.com/rice-configs/ricer/commit/a1f9804fb64953fd1379522a2e2b638ec69d197f)) - Jason Pena
- **(ricer::config)** Add `Configuration::add_git_repo` - ([36ab0c2](https://github.com/rice-configs/ricer/commit/36ab0c280a1f681e939af97e072ab45251e48c40)) - Jason Pena
- **(ricer::config)** Add `ConfigManager::remove_git_repo` - ([da96e1e](https://github.com/rice-configs/ricer/commit/da96e1e542a778be30daf212418c73dea529590c)) - Jason Pena
- **(ricer::config)** Add `ConfigManager::get_repo` - ([5a198ff](https://github.com/rice-configs/ricer/commit/5a198ffbd15b046355bf80d4f7047d5f2de9fb57)) - Jason Pena
- **(ricer::config)** Add `Configuration::rename_repo` - ([f7df216](https://github.com/rice-configs/ricer/commit/f7df2162957d95e03e7e825ccd9287be143f550b)) - Jason Pena
- **(ricer::config)** Add `Configuration::get_cmd_hook` - ([b62f976](https://github.com/rice-configs/ricer/commit/b62f976e7707930ee8e10914068915ecd13cb7f9)) - Jason Pena
- **(ricer::config)** Add `Configuration::write_ignore_file` - ([0cc201d](https://github.com/rice-configs/ricer/commit/0cc201d64854d34e69ecfaf5053868f98c3e60c1)) - Jason Pena
- **(ricer::config)** Add getters for managers in `ConfigManager` - ([eea30d1](https://github.com/rice-configs/ricer/commit/eea30d13a01d2d6296e16a7f05fe6b87062dd4d2)) - Jason Pena
- **(ricer::config::dir)** Add `ConfigDirManager` trait - ([d5e8ad9](https://github.com/rice-configs/ricer/commit/d5e8ad9fffc4eab1634456be80133804b35df4cc)) - Jason Pena
- **(ricer::config::dir)** Add `DefaultConfigDirManager` implementation - ([bfd190b](https://github.com/rice-configs/ricer/commit/bfd190bbda16141c4a7931331079e4f989acac80)) - Jason Pena
- **(ricer::config::dir)** Add `try_find_config_file` to `ConfigDirManager` - ([9a8b99c](https://github.com/rice-configs/ricer/commit/9a8b99cc5165bb5415efd265d46b0fc81b123bed)) - Jason Pena
- **(ricer::config::dir)** Add `try_find_git_repo` to `ConfigDirManager` - ([4634645](https://github.com/rice-configs/ricer/commit/46346450a69ef9b6b5de2143537841ce87cc7e89)) - Jason Pena
- **(ricer::config::dir)** Add `try_find_hook_script` to `ConfigDirManager` - ([ebe12e6](https://github.com/rice-configs/ricer/commit/ebe12e6ed5a257b3b5b33621dbc9be7f6ce7bf77)) - Jason Pena
- **(ricer::config::dir)** Add `try_find_ignore_file` to `ConfigDirManager` - ([4fa105d](https://github.com/rice-configs/ricer/commit/4fa105d79741607360e2994fec23a6333e0738a1)) - Jason Pena
- **(ricer::config::dir)** Document and assert invariants for `new` - ([483ab1d](https://github.com/rice-configs/ricer/commit/483ab1d5bb9f5a6871402473e015b09f2a658c39)) - Jason Pena
- **(ricer::config::dir)** Add `DefaultConfigDirManager::get_repo` - ([44ff602](https://github.com/rice-configs/ricer/commit/44ff60295bfdcd1cbe2ab8393d8443572f76a618)) - Jason Pena
- **(ricer::config::dir)** Add `DefaultConfigDirManager::remove_repo` - ([b98cd72](https://github.com/rice-configs/ricer/commit/b98cd72b4b16247151ae55bcd7778643bc839145)) - Jason Pena
- **(ricer::config::dir)** Add `DefaultConfigDirManager::rename_repo` - ([f32467b](https://github.com/rice-configs/ricer/commit/f32467bbe0d5c2bcc327e893d6de407dbb8727be)) - Jason Pena
- **(ricer::config::dir)** Add `DefaultConfigDirManager::get_cmd_hook` - ([f115c06](https://github.com/rice-configs/ricer/commit/f115c06bb939b16ddb1530ee11177853fc4ad56c)) - Jason Pena
- **(ricer::config::dir)** Add `DefaultConfigDirManager::write_ignore_file` - ([c0c42e4](https://github.com/rice-configs/ricer/commit/c0c42e4682fa9151748ccdb28f0127d7be65eae8)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager` - ([fd10742](https://github.com/rice-configs/ricer/commit/fd10742bed4114a2a7646bbad0c495d0c9061603)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::read` - ([47b822a](https://github.com/rice-configs/ricer/commit/47b822acbf0d9f8464ad9c933c38ad33ca882e4f)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::write` - ([3da49b8](https://github.com/rice-configs/ricer/commit/3da49b85570949d778cdaed92eaa0ff3d34dc73f)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::to_string` - ([c064bd6](https://github.com/rice-configs/ricer/commit/c064bd68662a52daebf7b152036e9c0190e6b31e)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::add_repo` - ([3d12e6a](https://github.com/rice-configs/ricer/commit/3d12e6a19d70adf018def698036cd3cc5019c56e)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::get_repo` - ([a528041](https://github.com/rice-configs/ricer/commit/a528041e95ce7e576dec79e53f6716c12f584f78)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::remove_repo` - ([9f1d63a](https://github.com/rice-configs/ricer/commit/9f1d63af1805f62f800f8b7add7445799c9d373c)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::rename_repo` - ([bae95fb](https://github.com/rice-configs/ricer/commit/bae95fbe47c8f8965814ef25ca766ca49e5e863b)) - Jason Pena
- **(ricer::config::file)** Add `DefaultConfigFileManager::get_cmd_hook` - ([087703a](https://github.com/rice-configs/ricer/commit/087703adf4cde23fe7cc785930d6f20ab9726091)) - Jason Pena
- **(ricer::config::file::hooks_section)** Setup command hook structure - ([4c53290](https://github.com/rice-configs/ricer/commit/4c532903c2b1bde65c6c02a776fc2f6a217143ab)) - Jason Pena
- **(ricer::config::file::hooks_section)** Add builder for `HookEntry` - ([dfb6a2c](https://github.com/rice-configs/ricer/commit/dfb6a2c385e17e62ebe93de24407b3a8c499a607)) - Jason Pena
- **(ricer::config::file::hooks_section)** Add `CommandHookEntry::new` - ([76a961c](https://github.com/rice-configs/ricer/commit/76a961c898afaf778b12b22c9ba1125b01096cad)) - Jason Pena
- **(ricer::config::file::hooks_section)** Add `CommandHookEntry::add_hook` - ([f952aff](https://github.com/rice-configs/ricer/commit/f952affdaa3f174f6961a33735a07855ffe5c894)) - Jason Pena
- **(ricer::config::file::hooks_section)** Add From trait to `CommandHookEntry` - ([8b717b8](https://github.com/rice-configs/ricer/commit/8b717b8ebb3d31dd71543873a1761d5aaafb1805)) - Jason Pena
- **(ricer::config::file::repos_section)** Setup repos section definition - ([5c6c973](https://github.com/rice-configs/ricer/commit/5c6c97337010fbc7d10a19abf23c0b6def7de8c1)) - Jason Pena
- **(ricer::config::file::repos_section)** Add `RepoEntryBuilder` - ([47777e5](https://github.com/rice-configs/ricer/commit/47777e5352d23b572ab212ac5c15354932cc9bd5)) - Jason Pena
- **(ricer::config::file::repos_section)** Add builder to `RepoTargetEntry` - ([c3a8f0d](https://github.com/rice-configs/ricer/commit/c3a8f0dc3c8cc0e48a60e646c12ac3558db2dffd)) - Jason Pena
- **(ricer::config::file::repos_section)** Add `RepoEntry::from` - ([0575096](https://github.com/rice-configs/ricer/commit/0575096a76c449b242be27a14065e73aede1fe1e)) - Jason Pena
- **(ricer::config::file::repos_section)** Add `RepoEntry::to_toml` - ([e2f71ff](https://github.com/rice-configs/ricer/commit/e2f71ff22a844bd84ca33159050f564343d752a8)) - Jason Pena
- **(ricer::config::locator)** Add `ConfigDirLocator` trait - ([c5af2fb](https://github.com/rice-configs/ricer/commit/c5af2fb77269a6ec13393887631b572ebaa50e6c)) - Jason Pena
- **(ricer::config::locator)** Add `XdgConfigDirLocator` - ([3f149ed](https://github.com/rice-configs/ricer/commit/3f149ed66a74ee83a967368f23a9faeb20c63870)) - Jason Pena
- **(ricer::config::locator)** Add `recover_xdg_config_dir_locator` - ([6d2f39d](https://github.com/rice-configs/ricer/commit/6d2f39d4e27904f976439d0e8687d3e2ae43c344)) - Jason Pena
- **(ricer::config::locator)** Make `ConfigDirLocator` mockable - ([80e9d9f](https://github.com/rice-configs/ricer/commit/80e9d9fedc657f240c4c31409148718e1bf32db9)) - Jason Pena
- **(ricer::error)** Implement `RicerError::from` for toml_edit::Error type - ([68ee94e](https://github.com/rice-configs/ricer/commit/68ee94e191d7b6695e7af5cb50737b7a132a07fd)) - Jason Pena
- **(ricer::error)** Add error types for handling of `hooks` and `repos` section - ([1355b1e](https://github.com/rice-configs/ricer/commit/1355b1e998cb04a1963d3f2ac7d77e0650042978)) - Jason Pena
- **(ricer_core)** Connect `error` module - ([3f43f42](https://github.com/rice-configs/ricer/commit/3f43f42318ba8c710a01b53d34c82f3395e5e6f7)) - Jason Pena
- **(ricer_core)** Connect `config` module - ([15069c4](https://github.com/rice-configs/ricer/commit/15069c43394f7eea8a4665ff09cbc637dc6340a1)) - Jason Pena
- **(ricer_core::config)** Add `ConfigDir` trait - ([b0ad079](https://github.com/rice-configs/ricer/commit/b0ad079ca02465dc69eaeb2b9271ad04a282da99)) - Jason Pena
- **(ricer_core::config)** Add `DefaultConfigDir` - ([c54d2a6](https://github.com/rice-configs/ricer/commit/c54d2a6acce9183a09fa23d8c05c0461d3fd5585)) - Jason Pena
- **(ricer_core::config)** Add `DefaultConfigDir::try_new` - ([b6abb2b](https://github.com/rice-configs/ricer/commit/b6abb2b1a9ba22bfe29062ffe0d30de4b74b48a0)) - Jason Pena
- **(ricer_core::config)** Implement `ConfigDir` for `DefaultConfigDir` - ([b63df60](https://github.com/rice-configs/ricer/commit/b63df6026f2bd7dcebab64a56cde9ff593accbc6)) - Jason Pena
- **(ricer_core::config)** Add `Config` - ([66f394c](https://github.com/rice-configs/ricer/commit/66f394c694aec8c572058b2d27c7f93635fcbd0f)) - Jason Pena
- **(ricer_core::config)** Add `Config::new` - ([92b8696](https://github.com/rice-configs/ricer/commit/92b8696ffba8b9f18797b6fa91c0f564666a3b30)) - Jason Pena
- **(ricer_core::config)** Add `Config::try_to_find_repo` - ([d538659](https://github.com/rice-configs/ricer/commit/d53865944a43d4677118d6c5cad647bf9e75bd83)) - Jason Pena
- **(ricer_core::config)** Add `Config::try_to_find_hook` - ([8be6bdd](https://github.com/rice-configs/ricer/commit/8be6bdd9cc1f8ec2fd552d2726feeba6de4f1da9)) - Jason Pena
- **(ricer_core::config)** Add `Config::try_to_find_ignore` - ([49fee7a](https://github.com/rice-configs/ricer/commit/49fee7af6c25e1a46fee024e520b0c28755c091d)) - Jason Pena
- **(ricer_core::config)** Add `Config::try_to_read_config_file` - ([3b4cf80](https://github.com/rice-configs/ricer/commit/3b4cf804dab765c2b33c15c5701f784909894acb)) - Jason Pena
- **(ricer_core::config)** Add more logging to `Config` handler - ([63f68bf](https://github.com/rice-configs/ricer/commit/63f68bf052c5c02492a7f30e1b5b7ef0ae22e691)) - Jason Pena
- **(ricer_core::config::file)** Implement Ricer's configuration file layout - ([106eed6](https://github.com/rice-configs/ricer/commit/106eed69abe0615d5e1e07fc46a0b21c7e899c6c)) - Jason Pena
- **(ricer_core::error)** Add `RicerError::ConfigError` - ([1858e39](https://github.com/rice-configs/ricer/commit/1858e39c57681f47f64d95f6ab60df62d98e8db8)) - Jason Pena
- **(ricer_test_tools::fakes)** Add `path_to_config_file` to `FakeConfigDir` - ([0b48a4c](https://github.com/rice-configs/ricer/commit/0b48a4c8b76eca1295b935db3ab73c8add9de87b)) - Jason Pena
- **(ricer_test_tools::fakes)** Add mutable variations of obtaining stub - ([e7a3884](https://github.com/rice-configs/ricer/commit/e7a38847b4e18208b16fb10c0c16ef0a952b8938)) - Jason Pena

### Miscellaneous Chores

- **(cargo)** Add tempfile and rstest dependencies - ([9ed77f4](https://github.com/rice-configs/ricer/commit/9ed77f4a95b32a0a88c96e517f9df0535fbe4cd2)) - Jason Pena
- **(cargo)** Bump to version 0.4.0 - ([ce3df06](https://github.com/rice-configs/ricer/commit/ce3df067c9f6ca7f9f57e83879239af7fbb9b930)) - Jason Pena
- **(cargo)** Upgrade to never versions of dependencies - ([7d927de](https://github.com/rice-configs/ricer/commit/7d927dec3d47c33e2aa34645403b7a2dd114961a)) - Jason Pena
- **(cargo)** [**breaking**] Rename library `ricer_core` to `ricer` - ([f254c19](https://github.com/rice-configs/ricer/commit/f254c197ae523c88481ba6dc8c92eb93f3a954cf)) - Jason Pena
- **(cargo)** [**breaking**] Connect `ricer_test_tools` library - ([3dc327f](https://github.com/rice-configs/ricer/commit/3dc327fdb0d1d97019b40a6a547d61258ffd81a0)) - Jason Pena
- **(cargo)** Add mockall for mocking purposes in testing - ([7ad1ae6](https://github.com/rice-configs/ricer/commit/7ad1ae6fd4e458ab4fd0697e070f3256d35f56ae)) - Jason Pena
- **(ci/cd)** Document `ricer` and `ricer_test_tools` libs - ([e48bef4](https://github.com/rice-configs/ricer/commit/e48bef4dbe88b619c7a8c59a7dfdcf0002c983c0)) - Jason Pena

### Refactoring

- **(bin)** [**breaking**] Move `src/ricer.rs` into `src/bin/` - ([7661b5d](https://github.com/rice-configs/ricer/commit/7661b5de1042437ffbdbcdc94863fb6679fd47cd)) - Jason Pena
- **(crates)** Rename `ricer_tester` to `ricer_test_tools` - ([601e3a4](https://github.com/rice-configs/ricer/commit/601e3a47796541f8e17e76f89ef4b54e34480b8e)) - Jason Pena
- **(crates)** [**breaking**] Use `*_stub` postfix rather than `path_to` for fakes API - ([4ebde02](https://github.com/rice-configs/ricer/commit/4ebde02fb83d858858db329147f69b8a980efb73)) - Jason Pena
- **(integration)** [**breaking**] Test only documented contracts for `ConfigManager` API only - ([514be44](https://github.com/rice-configs/ricer/commit/514be44b47dbeef85dd628dd96ef4cc81e961c08)) - Jason Pena
- **(ricer::cli)** [**breaking**] Move unit tests to `tests` module - ([e764212](https://github.com/rice-configs/ricer/commit/e76421202e140bcadd87360331447c5aa26fbe3a)) - Jason Pena
- **(ricer::config)** [**breaking**] Remove getters - ([d12d69a](https://github.com/rice-configs/ricer/commit/d12d69a0259e56d608af1a6a39dc415e83187813)) - Jason Pena
- **(ricer::config)** [**breaking**] Use `RicerError::Unrecoverable` - ([edfa6b5](https://github.com/rice-configs/ricer/commit/edfa6b5af52363539ad9dddff00173b91c8af8e2)) - Jason Pena
- **(ricer::config)** Thanks clippy - ([6b217b7](https://github.com/rice-configs/ricer/commit/6b217b7310e46bede21bb3ac4e2a3599943db35b)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Rename `try_find_config_file` to `config_file_path` - ([5567308](https://github.com/rice-configs/ricer/commit/5567308d2b383c740c5e38c1b80299d85275b262)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Rename `try_find_git_repo` to `git_repo_path` - ([dd54806](https://github.com/rice-configs/ricer/commit/dd548067838c3258ca13543d1e3424fbdf1df47e)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Rename `find_hook_script` to `hook_script_path` - ([01af1db](https://github.com/rice-configs/ricer/commit/01af1db56ef3216ecf4c9b337fdf5217d5a8c61d)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Rename `find_ignore_file` to `ignore_file_path` - ([75be038](https://github.com/rice-configs/ricer/commit/75be0382f7c144fa3cabc4ab81195350a6a391e5)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Use special error types rather than `Unrecoverable` - ([0b6cf48](https://github.com/rice-configs/ricer/commit/0b6cf4891eba1f564f3b20decf21bd21a4b3b59b)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Replace `config_file_path` with `setup_config_file` - ([0b395d4](https://github.com/rice-configs/ricer/commit/0b395d4febb6860606cfb750fd821eafb0671d84)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Replace `git_repo_path` with `add_git_repo` - ([1bf23d1](https://github.com/rice-configs/ricer/commit/1bf23d1b8151181fa5b26a239fafe140f8a5e70c)) - Jason Pena
- **(ricer::config::dir)** [**breaking**] Make `rename_repo` fail for non-existent repo - ([dfb8bd9](https://github.com/rice-configs/ricer/commit/dfb8bd99cbef871d6e0e28e271b97d5bb4b0c3e2)) - Jason Pena
- **(ricer::config::file::hooks_section)** [**breaking**] Make it easier to use builder - ([5c3f833](https://github.com/rice-configs/ricer/commit/5c3f8330663868a7d5a4afb21dc535012a625190)) - Jason Pena
- **(ricer::config::file::repos_section)** [**breaking**] Make `target` field optional - ([91b053c](https://github.com/rice-configs/ricer/commit/91b053cca1b08219e73dd4f7dc731dbe63301be5)) - Jason Pena
- **(ricer::config::locator)** Move tests into separate file - ([17cf27d](https://github.com/rice-configs/ricer/commit/17cf27d9c62b7e1184f8ab7250734a4e1d4bd79f)) - Jason Pena
- **(ricer::config::locator)** [**breaking**] Remove `try_*` prefix from API - ([89a07cf](https://github.com/rice-configs/ricer/commit/89a07cfe39d91f4115422db7e83ed3d559915810)) - Jason Pena
- **(ricer::error)** [**breaking**] Make error handling better - ([ea13f2b](https://github.com/rice-configs/ricer/commit/ea13f2bd28bc408b77ab7838ed37cef37017f758)) - Jason Pena
- **(ricer_core)** [**breaking**] Append script and file to `..find_hook` and `..find_ignore` - ([647d23e](https://github.com/rice-configs/ricer/commit/647d23e68988fe74a4889961a3078a7b5aeb4d45)) - Jason Pena
- **(ricer_core::config)** Rename `try_to_find_repo` to `try_to_find_git_repo` - ([27f243c](https://github.com/rice-configs/ricer/commit/27f243ca9418c8598e3e15bdbb4cf83382cb0454)) - Jason Pena
- **(ricer_tester)** [**breaking**] Move `ricer_tester` into `crates/` directory - ([49cb8da](https://github.com/rice-configs/ricer/commit/49cb8da244b3c6474aa9cb9d70b5d5790089c0e5)) - Jason Pena
- **(tests)** [**breaking**] Move unit tests into 'tests' module - ([1777f59](https://github.com/rice-configs/ricer/commit/1777f594ab0cac558fd754086868ab0249056834)) - Jason Pena
- **(tests)** [**breaking**] Rename unit test files to more recognizable names - ([7707889](https://github.com/rice-configs/ricer/commit/770788927c2f131fde059f922826312a1fc5946f)) - Jason Pena
- **(tests::dir)** [**breaking**] Refactor test cases for `config_file_path` - ([af58894](https://github.com/rice-configs/ricer/commit/af588947dc28c5c09cecbe2da43364171f7d5706)) - Jason Pena
- **(tests::dir)** [**breaking**] Use renamed methods in `DefaultConfigDirManager` - ([67df695](https://github.com/rice-configs/ricer/commit/67df6958b069ff92906cb3ed01f742b312f603a5)) - Jason Pena
- Thanks clippy - ([6dac662](https://github.com/rice-configs/ricer/commit/6dac662a1a6dfc91b431c0035316d14d6c56d69b)) - Jason Pena

### Tests

- **(config)** Test `ricer_core::config::Config::try_to_find_hook` - ([49aea80](https://github.com/rice-configs/ricer/commit/49aea807d9f36993fda9e670f948220d43c052ca)) - Jason Pena
- **(config)** Test `ricer_core::config::Config::try_to_find_git_repo` - ([2ed642d](https://github.com/rice-configs/ricer/commit/2ed642d289e83f651c6532605159b3ea8c347004)) - Jason Pena
- **(config)** Test `ricer_core::config::Config::try_to_read_config_file` - ([a39f868](https://github.com/rice-configs/ricer/commit/a39f868799652016fe3e31844e6be8f6bedd16ee)) - Jason Pena
- **(config::config)** Test `ConfigDir::try_to_find_ignore` - ([43458d1](https://github.com/rice-configs/ricer/commit/43458d1d92e87e17d8c3306d2fab680004ab7fec)) - Jason Pena
- **(integration)** Test `ConfigManager::write_config_file` - ([9a1639a](https://github.com/rice-configs/ricer/commit/9a1639ac51adc2555acb09ff4ffab61bdde6e0b9)) - Jason Pena
- **(integration)** Test `ConfigManager::add_git_repo` - ([6d60b78](https://github.com/rice-configs/ricer/commit/6d60b78dd78dd553abc46991a78561725afba467)) - Jason Pena
- **(integration)** Test `Configuration::remove_git_repo` - ([ec31895](https://github.com/rice-configs/ricer/commit/ec31895aa48221cd43d6a12c02d88c81e5abc249)) - Jason Pena
- **(integration)** [**breaking**] Improve test cases by testing behavior by contract - ([92641ab](https://github.com/rice-configs/ricer/commit/92641abc3c318fd579051465f39ce47f6f046fcc)) - Jason Pena
- **(integration::config_manager)** Test `read_config_file` - ([697404d](https://github.com/rice-configs/ricer/commit/697404dcd6b82030f86c85ccd469f1053182d161)) - Jason Pena
- **(ricer::config::dir)** Test `try_find_config_file` - ([3e54065](https://github.com/rice-configs/ricer/commit/3e54065c44f147a16147deb39d03f677544dc25a)) - Jason Pena
- **(ricer::config::dir)** Test `try_find_git_repo` - ([ed4cbd3](https://github.com/rice-configs/ricer/commit/ed4cbd37f41046a75dedc60ddf363543bbdb1aa6)) - Jason Pena
- **(ricer::config::dir)** Test `try_find_hook_script` - ([74db0f8](https://github.com/rice-configs/ricer/commit/74db0f8011f5e44473c24aa85629e5815e2e4e87)) - Jason Pena
- **(ricer::config::dir)** Test `try_find_ignore_file` - ([4b56083](https://github.com/rice-configs/ricer/commit/4b560831c2e89bf31cf78f861900521d0ac9540b)) - Jason Pena
- **(ricer::config::file::repos_section)** Test `RepoEntry::from` - ([5a6690e](https://github.com/rice-configs/ricer/commit/5a6690ea9305401f66addcba3938973fcdf69b1e)) - Jason Pena
- **(ricer::config::locator)** Test `DefaultConfigDirLocator::try_new_locate` - ([431b067](https://github.com/rice-configs/ricer/commit/431b067ba3549df30eac571cba95d1f8f3c363ca)) - Jason Pena
- **(ricer::config::locator)** Test `recover_default_config_dir_locator` - ([d6bd550](https://github.com/rice-configs/ricer/commit/d6bd55024ccc0a4ab310881da754b86d3b3d5b97)) - Jason Pena
- **(ricer_test_tools)** [**breaking**] Move `stubs` module into `ricer_test_tools` lib - ([d8ee1e8](https://github.com/rice-configs/ricer/commit/d8ee1e87636d7226894da2777314b5cb4e78bf1e)) - Jason Pena
- **(ricer_test_tools)** [**breaking**] Move `fakes` module into test tool library - ([65e5df1](https://github.com/rice-configs/ricer/commit/65e5df1233d6d47df0c24b80577c789f686cdd85)) - Jason Pena
- **(tests)** Test `DefaultConfigDirManager::setup_config_file` - ([b00a6f6](https://github.com/rice-configs/ricer/commit/b00a6f6f4af1cc1a748e5658b5b3a71c42783a28)) - Jason Pena
- **(tests)** Test `DefaultConfigDirLocator::add_repo` - ([af351c8](https://github.com/rice-configs/ricer/commit/af351c8383eb8ad3e93c67cdcabfb9ecdb9691d6)) - Jason Pena
- **(tests)** Test `DefaultConfigDirManager::get_repo` - ([13c6e23](https://github.com/rice-configs/ricer/commit/13c6e230af3cc6c4c8c83c54a358d04b9f5ee57f)) - Jason Pena
- **(tests)** Test `DefaultConfigDirManager::remove_repo` - ([276f18f](https://github.com/rice-configs/ricer/commit/276f18f55f33a5c1f1c2ccb5ff573f3102e84f26)) - Jason Pena
- **(tests)** Test `DefaultConfigDirManager::rename_repo` - ([b3d1b78](https://github.com/rice-configs/ricer/commit/b3d1b78561147028adf6e6cf04344abc595fe446)) - Jason Pena
- **(tests)** Test `DefaultConfigDirManager::get_cmd_hook` - ([1737a53](https://github.com/rice-configs/ricer/commit/1737a5395d1f3f363442bb8876ab04f6ec3c6be9)) - Jason Pena
- **(tests)** Test `DefaultConfigDirManager::write_ignore_file` - ([05c0f04](https://github.com/rice-configs/ricer/commit/05c0f04d1c100d697c4c586e66c23f61bbf871b6)) - Jason Pena
- **(tests::file)** Test `DefaultConfigFileManager::write` - ([9f58ba9](https://github.com/rice-configs/ricer/commit/9f58ba9f02fc5c7a6bef0813ffb8f6cb5b02a780)) - Jason Pena
- **(tests::file)** Test `DefaultConfigFileManager::add_repo` - ([9496d93](https://github.com/rice-configs/ricer/commit/9496d93b555170b144cb8e917d37eebc53cf70df)) - Jason Pena
- **(tests::file)** Test `DefaultConfigFileManager::get_repo` - ([3915fa6](https://github.com/rice-configs/ricer/commit/3915fa6e38c50e845e6a2bde78bc1f873f7a0e99)) - Jason Pena
- **(tests::file)** Test `DefaultConfigFileManager::remove_repo` - ([b49163a](https://github.com/rice-configs/ricer/commit/b49163a0764c07fabbacdaa6ec46085d5f65b51d)) - Jason Pena
- **(tests::file)** Test `DefaultConfigFileManager::get_cmd_hook` - ([d8e7d64](https://github.com/rice-configs/ricer/commit/d8e7d6413274794f7d19406b2dd6ed2bd1e8fc16)) - Jason Pena
- **(tests::files)** Test `DefaultConfigFileManager::rename_repo` - ([ae0c762](https://github.com/rice-configs/ricer/commit/ae0c7623da437cae1122af71da7bc6a010ed359e)) - Jason Pena
- **(tests::hooks_section)** Test `CommandHookEntry::from` - ([c8f1f46](https://github.com/rice-configs/ricer/commit/c8f1f46a51a28d4780facfe26afe94ba20745c17)) - Jason Pena
- **(tests::repos_section)** Test `RepoEntry::to_toml` - ([2b20a16](https://github.com/rice-configs/ricer/commit/2b20a1613a3b69765c754977096e4ac571e64c19)) - Jason Pena
- **(tools)** Add `FakeConfigDir` - ([89e569a](https://github.com/rice-configs/ricer/commit/89e569aa17cf66dce9d8c004655893f1346afb88)) - Jason Pena
- **(tools)** Add `FakeConfigDir::new` - ([a057e87](https://github.com/rice-configs/ricer/commit/a057e870e10eeadaa3c687c2ba446ff5c603f55e)) - Jason Pena
- **(tools)** Implement `ricer_core::config::ConfigDir` for `FakeConfigDir` - ([322c9b1](https://github.com/rice-configs/ricer/commit/322c9b16ba24c929e4e526fc91a7aaf7c5b84aba)) - Jason Pena
- **(tools)** Implement `Drop` for `FakeConfigDir` - ([8ba933c](https://github.com/rice-configs/ricer/commit/8ba933cfe3d47c257dd1aaae145bd603c6a3d354)) - Jason Pena
- **(tools)** Add more logging - ([77fd294](https://github.com/rice-configs/ricer/commit/77fd29484e75051f7bb2660dc525d7e1167a40e2)) - Jason Pena
- **(tools)** Add Git repository stub handler - ([6c48b7d](https://github.com/rice-configs/ricer/commit/6c48b7d4f4edb1436f3af6f8cfbf6d3e051635a0)) - Jason Pena
- **(tools::fakes)** Move `FakeConfigDir` into `tools::fakes` module - ([6243f88](https://github.com/rice-configs/ricer/commit/6243f88b8a3d67765ca1086076575ba39b7c6f41)) - Jason Pena
- **(tools::fakes)** Add `FakeConfigDirBuilder` - ([f7ef65a](https://github.com/rice-configs/ricer/commit/f7ef65a9697c11fe0151af7bc63abec9357befad)) - Jason Pena
- **(tools::fakes)** Add `FakeConfigDir::find_ignore` - ([3a6b149](https://github.com/rice-configs/ricer/commit/3a6b149765ebf65169c23290867cecbbee15fce8)) - Jason Pena
- **(tools::fakes)** Add `FakeConfigDir::path_to_hook_script` - ([8576f78](https://github.com/rice-configs/ricer/commit/8576f781be02d76eb88b1d688bfd2a5df2b25546)) - Jason Pena
- **(tools::fakes)** Add `FakeConfigDirBuilder::config_file` - ([6a9fb3e](https://github.com/rice-configs/ricer/commit/6a9fb3e88d77603486683811f1a8af0750b5f183)) - Jason Pena
- **(tools::stubs)** Add `StubFile` with `StubFileBuilder` - ([aa03d5f](https://github.com/rice-configs/ricer/commit/aa03d5fbfe4ffe24b1d6bec4ef1072fc9f67e284)) - Jason Pena
- **(tools::stubs)** Document and change `StubFile` to `FileStub` - ([7064d61](https://github.com/rice-configs/ricer/commit/7064d616e9745cf9185c9dcc00dffb43e5879a5c)) - Jason Pena
- Add documentation to test tools - ([0030e36](https://github.com/rice-configs/ricer/commit/0030e361f8b882c8707dd6f09806673326049bfd)) - Jason Pena

## [0.3.0](https://github.com/rice-configs/ricer/compare/v0.2.0..v0.3.0) - 2024-07-12

### Documentation

- **(changelog)** Document version 0.3.0 - ([cf75a1d](https://github.com/rice-configs/ricer/commit/cf75a1de3f2bbbf929cabaa78415a0c5091ebe15)) - Jason Pena
- **(ricer)** Remove invariant for `run_ricer` - ([9f02285](https://github.com/rice-configs/ricer/commit/9f0228522e738472ff5e6aebfc14e3c4239aaa1d)) - Jason Pena
- **(ricer_core::context)** State what the context layer does - ([9a7010a](https://github.com/rice-configs/ricer/commit/9a7010a2a50aee5b1d3bd292be56d425e75eeff5)) - Jason Pena
- **(ricer_core::context)** Fix up wording of `HookAction` - ([ab83628](https://github.com/rice-configs/ricer/commit/ab836286e016fad353f1b628311f9c337f6b143c)) - Jason Pena

### Features

- **(main)** [**breaking**] Add usage of `ricer_core::context` - ([4191c52](https://github.com/rice-configs/ricer/commit/4191c52df116b8dca462b1dd2ecad139d608d607)) - Jason Pena
- **(ricer_core::context)** Setup command context state layer - ([373725e](https://github.com/rice-configs/ricer/commit/373725ea09028f13cb27fd979806f7617722c839)) - Jason Pena
- **(ricer_core::context)** Setup context state for commit command - ([2a12055](https://github.com/rice-configs/ricer/commit/2a120558fde9d25e1f6b6ba3874675ce75fcbab3)) - Jason Pena
- **(ricer_core::context)** Setup context for push command - ([7db66a7](https://github.com/rice-configs/ricer/commit/7db66a7a6168efd065049f469a1c5cf58905bb9b)) - Jason Pena
- **(ricer_core::context)** Setup context for pull command - ([dc12402](https://github.com/rice-configs/ricer/commit/dc12402c460a4414684a335d33e5bf7fb34bef97)) - Jason Pena
- **(ricer_core::context)** Setup context for init command - ([58d4581](https://github.com/rice-configs/ricer/commit/58d458198f514b7c8b015475e71a90040f30576d)) - Jason Pena
- **(ricer_core::context)** Setup context for clone command - ([80beee4](https://github.com/rice-configs/ricer/commit/80beee402204e6cc5b2b00d0ceaa13b81f073bb9)) - Jason Pena
- **(ricer_core::context)** Setup context for delete command - ([fc461fc](https://github.com/rice-configs/ricer/commit/fc461fc34b47a926308299ed9c0a09ac7cbf79ab)) - Jason Pena
- **(ricer_core::context)** Setup context for rename command - ([5661ec9](https://github.com/rice-configs/ricer/commit/5661ec9f4225298e91da630d92969e67488659c6)) - Jason Pena
- **(ricer_core::context)** Setup context for status command - ([2ff61a2](https://github.com/rice-configs/ricer/commit/2ff61a22bf48827f870e42588b14e514f6763041)) - Jason Pena
- **(ricer_core::context)** Setup context for list command - ([b4fdc60](https://github.com/rice-configs/ricer/commit/b4fdc608b04cb01f6a4b4fa6584be3c0179f405a)) - Jason Pena
- **(ricer_core::context)** Setup context for enter command - ([b52a313](https://github.com/rice-configs/ricer/commit/b52a313c4323ab1e36c0e05dc67dd280da3a9fa1)) - Jason Pena
- **(ricer_core::context)** Setup context for git subcommand shortcut - ([f36e38a](https://github.com/rice-configs/ricer/commit/f36e38ac06c5cd1d396d72ac9203c6c46e870a6c)) - Jason Pena

### Miscellaneous Chores

- **(cargo)** Bump version to 0.3.0 - ([b76f648](https://github.com/rice-configs/ricer/commit/b76f648a37ab7ae07a2ddb489327ef0d9396d81c)) - Jason Pena

### Refactoring

- **(clippy)** Thanks clippy - ([4df6709](https://github.com/rice-configs/ricer/commit/4df67099352b4d85eae5ae518e10d58e499ceb53)) - Jason Pena
- **(ricer_core)** Rename `UseGitBinOnRepo` to `RepoGit` - ([1761293](https://github.com/rice-configs/ricer/commit/1761293bc60d584003b64376846501970fed190d)) - Jason Pena
- **(ricer_core)** [**breaking**] Combine shared command options in `SharedContext` - ([5f64454](https://github.com/rice-configs/ricer/commit/5f64454b6d7f7b6b4a76fd6f2e16fac3d13a9f0d)) - Jason Pena

## [0.2.0](https://github.com/rice-configs/ricer/compare/v0.1.1..v0.2.0) - 2024-07-10

### Documentation

- **(changelog)** Document version 0.2.0 - ([afd3b9a](https://github.com/rice-configs/ricer/commit/afd3b9a8c36f9b72bac2831bc48f853b371590f2)) - Jason Pena
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
