use std::{env, fs, process::Command};

use anyhow::{bail, Result};
use log::trace;

use crate::config::ScratchPackage;

#[derive(Debug)]
pub struct Scratch;

impl Scratch {
    pub fn install(&self, pkg: &ScratchPackage) -> Result<()> {
        self.run_script(pkg)?;
        Ok(())
    }

    fn run_script(&self, pkg: &ScratchPackage) -> Result<()> {
        let Some(script) = &pkg.script else {
            return Ok(());
        };

        let workdir =
            env::temp_dir().join(format!("mehr2-{}-{}", pkg.identifier, std::process::id()));
        fs::create_dir_all(&workdir)?;

        trace!("found script, dumping the installation into a script");
        fs::write(workdir.join("install.script"), script)?;

        let Some(shell) = env::var_os("SHELL") else {
            bail!("SHELL env var not found");
        };

        trace!(
            "executing {} with {}",
            &workdir.join("install.script").display(),
            shell.display()
        );

        let status = Command::new(&shell)
            .arg("install.script")
            .current_dir(&workdir)
            .status()?;

        if !status.success() {
            bail!("scratch package `{}` failed with {status}", pkg.identifier);
        }

        Ok(())
    }
}
