use std::path::PathBuf;

use crate::{config, lock, Args};

pub fn update(
    args: &Args,
    paths: &[PathBuf],
    conf: &config::Config,
    lock: &lock::Lock,
) -> anyhow::Result<()> {
    todo!()
}
