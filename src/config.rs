// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

//! Configuration file management.
//!
//! Manage Ricer's special configuration files by providing ways to perform
//! parsing, serialization, and deserialization, while preserving the original
//! formatting of said configuration files. Ricer uses the [TOML file
//! format][toml-spec] as the main data exchange format for configuration file
//! data. Thus, all logic in this module is centered around TOML.
//!
//! Ricer currently is expected to manage two types of configuration file:
//! repository, and hook configurations. These configuration files are mainly
//! located at whatever path is expected from any [`Locator`] implementation.
//! Currently, expected location for these configuration files is in the
//! `$XDG_CONFIG_HOME/ricer` directory.
//!
//! [toml-spec]: https://toml.io/en/v1.0.0
//!
//! # See also
//!
//! - [`XdgDirLayout`]
//! - [`DefaultLocator`]
//!
//! [`XdgDirLayout`]: crate::locate::XdgDirLayout
//! [`DefaultLocator`]: crate::locate::DefaultLocator

mod settings;
mod toml;

#[doc(inline)]
pub use settings::*;
pub use toml::*;

use crate::locate::Locator;

use log::debug;
use mkdirp::mkdirp;
use std::{
    fmt,
    fs::OpenOptions,
    io,
    io::{Read, Write},
    path::{Path, PathBuf},
};

/// Error types for [`ConfigFile`].
#[derive(Debug, thiserror::Error)]
pub enum ConfigFileError {
    #[error("Failed to make parent directory '{path}'")]
    MakeDirP { source: io::Error, path: PathBuf },

    #[error("Failed to open '{path}'")]
    FileOpen { source: io::Error, path: PathBuf },

    #[error("Failed to read '{path}'")]
    FileRead { source: io::Error, path: PathBuf },

    #[error("Failed to write '{path}'")]
    FileWrite { source: io::Error, path: PathBuf },

    #[error("Failed to parse '{path}'")]
    Toml { source: TomlError, path: PathBuf },
}

/// Format preserving configuration file handler.
///
/// Manage configuration file data by selecting which configuration startegy to
/// use, i.e., which configuration file type to handle. Currently, there exists
/// two configuration file types: repository, and command hook. Once caller has
/// selected configuration file type to use, the [`Locator`] they pass in will
/// determine the expected path of the configuration file.
///
/// The configuration file will be opened if it exists at the expected path
/// assigned by the [`Locator`]. However, if the configuration file does not
/// exist, then it will be created at the expected path instead. This includes
/// the parent directory if needed.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
/// - [`RepoConfig`]
/// - [`CmdHookConfig`]
/// - [`DefaultLocator`]
///
/// [`DefaultLocator`]: crate::locate::DefaultLocator
#[derive(Clone, Debug)]
pub struct ConfigFile<'cfg, C, L>
where
    C: Config,
    L: Locator,
{
    doc: Toml,
    config: C,
    locator: &'cfg L,
}

