use anyhow::Result;
use log::info;

use crate::{config, lock, providers, Action, Args};

pub mod info;
pub mod sync;
pub mod update;

type Cmd = fn(&Args, &config::Config, &lock::Lock) -> Result<()>;

pub fn for_action(action: Action) -> Cmd {
    match action {
        Action::Sync => sync::sync,
        Action::Update => update::update,
        Action::Info => info::info,
        Action::Providers => |_, _, _| -> Result<()> {
            info!(
                "Available providers: \n- {}",
                providers::names().collect::<Vec<_>>().join("\n- ")
            );
            Ok(())
        },
        _ => unreachable!(),
    }
}
