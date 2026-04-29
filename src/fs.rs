use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn binary_is_executable(name: &str) -> bool {
    Command::new(name).output().is_ok()
}

pub fn has_binary(paths: &[PathBuf], name: &str) -> Option<PathBuf> {
    for path in paths {
        let full = path.join(name);
        if full.exists() && fs::metadata(&full).is_ok() {
            return Some(full);
        }
    }
    None
}

pub fn file_ok(path: &Path) -> bool {
    path.exists() && path.is_file()
}
