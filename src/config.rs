// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Configuration data management.
//!
//! This module is responsible for providing an interface for managing Ricer's
//! configuration data.
//!
//! Ricer uses `$XDG_CONFIG_HOME/ricer` to house all of its configuration
//! information. The following represents the overall structure of this
//! directory:
//!
//! ```markdown
//! $XDG_CONFIG_HOME/ricer/
//! |-- hooks/
//! |   |-- hook_script1.sh
//! |   |-- hook_script2.sh
//! |   `-- hook_scriptn.sh
//! |-- repos/
//! |   |-- config1.git/
//! |   |-- config2.git/
//! |   `-- confign.git/
//! |-- ignores/
//! |   |-- config1.ignore
//! |   |-- config2.ignore
//! |   `-- confign.ignore
//! `-- config.toml
//! ```
//!
//! The `config.toml` file is the main configuration file for Ricer. The user
//! can directly modify this configuration file through their preferred text
//! editor. However, the user can indirectly modify `config.toml` through
//! Ricer's command set, e.g., init and clone.
//!
//! The `hooks/` directory contains all scripts that the user can use as hooks
//! for Ricer's command set. Ricer _will_ only execute hooks stored in this
//! directory. Thus, the user can refer to hook scripts by name without the need
//! to provide an absolute or relative path, because Ricer will automatically
//! look in the `hooks/` directory for any hook scripts the user wants.
//!
//! > __NOTE:__ The limiting of hook scripts to the `hook/` directory is done
//! > as a very basic security measure. The hope is that the user can easily
//! > identify potentially dangerious hook scripts by centeralizing all scripts
//! > in one easily accessible location.
//!
//! The `repos/` directory contains all the cloned and initialized repositories
//! the user wants Ricer to keep track of. This directory is where Ricer finds
//! and modifies repository information the user passes in through the CLI.
//!
//! Finally, the `ignores/` directory houses exclude/ignore files that ensure
//! that each repository in `repos/` only tracks the portions of the user's
//! home directory that they are responsible for. This ensures that no
//! repository the user is tracking through Ricer attempts to track their
//! entire home directory.

use anyhow::anyhow;
use directories::ProjectDirs;
use std::path::{PathBuf, Path};

use crate::error::RicerError;

/// Configuration directory representation.
///
/// Meant to represent the layout of Ricer's configuration directory. As a trait
/// we can easily define and modify the directory layout any time without
/// much hassle.
///
/// # Preconditions
///
/// 1. Implementors of this trait should only provide _expected_ paths for each
///    trait method such that path verification should be left to [`Config`].
///
/// [`Config`]: crate::config::Config
pub trait ConfigDir {
    /// Get location of Ricer's base configuration directory.
    fn base_dir(&self) -> &Path;

    /// Get location of Ricer's hook script directory.
    fn hooks_dir(&self) -> &Path;

    /// Get location of Ricer's tracked repositories directory.
    fn repos_dir(&self) -> &Path;

    /// Get location of Ricer's set of ignore/exclude files directory.
    fn ignores_dir(&self) -> &Path;
}

/// Default configuration directory.
///
/// Meant to represent Ricer's default configuration directory in
/// `$XDG_CONFIG_HOME/ricer`. See [`crate::config`] for more details about the
/// layout of Ricer's configuration directory.
///
/// [`crate::config`]: crate::config
#[derive(Debug)]
pub struct DefaultConfigDir {
    // $XDG_CONFIG_HOME/ricer/
    base_dir: PathBuf,

    // $XDG_CONFIG_HOME/ricer/hooks/
    hooks_dir: PathBuf,

    // $XDG_CONFIG_HOME/ricer/repos/
    repos_dir: PathBuf,

    // $XDG_CONFIG_HOME/ricer/ignores/
    ignores_dir: PathBuf,
}

impl DefaultConfigDir {
    pub fn try_new() -> Result<Self, RicerError> {
        let dir = match ProjectDirs::from("com", "awkless", "ricer") {
            Some(dir) => dir,
            None => {
                return Err(RicerError::ConfigError(anyhow!(
                    "Failed to locate default configuration direcotry"
                )))
            }
        };

        let base_dir = dir.config_dir().to_path_buf();
        let hooks_dir = dir.config_dir().join("hooks/");
        let repos_dir = dir.config_dir().join("repos/");
        let ignores_dir = dir.config_dir().join("ignores/");

        Ok(Self { base_dir, hooks_dir, repos_dir, ignores_dir })
    }
}
