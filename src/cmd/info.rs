use std::path::PathBuf;

use crate::{config, fs, lock, providers, Args};
use log::trace;

pub fn info(
    _args: &Args,
    paths: &[PathBuf],
    conf: &config::Config,
    lock: &lock::Lock,
) -> anyhow::Result<()> {
    let diff = lock.diff_set(&conf);
    trace!(
        "providers: \n{}",
        conf.providers
            .iter()
            .filter(|(provider, _)| *provider != "scratch")
            .map(|(provider, _)| format!(
                "{provider}: {} at {}",
                if fs::binary_is_executable(provider) {
                    "OK"
                } else {
                    "MISSING"
                },
                fs::has_binary(paths, provider)
                    .unwrap_or_default()
                    .display()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    for (provider, packages) in &conf.providers {
        match packages {
            config::Packages::Packages(items) => {
                let provider_diff = diff.get(provider).unwrap();
                let resolved_provider = providers::from_name(&provider).unwrap();
                trace!(
                    "{provider}: \n{}",
                    items
                        .iter()
                        .map(|i| {
                            format!(
                                "{i}: {}",
                                if provider_diff.contains(i) {
                                    if resolved_provider.is_installed(paths, i).is_ok_and(|b| b) {
                                        "UNTRACKED"
                                    } else {
                                        "MISSING"
                                    }
                                } else {
                                    "OK"
                                }
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            config::Packages::ScratchPackages(scratch_packages) => {
                let provider_diff = diff.get(provider).unwrap();
                trace!(
                    "scratch: \n{}",
                    scratch_packages
                        .iter()
                        .map(|i| {
                            format!(
                                "{}: {}",
                                i.identifier,
                                if provider_diff.contains(&i.identifier) {
                                    "MISSING"
                                } else {
                                    "DONE"
                                }
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
    }

    Ok(())
}
