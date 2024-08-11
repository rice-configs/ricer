// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Locate configuration directory.
//!
//! Basic way to locate Ricer's configuration directory at an expected area in
//! the user's environment. This logic remains seperate from configuration
//! directory management logic to make it easier to change the expected location
//! of Ricer's configuration directory at any time if need be. By default Ricer
//! expects its configuration directory to be at `$XDG_CONFIG_HOME/ricer`, i.e.,
//! `$HOME/.config/ricer`.

use anyhow::anyhow;
use directories::BaseDirs;
use log::{debug, trace, warn};
use std::fs::create_dir;
use std::path::{Path, PathBuf};

#[cfg(test)]
use mockall::automock;

use crate::error::{RicerError, RicerResult};

/// Configuration directory locator representation.
#[cfg_attr(test, automock)]
pub trait ConfigDirLocator {
    /// Provide absolute path to located configuration directory.
    fn config_dir(&self) -> &Path;
}

/// [XDG Base Directory][xdg-base-dir-spec] representation.
///
/// [xdg-base-dir-spec]: https://wiki.archlinux.org/title/XDG_Base_Directory
#[cfg_attr(test, automock)]
pub trait XdgBaseDirSpec {
    /// Absolute path of `$XDG_CONFIG_HOME`.
    fn config_home_dir(&self) -> &Path;
}

/// Default implementation of [`XdgBaseDirSpec`].
pub struct DefaultXdgBaseDirSpec {
    xdg_spec: BaseDirs,
}

impl DefaultXdgBaseDirSpec {
    /// Construct new instance of default XDG Base Directory Specification
    /// handler.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return Default XDG Base Directory Specification handler instance.
    ///
    /// # Invariants
    ///
    /// None.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// # Errors
    ///
    /// Issues [`RicerError::Unrecoverable`] if it cannot determine home
    /// directory of user.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, XdgBaseDirSpec};
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// println!("{}", xdg_spec.config_home_dir().display());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # See
    ///
    /// - <https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.new>
    pub fn new() -> RicerResult<Self> {
        trace!("Construct default XDG Base Directory Specification handler");
        let xdg_spec = BaseDirs::new().ok_or(RicerError::Unrecoverable(anyhow!(
            "Failed to locate configuration directory"
        )))?;

        Ok(Self { xdg_spec })
    }
}

impl XdgBaseDirSpec for DefaultXdgBaseDirSpec {
    /// Get path of `$XDG_CONFIG_HOME`.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return path of `$XDG_CONFIG_HOME`.
    ///
    /// # Invariants
    ///
    /// 1. Returned path is guaranteed to be absolute.
    ///
    /// # Side Effects
    ///
    /// None.
    fn config_home_dir(&self) -> &Path {
        let path = self.xdg_spec.config_dir();
        debug_assert!(path.is_absolute(), "Path of $XDG_CONFIG_HOME is not absolute");
        path
    }
}

/// Configuration directory locatory following XDG base directory specification.
///
/// Attempts to locate Ricer's configuration directory using an implementation
/// of [`XdgBaseDirSpec`]. Expects the configuration directory to be at
/// `$XDG_CONFIG_HOME/ricer`.
///
/// # Invariants
///
/// 1. Locator provides an absolute path to configuration directory.
pub struct DefaultConfigDirLocator {
    config_dir: PathBuf,
}

