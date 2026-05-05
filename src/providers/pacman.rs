use super::{Package, PackageManager};
use anyhow::bail;
use std::{path::PathBuf, process::Command};

#[derive(Debug)]
pub struct Pacman;

impl PackageManager for Pacman {
    fn upgrade(&self, packages: &[Package]) -> anyhow::Result<()> {
        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .args(packages)
            .status()?;

        if !status.success() {
            bail!("pacman failed to upgrade packages with status {status}");
        }

        Ok(())
    }

    fn install(&self, packages: &[Package]) -> anyhow::Result<()> {
        Pacman.upgrade(packages)
    }

    fn update(&self) -> anyhow::Result<()> {
        let status = Command::new("sudo").arg("pacman").arg("-Sy").status()?;

        if !status.success() {
            bail!("pacman failed to update package databases with status {status}");
        }

        Ok(())
    }

    fn is_installed(&self, paths: &[PathBuf], package: &Package) -> anyhow::Result<bool> {
        let status = Command::new("pacman").arg("-Q").arg(package).output()?;
        Ok(status.status.success())
    }
}
