use anyhow::anyhow;
use log::info;
use std::path::PathBuf;

use crate::{
    config::{self, Provider},
    lock, providers, Args,
};

pub fn sync(
    args: &Args,
    paths: &[PathBuf],
    conf: &config::Config,
    lock: &lock::Lock,
) -> anyhow::Result<()> {
    let diff = lock.diff(conf);

    if diff.values().map(|l| l.len()).sum::<usize>() == 0 {
        info!("nothing to do, empty diff, exiting...")
    }

    for Provider { name, .. } in &conf.providers {
        if name == "scratch" {
            continue;
        }

        let Some(pkgs) = diff.get(name) else {
            continue;
        };

        if pkgs.is_empty() {
            continue;
        }

        let Some(resolved_provider) = providers::from_name(&name) else {
            return Err(anyhow!("Unkown provider `{}`", name));
        };

        let len = pkgs.len();
        info!(
            "[{}] installing {} packages: \n{}",
            name,
            len,
            pkgs.iter()
                .enumerate()
                .map(|(i, s)| format!("{:02}/{len:02}: {s}", i + 1))
                .collect::<Vec<_>>()
                .join("\n")
        );

        resolved_provider.install(args, &pkgs)?;
    }

    if let Some(provider) = conf.providers.iter().find(|p| p.name == "scratch") {
        if let config::Packages::ScratchPackages(scratch_pkgs) = &provider.packages {
            let pending: Vec<&config::ScratchPackage> = match diff.get("scratch") {
                Some(ids) => scratch_pkgs
                    .iter()
                    .filter(|p| ids.iter().any(|id| id == &p.identifier))
                    .collect(),
                None => vec![],
            };

            if !pending.is_empty() {
                let len = pending.len();
                info!(
                    "[scratch] installing {} packages: \n{}",
                    len,
                    pending
                        .iter()
                        .enumerate()
                        .map(|(i, p)| format!("{:02}/{len:02}: {}", i + 1, p.identifier))
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                let scratch = providers::Scratch;
                for pkg in pending {
                    scratch.install(args, pkg)?;
                }
            }
        }
    }

    if !args.dry {
        // all installs worked, so we are dumping config as lock to disk
        let lock: lock::Lock = conf.into();
        lock.dump(&args._lock_path)?;
    }

    Ok(())
}