impl DefaultConfigDirLocator {
    /// Construct new default configuration directory locator.
    ///
    /// This method will construct a new instance of [`DefaultConfigDirLocator`]
    /// that automatically locates the expected absolute path to Ricer's
    /// configuration directory at `$XDG_CONFIG_HOME/ricer` using an
    /// implementation of [`XdgBaseDirSpec`].
    ///
    /// # Preconditions
    ///
    /// 1. Valid instance of [`XdgBaseDirSpec`].
    ///
    /// # Postconditions
    ///
    /// 1. Return new default configuration directory locator instance.
    ///
    /// # Invariants
    ///
    /// 1. Located path to configuration directory is absolute.
    ///
    /// # Side Effects
    ///
    /// None.
    ///
    /// # Errors
    ///
    /// Issues [`RicerError::NoConfigDir`] if configuration directory does not
    /// exist at `$XDG_CONFIG_HOME/ricer`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::{DefaultXdgBaseDirSpec, DefaultConfigDirLocator};
    /// use ricer::error::RicerError;
    ///
    /// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
    /// let locator = match DefaultConfigDirLocator::new_locate(&xdg_spec) {
    ///     Ok(locator) => locator,
    ///     Err(RicerError::NoConfigDir(..)) => {
    ///         // TODO: Recovery logic...
    ///         todo!();
    ///     }
    ///     Err(err) => return Err(err.into()),
    /// };
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`RicerError::NoConfigDir`]: crate::error::RicerError::NoConfigDir
    pub fn new_locate(xdg_spec: &dyn XdgBaseDirSpec) -> RicerResult<Self> {
        trace!("Construct default configuration directory locator");
        let config_dir = xdg_spec.config_home_dir().join("ricer");
        debug_assert!(config_dir.is_absolute(), "Path to $XDG_CONFIG_HOME/ricer is not absolute");
        if !config_dir.exists() {
            return Err(RicerError::NoConfigDir(anyhow!(
                "Expected configuration directory at '{}'",
                config_dir.display()
            )));
        }

        debug!("Configuration directory located at '{}'", config_dir.display());
        Ok(Self { config_dir })
    }
}

impl ConfigDirLocator for DefaultConfigDirLocator {
    /// Get located path to configuration directory.
    ///
    /// # Preconditions
    ///
    /// None.
    ///
    /// # Postconditions
    ///
    /// 1. Return path of `$XDG_CONFIG_HOME/ricer`.
    ///
    /// # Invariants
    ///
    /// 1. Returned path is guaranteed to be absolute.
    ///
    /// # Side Effects
    ///
    /// None.
    fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }
}

/// Recovery logic for [`DefaultConfigDirLocator`].
///
/// Creates the configuration directory at expected location provided by
/// [`XdgBaseDirSpec`] as a recovery strategy.
///
/// # Preconditions
///
/// 1. Valid instance of [`XdgBaseDirSpec`].
///
/// # Postconditions
///
/// 1. Return instance of [`DefaultConfigDirLocator`].
///
/// # Invariants
///
/// None.
///
/// # Side Effects
///
/// None.
///
/// # Errors
///
/// Cannot create the configuration directory at expected path.
///
/// # Examples
///
/// ```no_run
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// use ricer::config::locator::{
///     recover_default_config_dir_locator, DefaultXdgBaseDirSpec, DefaultConfigDirLocator
/// };
/// use ricer::error::RicerError;
///
/// let xdg_spec = DefaultXdgBaseDirSpec::new()?;
/// let locator = match DefaultConfigDirLocator::new_locate(&xdg_spec) {
///     Ok(locator) => locator,
///     Err(RicerError::NoConfigDir(..)) => recover_default_config_dir_locator(&xdg_spec)?,
///     Err(err) => return Err(err.into()),
/// };
/// # Ok(())
/// # }
/// ```
///
/// # See
///
/// - <https://doc.rust-lang.org/std/fs/fn.create_dir.html#errors>
///
/// [`RicerError::NoConfigDir`]: crate::error::RicerError::NoConfigDir
pub fn recover_default_config_dir_locator(
    xdg_spec: &dyn XdgBaseDirSpec,
) -> RicerResult<DefaultConfigDirLocator> {
    let config_dir = xdg_spec.config_home_dir().join("ricer");

    warn!("Creating configuration directory since it does not exist at '{}'", config_dir.display());
    create_dir(config_dir)?;

    let locator = DefaultConfigDirLocator::new_locate(xdg_spec)?;
    Ok(locator)
}
