use crate::{
    config::{self},
    lock,
};
use anyhow::Result;
use std::path::PathBuf;

mod cargo;
mod pacman;

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

pub fn from_name(name: &str) -> Option<Box<dyn PackageManager>> {
    Some(match name {
        "pacman" => Box::new(pacman::Pacman {}),
        "cargo" => Box::new(cargo::Cargo),
        "go" => todo!(),
        _ => return None,
    })
}