impl<'cfg, C, L> ConfigFile<'cfg, C, L>
where
    C: Config,
    L: Locator,
{
    /// Load new configuration manager.
    ///
    /// If path to configuration file does not exist, then it will be created at
    /// target location. Otherwise, configuration file will be read and parsed
    /// like normal.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigFileError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigFileError::FileRead`] if target configuration file
    ///    could not be read.
    /// 1. Return [`ConfigFileError::Toml`] if target configuration file
    ///    could not be parsed into TOML format.
    pub fn load(config: C, locator: &'cfg L) -> Result<Self, ConfigFileError> {
        let path = config.location(locator);
        debug!("Load new configuration manager from '{}'", path.display());
        let root = path.parent().unwrap();
        mkdirp(root).map_err(|err| ConfigFileError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(false)
            .read(true)
            .create(true)
            .open(path)
            .map_err(|err| ConfigFileError::FileOpen { source: err, path: path.into() })?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| ConfigFileError::FileRead { source: err, path: path.into() })?;
        let doc: Toml = buffer
            .parse()
            .map_err(|err| ConfigFileError::Toml { source: err, path: path.into() })?;

        Ok(Self { doc, config, locator })
    }

    /// Save configuration data at expected location.
    ///
    /// If expected configuration file does not exist at location, then it will
    /// be created and written into automatically.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::MakeDirP`] if parent directory to to
    ///    expected configuration file path could not be created when needed.
    /// 1. Return [`ConfigFileError::FileOpen`] if target configuration file
    ///    could not be created when needed.
    /// 1. Return [`ConfigFileError::FileWrite`] if target configuration file
    ///    cannot be written into.
    pub fn save(&mut self) -> Result<(), ConfigFileError> {
        debug!("Save configuration manager data to '{}'", self.as_path().display());
        let root = self.as_path().parent().unwrap();
        mkdirp(root).map_err(|err| ConfigFileError::MakeDirP { source: err, path: root.into() })?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .read(true)
            .create(true)
            .open(self.as_path())
            .map_err(|err| ConfigFileError::FileOpen {
                source: err,
                path: self.as_path().into(),
            })?;
        let buffer = self.doc.to_string();
        file.write_all(buffer.as_bytes()).map_err(|err| ConfigFileError::FileWrite {
            source: err,
            path: self.as_path().into(),
        })?;

        Ok(())
    }

    /// Get configuration entry in deserialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be deserialized.
    pub fn get(&self, key: impl AsRef<str>) -> Result<C::Entry, ConfigFileError> {
        self.config
            .get(&self.doc, key.as_ref())
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    /// Add new configuration entry in serialized form.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be serialized.
    pub fn add(&mut self, entry: C::Entry) -> Result<Option<C::Entry>, ConfigFileError> {
        self.config
            .add(&mut self.doc, entry)
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    /// Rename configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be renamed.
    pub fn rename(
        &mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> Result<C::Entry, ConfigFileError> {
        self.config
            .rename(&mut self.doc, from.as_ref(), to.as_ref())
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    /// Remove configuration entry.
    ///
    /// # Errors
    ///
    /// 1. Return [`ConfigFileError::Toml`] if entry cannot be removed.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Result<C::Entry, ConfigFileError> {
        self.config
            .remove(&mut self.doc, key.as_ref())
            .map_err(|err| ConfigFileError::Toml { source: err, path: self.as_path().into() })
    }

    pub fn as_path(&self) -> &Path {
        self.config.location(self.locator)
    }
}

impl<'cfg, C, L> fmt::Display for ConfigFile<'cfg, C, L>
where
    C: Config,
    L: Locator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.doc)
    }
}

/// TOML serialization and deserialization configuration.
///
/// Interface to simplify serialization and deserialization of parsed TOML data.
///
/// # See also
///
/// - [`Toml`]
pub trait Config: fmt::Debug {
    type Entry: Settings;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError>;
    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError>;
    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError>;
    fn location<'cfg>(&self, locator: &'cfg impl Locator) -> &'cfg Path;
}

/// Repository data configuration management.
///
/// Handles serialization and deserialization of repository settings.
/// Repository settings are held within the "repos" section of a
/// configuration file.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
/// - [`RepoSettings`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RepoConfig;

impl Config for RepoConfig {
    type Entry = RepoSettings;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.get("repos", key.as_ref())?;
        Ok(RepoSettings::from(entry))
    }

    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError> {
        let entry = doc.add("repos", entry.to_toml())?.map(RepoSettings::from);
        Ok(entry)
    }

    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.remove("repos", key.as_ref())?;
        Ok(RepoSettings::from(entry))
    }

    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.rename("repos", from.as_ref(), to.as_ref())?;
        Ok(RepoSettings::from(entry))
    }

    fn location<'cfg>(&self, locator: &'cfg impl Locator) -> &'cfg Path {
        locator.repos_config()
    }
}

/// Command hook configuration management.
///
/// Handles serialization and deserialization of command hook settings.
/// Command hook settings are held within the "hooks" section of a
/// configuration file.
///
/// # Invariants
///
/// Will preserve existing formatting of configuration file if any.
///
/// # See also
///
/// - [`Toml`]
/// - [`CmdHookSettings`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CmdHookConfig;

impl Config for CmdHookConfig {
    type Entry = CmdHookSettings;

    fn get(&self, doc: &Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.get("hooks", key.as_ref())?;
        Ok(CmdHookSettings::from(entry))
    }

    fn add(&self, doc: &mut Toml, entry: Self::Entry) -> Result<Option<Self::Entry>, TomlError> {
        let entry = doc.add("hooks", entry.to_toml())?.map(CmdHookSettings::from);
        Ok(entry)
    }

    fn remove(&self, doc: &mut Toml, key: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.remove("hooks", key.as_ref())?;
        Ok(CmdHookSettings::from(entry))
    }

