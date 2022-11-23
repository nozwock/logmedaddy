use std::path::PathBuf;

pub const APP_DIR: &str = "com.github.nozwock.logmedaddy";

pub fn app_config_dir() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join(APP_DIR))
}

pub fn app_config_path() -> PathBuf {
    app_config_dir().unwrap_or_default().join("config.ron")
}
