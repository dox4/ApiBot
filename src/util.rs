use std::path::PathBuf;

use dirs;

pub(crate) fn app_home_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(format!(".{}", env!("CARGO_PKG_NAME")))
}
