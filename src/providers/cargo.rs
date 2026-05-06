use crate::fs;
use crate::Args;

use super::{Package, PackageManager};
use anyhow::bail;
use log::info;
use std::{path::PathBuf, process::Command};

#[derive(Debug)]
pub struct Cargo;

impl PackageManager for Cargo {
    fn upgrade(&self, args: &Args, packages: &[super::Package]) -> anyhow::Result<()> {
        // cargo does not support upgrading specific packages, so we have to reinstall
        for pkg in packages {
            if args.dry {
                info!("[cargo] dry: would run cargo install {pkg} --force");
                continue;
            }

            let status = Command::new("cargo")
                .arg("install")
                .arg(pkg)
                .arg("--force")
                .status()?;

            if !status.success() {
                bail!("cargo failed to install `{pkg}` with status {status}");
            }
        }
        Ok(())
    }

    fn install(&self, args: &Args, packages: &[Package]) -> anyhow::Result<()> {
        self.upgrade(args, packages)
    }

    fn update(&self, _args: &Args) -> anyhow::Result<()> {
        // Installed cargo binaries are updated by reinstalling with `cargo install --force`.
        Ok(())
    }

    fn is_installed(&self, paths: &[PathBuf], package: &Package) -> anyhow::Result<bool> {
        // binary exists in PATH or cargo bin dir
        if let Some(path) = fs::has_binary(paths, package) {
            return Ok(path.to_string_lossy().contains(".cargo/bin") || true);
        }

        Ok(false)
    }
}
