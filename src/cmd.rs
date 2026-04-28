use std::env::SplitPaths;

use crate::{config, lock, providers};
use log::{error, info, trace, warn};

pub fn info(paths: SplitPaths, config: config::Config, lock: lock::Lock) {
    let diff = lock.diff_set(&config);
    trace!(
        "providers: \n{}",
        config
            .packages
            .iter()
            .filter(|(provider, _)| *provider != "scratch")
            .map(|(provider, _)| format!(
                "{provider}: {} ({:#?})",
                if providers::binary_is_executable(provider) {
                    "OK"
                } else {
                    "MISSING"
                },
                providers::has_binary(&paths, provider).unwrap_or_default()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    for (provider, packages) in config.packages {
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
                                    if resolved_provider.is_installed(i).is_ok_and(|b| b) {
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
                trace!("scratch: \n")
            }
        }
    }
}
