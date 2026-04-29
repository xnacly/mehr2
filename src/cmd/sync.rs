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

        if !args.dry {
            resolved_provider.install(&pkgs)?;
        }
    }

    if !args.dry {
        // all installs worked, so we are dumping config as lock to disk
        let lock: lock::Lock = conf.into();
        lock.dump(&args._lock_path)?;
    }

    Ok(())
}
