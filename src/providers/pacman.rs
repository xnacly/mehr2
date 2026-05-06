use super::{Package, PackageManager};
use crate::Args;
use anyhow::bail;
use log::info;
use std::{path::PathBuf, process::Command};

#[derive(Debug)]
pub struct Pacman;

impl PackageManager for Pacman {
    fn upgrade(&self, args: &Args, packages: &[Package]) -> anyhow::Result<()> {
        if args.dry {
            info!(
                "[pacman] dry: would run sudo pacman -S {}",
                packages.join(" ")
            );
            return Ok(());
        }

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

    fn install(&self, args: &Args, packages: &[Package]) -> anyhow::Result<()> {
        self.upgrade(args, packages)
    }

    fn update(&self, args: &Args) -> anyhow::Result<()> {
        if args.dry {
            info!("[pacman] dry: would run sudo pacman -Sy");
            return Ok(());
        }

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
