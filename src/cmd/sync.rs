use std::path::PathBuf;

use crate::{config, lock, Args};

pub fn sync(
    args: &Args,
    paths: &[PathBuf],
    conf: &config::Config,
    lock: &lock::Lock,
) -> anyhow::Result<()> {
    todo!()
}
