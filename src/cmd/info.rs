use std::path::PathBuf;

use crate::{
    config::{self, Provider},
    fs, lock, providers, Args,
};
use log::trace;
use serde::Serialize;

#[derive(Serialize)]
struct ProviderReport {
    pub name: String,
    pub status: ProviderStatus,
    pub path: Option<String>,
    pub packages: Vec<PackageReport>,
}

#[derive(Debug, Serialize)]
enum ProviderStatus {
    Ok,
    Missing,
}

#[derive(Debug, Serialize)]
enum PackageStatus {
    Ok,
    Missing,
    Untracked,
    Done,
}

#[derive(Debug, Serialize)]
struct PackageReport {
    pub name: String,
    pub status: PackageStatus,
}

fn build_report(
    paths: &[PathBuf],
    conf: &config::Config,
    lock: &lock::Lock,
) -> Vec<ProviderReport> {
    let diff = lock.diff_set(&conf);
    let mut report = vec![];

    for Provider { name, packages } in &conf.providers {
        let provider_ok = fs::binary_is_executable(name);
        let provider_path = fs::has_binary(paths, name).map(|p| p.display().to_string());

        let mut provider_report = ProviderReport {
            name: name.clone(),
            status: if provider_ok {
                ProviderStatus::Ok
            } else {
                ProviderStatus::Missing
            },
            path: provider_path,
            packages: vec![],
        };

        match packages {
            config::Packages::Packages(items) => {
                let provider_diff = diff.get(name).unwrap();
                let resolved_provider = providers::from_name(name).unwrap();

                for i in items {
                    let status = if provider_diff.contains(i) {
                        if resolved_provider.is_installed(paths, i).is_ok_and(|b| b) {
                            PackageStatus::Untracked
                        } else {
                            PackageStatus::Missing
                        }
                    } else {
                        PackageStatus::Ok
                    };

                    provider_report.packages.push(PackageReport {
                        name: i.clone(),
                        status,
                    });
                }
            }
            config::Packages::ScratchPackages(scratch_packages) => {
                let provider_diff = diff.get(name).unwrap();

                for i in scratch_packages {
                    let status = if provider_diff.contains(&i.identifier) {
                        PackageStatus::Missing
                    } else {
                        PackageStatus::Done
                    };

                    provider_report.packages.push(PackageReport {
                        name: i.identifier.clone(),
                        status,
                    });
                }
            }
        }

        report.push(provider_report);
    }

    report
}

pub fn info(args: &Args, conf: &config::Config, lock: &lock::Lock) -> anyhow::Result<()> {
    let report = build_report(&args._system_path, conf, lock);
    if args.json {
        println!("{}", serde_json::to_string(&report).unwrap());
    } else {
        trace!(
            "providers: \n{}",
            report
                .iter()
                .filter(|p| p.name != "scratch")
                .map(|p| format!(
                    "{}: {:?} at {}",
                    p.name,
                    p.status,
                    p.path
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| "None")
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );

        for p in &report {
            trace!(
                "{}: \n{}",
                p.name,
                p.packages
                    .iter()
                    .map(|p| format!("{}: {:?}", p.name, p.status))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    Ok(())
}
