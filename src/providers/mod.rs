use std::{
    env::{self, SplitPaths},
    fs,
    path::PathBuf,
    process::Command,
};

use anyhow::Result;

mod cargo;
mod pacman;

use crate::{
    config::{self, Packages},
    lock,
};

pub fn process_packages(config: config::Config, lock: lock::Lock) -> Result<lock::Lock> {
    let mut result_lock = lock::Lock::default();
    let diff = lock.diff(&config);
    todo!();
}

type Package = String;

pub trait PackageManager: std::fmt::Debug {
    fn upgrade(&self, packages: &[Package]) -> Result<()>;
    fn install(&self, packages: &[Package]) -> Result<()>;
    fn update(&self) -> Result<()>;
    fn is_installed(&self, paths: &[PathBuf], package: &Package) -> Result<bool>;
}

pub fn binary_is_executable(name: &str) -> bool {
    Command::new(name).output().is_ok()
}

pub fn has_binary(paths: &[PathBuf], name: &str) -> Option<PathBuf> {
    for path in paths {
        let full = path.join(name);
        if full.exists() && fs::metadata(&full).is_ok() {
            return Some(full);
        }
    }
    None
}

pub fn from_name(name: &str) -> Option<Box<dyn PackageManager>> {
    Some(match name {
        "pacman" => Box::new(pacman::Pacman {}),
        "cargo" => Box::new(cargo::Cargo),
        "go" => todo!(),
        _ => return None,
    })
}
