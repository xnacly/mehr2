use crate::Args;
use anyhow::Result;
use std::path::PathBuf;

mod cargo;
mod pacman;
mod scratch;

pub use scratch::Scratch;

type Package = String;

pub trait PackageManager: std::fmt::Debug {
    fn upgrade(&self, args: &Args, packages: &[Package]) -> Result<()>;
    fn install(&self, args: &Args, packages: &[Package]) -> Result<()>;
    fn update(&self, args: &Args) -> Result<()>;
    fn is_installed(&self, paths: &[PathBuf], package: &Package) -> Result<bool>;
}

fn make_pacman() -> Box<dyn PackageManager> {
    Box::new(pacman::Pacman)
}

fn make_cargo() -> Box<dyn PackageManager> {
    Box::new(cargo::Cargo)
}

/// Single source of truth for supported provider names.
/// `None` ctor = special-cased dispatch (currently only "scratch").
const ENTRIES: &[(&str, Option<fn() -> Box<dyn PackageManager>>)] = &[
    ("pacman", Some(make_pacman)),
    ("cargo", Some(make_cargo)),
    ("scratch", None),
];

pub fn from_name(name: &str) -> Option<Box<dyn PackageManager>> {
    ENTRIES
        .iter()
        .find(|(n, _)| *n == name)
        .and_then(|(_, ctor)| ctor.map(|f| f()))
}

pub fn is_valid_name(name: &str) -> bool {
    ENTRIES.iter().any(|(n, _)| *n == name)
}

pub fn names() -> impl Iterator<Item = &'static str> {
    ENTRIES.iter().map(|(name, _)| *name)
}
