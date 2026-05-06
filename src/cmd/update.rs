use std::path::PathBuf;

use anyhow::anyhow;
use log::info;

use crate::{config, lock, providers, Args};

pub fn update(
    args: &Args,
    paths: &[PathBuf],
    conf: &config::Config,
    lock: &lock::Lock,
) -> anyhow::Result<()> {
    let mut updated = 0usize;

    for provider in &conf.providers {
        if provider.name == "scratch" {
            info!("[scratch] skipping update; scratch updates are not implemented");
            continue;
        }

        let Some(packages) = lock.packages.get(&provider.name) else {
            continue;
        };

        if packages.is_empty() {
            continue;
        }

        let Some(resolved_provider) = providers::from_name(&provider.name) else {
            return Err(anyhow!("Unknown provider `{}`", provider.name));
        };

        let len = packages.len();
        info!(
            "[{}] updating {} managed packages: \n{}",
            provider.name,
            len,
            packages
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{:02}/{len:02}: {s}", i + 1))
                .collect::<Vec<_>>()
                .join("\n")
        );

        updated += len;

        resolved_provider.update(args)?;
        resolved_provider.upgrade(args, packages)?;
    }

    if updated == 0 {
        info!("nothing to update, lock file does not contain managed packages");
    }

    let _ = paths;

    Ok(())
}
