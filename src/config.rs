use std::{collections::HashMap, fs, path::PathBuf};

use log::info;
use mlua::LuaSerdeExt;
use mlua::UserData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ScratchPackage {
    pub identifier: String,
    /// the package requires all members to be executables on the system for it to build
    pub needs: Option<Vec<String>>,
    /// inline shell script to execute when installing and updating packages
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Packages {
    /// packages that can be installed by their names
    Packages(Vec<String>),
    /// packages that have to be made from scratch via commands
    ScratchPackages(Vec<ScratchPackage>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Provider {
    pub name: String,
    pub packages: Packages,
}

#[derive(Debug, Deserialize, Serialize)]
/// the MEHR2 struct in the mehr2.lua file
pub struct Config {
    pub providers: Vec<Provider>,
}

impl Config {
    pub fn from_path_buf(lua: &mlua::Lua, path: &PathBuf) -> Result<Self, String> {
        let path_clone = path.clone();
        let path_as_str = path_clone.to_str().unwrap_or_else(|| "invalid utf8");
        info!("loading configuration...");
        let config_as_str = fs::read_to_string(path).map_err(|err| {
            format!(
                "Failed to read configuration file '{}': {}",
                path_as_str, err
            )
        })?;

        lua.load(config_as_str)
            .set_name(path_as_str.to_string())
            .exec()
            .map_err(|err| format!("{}: {}", path_as_str, err))?;

        let raw_conf = lua
            .globals()
            .get::<mlua::Value>("MEHR2")
            .map_err(|err| format!("{}: {}", path_as_str, err))?;

        if raw_conf.is_nil() {
            return Err(format!(
                "{}: MEHR2 table is missing from configuration",
                path_as_str
            ));
        }

        lua.from_value(raw_conf)
            .map_err(|err| format!("{}: {}", path_as_str, err))
    }
}

impl UserData for Config {}
