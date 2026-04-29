use std::path::PathBuf;

use crate::{config, lock, providers};
use log::trace;

pub fn info(paths: &[PathBuf], config: config::Config, lock: lock::Lock) {
    let diff = lock.diff_set(&config);
    trace!(
        "providers: \n{}",
        config
            .providers
            .iter()
            .filter(|(provider, _)| *provider != "scratch")
            .map(|(provider, _)| format!(
                "{provider}: {} at {}",
                if providers::binary_is_executable(provider) {
                    "OK"
                } else {
                    "MISSING"
                },
                providers::has_binary(paths, provider)
                    .unwrap_or_default()
                    .display()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    for (provider, packages) in config.providers {
        match packages {
            config::Packages::Packages(items) => {
                let provider_diff = diff.get(&provider).unwrap();
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
                let provider_diff = diff.get(&provider).unwrap();
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
}