    fn rename(&self, doc: &mut Toml, from: &str, to: &str) -> Result<Self::Entry, TomlError> {
        let entry = doc.rename("hooks", from.as_ref(), to.as_ref())?;
        Ok(CmdHookSettings::from(entry))
    }

    fn location<'cfg>(&self, locator: &'cfg impl Locator) -> &'cfg Path {
        locator.hooks_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        locate::MockLocator,
        testenv::{FileKind, FixtureHarness},
    };

    use anyhow::Result;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn config_dir() -> Result<FixtureHarness> {
        let harness = FixtureHarness::open()?
            .with_file("config.toml", |fixture| {
                fixture
                    .with_data(indoc! {r#"
                        # Formatting should remain the same!

                        [repos.vim]
                        branch = "master"
                        remote = "origin"
                        workdir_home = true

                        [hooks]
                        bootstrap = [
                            { pre = "hook.sh", post = "hook.sh", workdir = "/some/dir" },
                            { pre = "hook.sh" }
                        ]
                    "#})
                    .with_kind(FileKind::Normal)
            })
            .with_file("not_table.toml", |fixture| {
                fixture
                    .with_data(indoc! {r#"
                        repos = 'not a table'
                        hooks = 'not a table'
                    "#})
                    .with_kind(FileKind::Normal)
            })
            .with_file("bad_format.toml", |fixture| {
                fixture.with_data("this 'will fail!").with_kind(FileKind::Normal)
            })
            .setup()?;
        Ok(harness)
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_load_parse_file(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_file("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let config = ConfigFile::load(config_kind, &locator)?;
        assert_eq!(config.to_string(), fixture.as_str());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::hook_cmd_config(CmdHookConfig)]
    fn config_file_load_create_new_file(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(config_dir.as_path().join("repos.toml"));
        locator.expect_hooks_config().return_const(config_dir.as_path().join("hooks.toml"));

        let config = ConfigFile::load(config_kind, &locator)?;
        assert!(config.as_path().exists());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_load_return_err_toml(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_file("bad_format.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let result = ConfigFile::load(config_kind, &locator);
        assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

        Ok(())
    }

    #[rstest]
    #[case::repo_config(
        RepoConfig,
        RepoSettings::new("dwm").branch("main").remote("upstream").workdir_home(true),
    )]
    #[case::cmd_hook_config(
        CmdHookConfig,
        CmdHookSettings::new("commit").add_hook(HookSettings::new().post("hook.sh")),
    )]
    fn config_file_save_preserves_formatting<E, T>(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: T,
        #[case] expect: E,
    ) -> Result<()>
    where
        E: Settings,
        T: Config<Entry = E>,
    {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_file_mut("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        config.add(expect)?;
        config.save()?;
        fixture.sync()?;
        assert_eq!(config.to_string(), fixture.as_str());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_save_create_new_file(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(config_dir.as_path().join("repos.toml"));
        locator.expect_hooks_config().return_const(config_dir.as_path().join("hooks.toml"));

        let mut config = ConfigFile::load(config_kind, &locator)?;
        config.save()?;
        assert!(config.as_path().exists());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(
        RepoConfig,
        "vim",
        RepoSettings::new("vim").branch("master").remote("origin").workdir_home(true),
    )]
    #[case::repo_config(
        CmdHookConfig,
        "bootstrap",
        CmdHookSettings::new("bootstrap")
            .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
            .add_hook(HookSettings::new().pre("hook.sh")),
    )]
    fn config_file_get_return_setting<E, T>(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: T,
        #[case] key: &str,
        #[case] expect: E,
    ) -> Result<()>
    where
        E: Settings,
        T: Config<Entry = E>,
    {
        let config_dir = config_dir?;
        let fixture = config_dir.get_file("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let config = ConfigFile::load(config_kind, &locator)?;
        let result = config.get(key)?;
        assert_eq!(result, expect);

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_get_return_err_toml(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_file("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let config = ConfigFile::load(config_kind, &locator)?;
        let result = config.get("non-existent");
        assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

        Ok(())
    }

    #[rstest]
    #[case::repo_config(
        RepoConfig,
        RepoSettings::new("dwm").branch("main").remote("upstream").workdir_home(true),
    )]
    #[case::cmd_hook_config(
        CmdHookConfig,
        CmdHookSettings::new("commit").add_hook(HookSettings::new().post("hook.sh")),
    )]
    fn config_file_new_return_none<E, T>(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: T,
        #[case] entry: E,
    ) -> Result<()>
    where
        E: Settings,
        T: Config<Entry = E>,
    {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_file_mut("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.add(entry)?;
        config.save()?;
        fixture.sync()?;
        assert_eq!(result, None);
        assert_eq!(config.to_string(), fixture.as_str());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(
        RepoConfig,
        RepoSettings::new("vim").branch("main").remote("upstream").workdir_home(false),
        Some(RepoSettings::new("vim").branch("master").remote("origin").workdir_home(true)),
    )]
    #[case::cmd_hook_config(
        CmdHookConfig,
        CmdHookSettings::new("bootstrap")
            .add_hook(HookSettings::new().pre("new_hook.sh").post("new_hook.sh"))
            .add_hook(HookSettings::new().pre("new_hook.sh").workdir("/new/dir")),
        Some(CmdHookSettings::new("bootstrap")
            .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
            .add_hook(HookSettings::new().pre("hook.sh"))),
    )]
    fn config_file_new_return_some<E, T>(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: T,
        #[case] entry: E,
        #[case] expect: Option<E>,
    ) -> Result<()>
    where
        E: Settings,
        T: Config<Entry = E>,
    {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_file_mut("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.add(entry)?;
        config.save()?;
        fixture.sync()?;
        assert_eq!(result, expect);
        assert_eq!(config.to_string(), fixture.as_str());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_add_return_err_toml(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_file_mut("not_table.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.add(Default::default());
        assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));
        Ok(())
    }

    #[rstest]
    #[case::repo_config(
        RepoConfig,
        "vim",
        "neovim",
        RepoSettings::new("vim").branch("master").remote("origin").workdir_home(true),
    )]
    #[case::cmd_hook_config(
        CmdHookConfig,
        "bootstrap",
        "commit",
        CmdHookSettings::new("bootstrap")
            .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
            .add_hook(HookSettings::new().pre("hook.sh")),
    )]
    fn config_file_rename_return_old_setting<E, T>(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: T,
        #[case] from: &str,
        #[case] to: &str,
        #[case] expect: E,
    ) -> Result<()>
    where
        E: Settings,
        T: Config<Entry = E>,
    {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_file_mut("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.rename(from, to)?;
        config.save()?;
        fixture.sync()?;
        assert_eq!(result, expect);
        assert_eq!(config.to_string(), fixture.as_str());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_rename_return_err_toml(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_file("not_table.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.rename("gonna", "fail");
        assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

        Ok(())
    }

    #[rstest]
    #[case::repo_config(
        RepoConfig,
        "vim",
        RepoSettings::new("vim").branch("master").remote("origin").workdir_home(true),
    )]
    #[case::cmd_hook_config(
        CmdHookConfig,
        "bootstrap",
        CmdHookSettings::new("bootstrap")
            .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/dir"))
            .add_hook(HookSettings::new().pre("hook.sh")),
    )]
    fn config_file_remove_return_deleted_setting<E, T>(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: T,
        #[case] key: &str,
        #[case] expect: E,
    ) -> Result<()>
    where
        E: Settings,
        T: Config<Entry = E>,
    {
        let mut config_dir = config_dir?;
        let fixture = config_dir.get_file_mut("config.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.remove(key)?;
        config.save()?;
        fixture.sync()?;
        assert_eq!(result, expect);
        assert_eq!(config.to_string(), fixture.as_str());

        Ok(())
    }

    #[rstest]
    #[case::repo_config(RepoConfig)]
    #[case::cmd_hook_config(CmdHookConfig)]
    fn config_file_remove_return_err_toml(
        config_dir: Result<FixtureHarness>,
        #[case] config_kind: impl Config,
    ) -> Result<()> {
        let config_dir = config_dir?;
        let fixture = config_dir.get_file("not_table.toml")?;
        let mut locator = MockLocator::new();
        locator.expect_repos_config().return_const(fixture.as_path().into());
        locator.expect_hooks_config().return_const(fixture.as_path().into());

        let mut config = ConfigFile::load(config_kind, &locator)?;
        let result = config.remove("fail");
        assert!(matches!(result.unwrap_err(), ConfigFileError::Toml { .. }));

        Ok(())
    }
}
