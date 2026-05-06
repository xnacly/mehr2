use anyhow::Result;
use std::path::PathBuf;

mod cargo;
mod pacman;
mod scratch;

pub use scratch::Scratch;

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
