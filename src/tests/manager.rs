// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::manager::{ConfigManager, DefaultDirLocator, RepositoryData, ConfigManagerError};
use crate::tests::FakeConfigDir;

use anyhow::Result;
use rstest::{fixture, rstest};
use indoc::indoc;


#[fixture]
fn config_data() -> Result<FakeConfigDir> {
    let fake = FakeConfigDir::builder()?
        .config_file(
            "repos.toml",
            indoc! {r#"
                [repos.vim]
                branch = "main"
                remote = "origin"
                workdir_home = true
            "#},
        )?
        .build();
    Ok(fake)
}

#[rstest]
fn config_manager_load(config_data: Result<FakeConfigDir>) -> Result<()> {
    let config_data = config_data?;
    let layout = StubDirLayout::new(&config_data);
    let locator = DefaultDirLocator::locate(layout);
    let mut config = ConfigManager::new(RepositoryData, locator);
    let result = config.load();
    assert!(matches!(result.unwrap_err(), ConfigManagerError::FileOpen { .. }));
    Ok(())
}
