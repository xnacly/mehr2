use std::{env, fs, process::Command};

use anyhow::{bail, Result};
use log::{info, trace};

use crate::config::ScratchPackage;
use crate::{fs as mehrfs, Args};

#[derive(Debug)]
pub struct Scratch;

impl Scratch {
    pub fn install(&self, args: &Args, pkg: &ScratchPackage) -> Result<()> {
        self.run_script(args, pkg)?;
        Ok(())
    }

    fn run_script(&self, args: &Args, pkg: &ScratchPackage) -> Result<()> {
        let Some(script) = &pkg.script else {
            return Ok(());
        };

        let identifier = &pkg.identifier;

        if let Some(dependencies) = &pkg.needs {
            for dep in dependencies {
                let Some(path) = mehrfs::has_binary(&args._system_path, dep) else {
                    bail!("{dep} not found in PATH, but required by scratch-{identifier}");
                };

                if !mehrfs::binary_is_executable(&path.to_string_lossy()) {
                    bail!(
                        "{} not executable, but required by scratch-{identifier}",
                        &path.display()
                    );
                }

                trace!(
                    "[scratch-{identifier}] {}({}) exists and is executable",
                    dep,
                    path.display()
                );
            }
        }

        let Some(shell) = env::var_os("SHELL") else {
            bail!("SHELL env var not found, but required to install scratch packages");
        };

        if args.dry {
            info!(
                "[scratch-{identifier}] dry: would run script for `{}` with {}: \n{}",
                pkg.identifier,
                shell.display(),
                script,
            );
            return Ok(());
        }

        let workdir =
            env::temp_dir().join(format!("mehr2-{}-{}", pkg.identifier, std::process::id()));
        fs::create_dir_all(&workdir)?;

        trace!("[scratch-{identifier}] found script, dumping the installation into a script");
        fs::write(workdir.join("install.script"), script)?;

        let script_path = workdir.join("install.script");

        trace!(
            "[scratch-{identifier}] executing {} with {}",
            script_path.display(),
            shell.display()
        );

        let status = Command::new(&shell)
            .arg("install.script")
            .current_dir(&workdir)
            .status()?;

        if !status.success() {
            bail!("[scratch-{identifier}] failed with {status}",);
        }

        Ok(())
    }
}
