use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    path::PathBuf,
};

use crate::config::{
    Config,
    Packages::{Packages, ScratchPackages},
    Provider,
};

/// Lock holds a map of installed packages, by their package manager, stored in lock.mehr2
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Lock {
    pub packages: HashMap<String, Vec<String>>,
}

impl Lock {
    /// dumps self to path
    pub fn dump(&self, path: &PathBuf) -> Result<()> {
        let file = File::options().write(true).create(true).open(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn diff(&self, config: &Config) -> HashMap<String, Vec<String>> {
        let other: Lock = config.into();
        let mut store = HashMap::new();
        for (key, other_packages) in other.packages {
            let other: HashSet<_> = HashSet::from_iter(other_packages.iter());
            if let Some(packages) = self.packages.get(&key) {
                let diff = other
                    .difference(&HashSet::from_iter(packages.iter()))
                    .map(|p| p.to_string())
                    .collect();
                store.insert(key, diff);
            } else {
                store.insert(key, other_packages);
            }
        }
        store
    }

    pub fn diff_set(&self, config: &Config) -> HashMap<String, HashSet<String>> {
        let desired: Lock = config.into();
        let mut result: HashMap<String, HashSet<String>> = HashMap::new();

        for (provider, desired_packages) in desired.packages {
            let installed_packages = self.packages.get(&provider);

            let installed_set: HashSet<&String> = installed_packages
                .map(|v| v.iter().collect())
                .unwrap_or_default();

            let missing: HashSet<String> = desired_packages
                .into_iter()
                .filter(|pkg| !installed_set.contains(pkg))
                .collect();

            result.insert(provider, missing);
        }

        result
    }
}

impl From<&Config> for Lock {
    fn from(value: &Config) -> Self {
        let mut lock = Self::default();
        value
            .providers
            .iter()
            .for_each(|Provider { name, packages }| {
                let package_names: Vec<String> = match packages {
                    Packages(packages) => packages.clone(),
                    ScratchPackages(packages) => {
                        packages.into_iter().map(|p| p.identifier.clone()).collect()
                    }
                };
                lock.packages.insert(name.clone(), package_names);
            });
        lock
    }
}

impl TryFrom<&PathBuf> for Lock {
    type Error = String;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let lock_content =
            fs::read_to_string(value).map_err(|err| format!("failed to open lock file: {err}"))?;
        Ok(serde_json::from_str(&lock_content)
            .map_err(|err| format!("failed to deserialize lock file: {err}"))?)
    }
}
