#![allow(unused)]
use std::{env, path::PathBuf, process::exit};

use clap::Parser;

use config::Config;
use lock::Lock;
use log::{error, info, trace, LevelFilter};

mod cmd;
/// config contains the logic for deserializing mehr2.lua
mod config;
/// small file system helpers
mod fs;
/// lock abstracts working with mehr2_lock.json files
mod lock;
/// path deals with looking up executables and file paths
mod path;
/// providers contains the PackageManager trait and its implementations
mod providers;

#[derive(Debug, PartialEq, Clone, clap::ValueEnum, Copy)]
#[repr(usize)]
pub enum Action {
    /// Sync system to configuration file
    Sync,
    /// Attempt to update all mehr managed packages
    Update,
    /// Overview over packages managed by mehr
    Info,
    Version,
}

/// Declarative package provisioning across Linux distributions
#[derive(clap::Parser, Debug)]
pub struct Args {
    #[clap(value_enum)]
    cmd: Action,
    #[clap(short, long)]
    /// disable safe guards for a specific action
    force: bool,
    #[clap(long)]
    /// give the actions output in json
    json: bool,
    #[clap(short, long)]
    /// remove all info, error, warn, trace, debug logs
    silent: bool,
    #[clap(short, long)]
    /// stop right before invoking providers
    dry: bool,

    /// injected to propagate file paths
    #[clap(skip)]
    _configuration_path: PathBuf,
    #[clap(skip)]
    _lock_path: PathBuf,
}

fn main() {
    let mut args = Args::parse();

    if !args.silent {
        colog::basic_builder()
            .filter(None, LevelFilter::max())
            .init();
    }

    let config_dir_path = path::config()
        .map(|e| {
            std::path::absolute(e).unwrap_or_else(|e| {
                error!("{e}");
                exit(1);
            })
        })
        .unwrap_or_else(|e| {
            error!("{e}");
            exit(1);
        });

    args._configuration_path = config_dir_path.join("mehr2.lua");
    trace!("using configuration file: {:?}", args._configuration_path);
    args._lock_path = config_dir_path.join("mehr2_lock.json");
    trace!("using lock file: {:?}", args._lock_path);

    let lua_ctx = mlua::Lua::new();
    let config = match Config::from_path_buf(&lua_ctx, &args._configuration_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!("{err}");
            exit(1);
        }
    };

    let lock: Lock = (&args._lock_path)
        .try_into()
        .inspect_err(|e| {dbg!(e);info!("lock file not found, this seems to be the first run\nthere will be something in the lock file once we install something")})
        .unwrap_or_default();

    if let Action::Version = &args.cmd {
        const BUILD_INFO: &str = concat!(
            "version=",
            env!("CARGO_PKG_VERSION"),
            ";commit=",
            env!("GIT_HASH"),
            ";built=",
            env!("BUILD_TIMESTAMP"),
            ";features=",
            env!("BUILD_FEATURES"),
            ";profile=",
            env!("BUILD_PROFILE"),
        );

        println!(
            "mehr2 version {} by xnacly and contributors",
            env!("CARGO_PKG_VERSION")
        );
        println!("{}", BUILD_INFO.replace(";", "\n"));
        let exe = std::env::current_exe().unwrap();
        println!("from={}", exe.display());
        println!(
            "config={} ({})",
            args._configuration_path.display(),
            if fs::file_ok(&args._configuration_path) {
                "OK"
            } else {
                "MISSING"
            }
        );
        println!(
            "lock={} ({})",
            args._lock_path.display(),
            if fs::file_ok(&args._lock_path) {
                "OK"
            } else {
                "MISSING"
            }
        );

        return;
    }

    let path_env = env::var_os("PATH").unwrap();
    let paths = env::split_paths(&path_env).collect::<Vec<_>>();
    if let Err(e) = cmd::for_action(args.cmd)(&args, &paths, &config, &lock) {
        error!("failed to execute action: {e}");
        exit(1);
    }
}
