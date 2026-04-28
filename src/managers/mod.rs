use std::process::Command;

use anyhow::Result;

mod pacman;

use log::{info, trace, warn};

use crate::{
    config::{self, Packages},
    lock,
};

pub fn process_packages(config: config::Config, lock: lock::Lock) -> Result<lock::Lock> {
    let mut result_lock = lock::Lock::default();
    let diff = lock.diff(&config);
    if let Some(packages) = diff.get("default") {
        if let Some(system_manager) = default() {
            info!(
                "using default package manager to install {} package(s): \n{}",
                packages.len(),
                packages.join("\n"),
            );
            system_manager.update()?;
            system_manager.install(packages)?;
            result_lock
                .packages
                .insert("default".to_string(), packages.to_vec());
        } else {
            warn!("could not determine a default package manager, doing nothing for 'default' packages...")
        }
    }

    // TODO: match specific package managers here

    Ok(dbg!(result_lock))
}

type Package = String;

pub trait PackageManager: std::fmt::Debug {
    fn upgrade(&self, packages: &[Package]) -> Result<()>;
    fn install(&self, packages: &[Package]) -> Result<()>;
    /// update syncs the current package manager to its repos
    fn update(&self) -> Result<()>;
}

fn binary_is_executable(name: &str) -> bool {
    Command::new(name).output().is_ok()
}

/// default returns the default package manager for the given system
pub fn default() -> Option<Box<dyn PackageManager>> {
    trace!("querying system for default package manager...");
    let system_level = ["pacman", "apt"];
    for name in system_level {
        if binary_is_executable(name) {
            info!("found default='{name}'");
            return from_name(name);
        }
    }
    None
}

pub fn from_name(name: &str) -> Option<Box<dyn PackageManager>> {
    Some(match name {
        "pacman" => Box::new(pacman::Pacman {}),
        "cargo" => todo!(),
        "go" => todo!(),
        _ => return None,
    })
}
