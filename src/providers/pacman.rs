use super::{Package, PackageManager};
use std::{path::PathBuf, process::Command};

#[derive(Debug)]
pub struct Pacman;

impl PackageManager for Pacman {
    fn upgrade(&self, packages: &[Package]) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .args(packages)
            .status()
            .map(|_| {})?)
    }

    fn install(&self, packages: &[Package]) -> anyhow::Result<()> {
        Pacman.upgrade(packages)
    }

    fn update(&self) -> anyhow::Result<()> {
        Ok(Command::new("sudo")
            .arg("pacman")
            .arg("-Sy")
            .status()
            .map(|_| {})?)
    }

    fn is_installed(&self, paths: &[PathBuf], package: &Package) -> anyhow::Result<bool> {
        let status = Command::new("pacman").arg("-Q").arg(package).output()?;
        Ok(status.status.success())
    }
}
