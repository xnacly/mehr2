use super::{Package, PackageManager};
use std::{path::PathBuf, process::Command};

#[derive(Debug)]
pub struct Cargo;

impl PackageManager for Cargo {
    fn upgrade(&self, packages: &[super::Package]) -> anyhow::Result<()> {
        // cargo does not support upgrading specific packages, so we have to reinstall
        for pkg in packages {
            Command::new("cargo")
                .arg("install")
                .arg(pkg)
                .arg("--force")
                .status()?;
        }
        Ok(())
    }

    fn install(&self, packages: &[Package]) -> anyhow::Result<()> {
        self.upgrade(packages)
    }

    fn update(&self) -> anyhow::Result<()> {
        Command::new("cargo").arg("update").status()?;
        Ok(())
    }

    fn is_installed(&self, paths: &[PathBuf], package: &Package) -> anyhow::Result<bool> {
        // binary exists in PATH or cargo bin dir
        if let Some(path) = super::has_binary(paths, package) {
            return Ok(path.to_string_lossy().contains(".cargo/bin") || true);
        }

        Ok(false)
    }
}
