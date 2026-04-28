use std::{env, process::exit};

use clap::Parser;

use config::Config;
use lock::Lock;
use log::{error, info, warn, LevelFilter};
use providers::process_packages;

mod cmd;
/// config contains the logic for deserializing mehr2.lua
mod config;
/// lock abstracts working with mehr2_lock.json files
mod lock;
/// path deals with looking up executables and file paths
mod path;
/// providers contains the PackageManager trait and its implementations
mod providers;

#[derive(Debug, PartialEq, Clone, clap::ValueEnum)]
enum Command {
    /// sync system to configuration file
    Sync,
    /// attempt to update all
    Update,
    /// show an overview for both installed and non installed packages
    Info,
}

/// Operating system-independent package managment abstraction
#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(value_enum)]
    cmd: Command,
}

fn main() {
    colog::basic_builder()
        .filter(None, LevelFilter::max())
        .init();

    let args = Args::parse();
    let config_dir_path = path::config()
        .map(|e| std::path::absolute(e).unwrap())
        .unwrap();

    let configuration_path = config_dir_path.join("mehr2.lua");
    info!("using configuration file: {:?}", configuration_path);
    let lock_path = config_dir_path.join("mehr2_lock.json");
    info!("using lock file: {:?}", lock_path);
    let lua_ctx = mlua::Lua::new();
    let config = match Config::from_path_buf(&lua_ctx, configuration_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!("{err}");
            exit(1);
        }
    };

    let lock: Lock = (&lock_path)
        .try_into()
        .inspect_err(|e| warn!("{e}"))
        .unwrap_or_default();
    match args.cmd {
        Command::Sync => match process_packages(config, lock) {
            Ok(lock) => {
                if let Err(err) = lock.dump(&lock_path) {
                    error!("{err}")
                }
            }
            Err(err) => error!("{err}"),
        },
        Command::Update => todo!("update"),
        Command::Info => {
            let paths = env::split_paths(&env::var_os("PATH").unwrap());
            cmd::info(path_env, config, lock);
        }
    }
}
