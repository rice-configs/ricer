// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: GPL-2.0-or-later WITH GPL-CC-1.0

//! Locate configuration directory.
//!
//! Basic way to locate Ricer's configuration directory at an expected area in
//! the user's environment. This logic remains seperate from configuration
//! directory management logic to make it easier to change the expected location
//! of Ricer's configuration directory at any time if need be.
//!
//! By default Ricer expects its configuration directory to be at
//! `$XDG_CONFIG_HOME/ricer`, i.e., `$HOME/.config/ricer`. Thus, the
//! [`XdgConfigDirLocator`] handler is expected to be used. However, if the
//! location of the configuration directory needs to change for whatever reason,
//! then simply implement a new locator with [`ConfigDirLocator`] trait.

use anyhow::anyhow;
use directories::ProjectDirs;
use log::{debug, trace, warn};
use std::fs::create_dir;
use std::path::Path;

use crate::error::{RicerError, RicerResult};

/// Configuration directory locator representation.
pub trait ConfigDirLocator {
    /// Provide absolute path to located configuration directory.
    fn config_dir(&self) -> &Path;
}

/// Configuration directory locatory following XDG base directory specification.
///
/// Attempts to locate Ricer's configuration directory at
/// `$XDG_CONFIG_HOME/ricer` following the XDG base directory specification.
pub struct XdgConfigDirLocator {
    xdg_spec: ProjectDirs,
}

impl XdgConfigDirLocator {
    /// Construct new [`XdgConfigDirLocator`] that tries to locate configuration
    /// directory and ensures that it exists.
    ///
    /// This method will construct a new instance of [`XdgConfigDirLocator`]
    /// that automatically locates the expected absolute path to Ricer's
    /// configuration directory at `$XDG_CONFIG_HOME/ricer`. It will also check
    /// that the configuration directory exists at that expected absolute path.
    ///
    /// # Errors
    ///
    /// 1. Issues [`RicerError::Unrecoverable`] if expected absolute path to
    ///    configuration directory cannot be determined in user's OS
    ///    environment.
    /// 2. Issues [`RicerError::NoConfigDir`] if expected absolute path to
    ///    configuration directory can be determined, but the configuration
    ///    directory itself does not exist at the expected location.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// use ricer::config::locator::XdgConfigDirLocator;
    /// use ricer::error::RicerError;
    ///
    /// let locator = match XdgConfigLocator::try_new_locate() {
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
    /// # See
    ///
    /// - <https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.new>
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    /// [`RicerError::NoConfigDir`]: crate::error::RicerError::NoConfigDir
    pub fn try_new_locate_exists() -> RicerResult<Self> {
        let locator = Self::try_new_locate()?;
        if !locator.config_dir().exists() {
            return Err(RicerError::NoConfigDir(anyhow!(
                "Expected configuration directory at '{}'",
                locator.config_dir().display()
            )));
        }

        debug!("Configuration directory located at '{}'", locator.config_dir().display());
        Ok(locator)
    }

    /// Construct new [`XdgConfigDirLocator`] that tries to locate configuration
    /// directory only.
    ///
    /// This method will construct a new instance of [`XdgConfigDirLocator`]
    /// that automatically locates the expected absolute path to Ricer's
    /// configuration directory at `$XDG_CONFIG_HOME/ricer`.
    ///
    /// This method is mainly used at the crate level to help implement basic
    /// recovery logic for [`try_new_locate_exists`]. See
    /// [`recover_xdg_config_dir_locator`] for example usage.
    ///
    /// # Errors
    ///
    /// Issues [`RicerError::Unrecoverable`] if expected absolute path to
    /// configuration directory cannot be determined in user's OS environment.
    ///
    /// # See
    ///
    /// - <https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.new>
    ///
    /// [`RicerError::Unrecoverable`]: crate::error::RicerError::Unrecoverable
    /// [`try_new_locate_exists`]: #method.try_new_locate_exists
    pub(crate) fn try_new_locate() -> RicerResult<Self> {
        trace!("Locate expected path to configuration directory");
        let xdg_spec = ProjectDirs::from("com", "awkless", "ricer").ok_or(
            RicerError::Unrecoverable(anyhow!("Failed to locate configuration directory")),
        )?;

        Ok(Self { xdg_spec })
    }
}

impl ConfigDirLocator for XdgConfigDirLocator {
    fn config_dir(&self) -> &Path {
        self.xdg_spec.config_dir()
    }
}

/// Recovery logic for [`XdgConfigDirLocator::try_new_locate_exists`] issuing
/// a [`RicerError::NoConfigDir`] error.
///
/// Creates the configuration directory at expected location provided by
/// [`XdgConfigDirLocator`] as a recovery strategy.
///
/// # Errors
///
/// Cannot determine absolute path to configuration directory.
///
/// # Examples
///
/// ```no_run
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// use ricer::config::locator::{recover_xdg_config_dir_locator, XdgConfigDirLocator};
/// use ricer::error::RicerError;
///
/// let locator = match XdgConfigLocator::try_new_locate() {
///     Ok(locator) => locator,
///     Err(RicerError::NoConfigDir(..)) => recover_xdg_config_dir_locator()?,
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
pub fn recover_xdg_config_dir_locator() -> RicerResult<XdgConfigDirLocator> {
    let locator = XdgConfigDirLocator::try_new_locate()?;
    warn!("Create configuration directory at '{}'", locator.config_dir().display());
    create_dir(locator.config_dir())?;
    Ok(locator)
}
