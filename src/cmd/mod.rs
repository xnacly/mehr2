use std::path::PathBuf;

use anyhow::Result;

use crate::{config, lock, Action, Args};

pub mod info;
pub mod sync;
pub mod update;

type Cmd = fn(&Args, &[PathBuf], &config::Config, &lock::Lock) -> Result<()>;

pub fn for_action(action: Action) -> Cmd {
    match action {
        Action::Sync => sync::sync,
        Action::Update => update::update,
        Action::Info => info::info,
        _ => unreachable!(),
    }
}
