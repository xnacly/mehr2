use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use log::info;

/// config returns the path to the mehr configuration directory, containing the mehr2.lua
/// configuration file and the mehr2_lock.json lock file, lookup prio:
///
/// 1. $MEHR_PATH
/// 2. $XDG_CONFIG_HOME/.config/mehr2
/// 3. $HOME/.config/mehr2/
pub fn config() -> Result<PathBuf> {
    if let Ok(mehr_path) = env::var("MEHR_PATH") {
        info!("inf: MEHR_PATH found: {}", &mehr_path);
        return Ok(Path::new(&mehr_path).to_path_buf());
    }

    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        let path = Path::new(&xdg_config_home).join("mehr2").to_path_buf();
        info!("inf: Extracted path from $XDG_CONFIG_HOME: {:?}", &path);
        return Ok(path);
    }

    if let Ok(home) = env::var("HOME") {
        let path = Path::new(&home).join(".config").join("mehr2").to_path_buf();
        info!("inf: Extracted path from $HOME: {:?}", &path);
        return Ok(path);
    }

    Err(anyhow!("Wasn't able to create a path to the configuration"))
}
