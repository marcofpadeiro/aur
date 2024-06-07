use std::{path::Path, process::Stdio};

use serde::{Deserialize, Serialize};

use crate::theme;

const DEFAULT_PATH: &str = ".config/aurme";
const SETTINGS_FILE: &str = "config.json";
const CACHE_PATH: &str = ".cache/aurme";
const PACKAGES_CACHE_PATH: &str = ".cache/aurme/packages";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    cache_path: String,
    db_path: String,
    keep_cache: bool,
    no_confirm: bool,
    verbose: String,
}

impl Config {
    pub fn get_cache_path(&self) -> &str {
        &self.cache_path
    }

    pub fn get_db_path(&self) -> &str {
        &self.db_path
    }

    pub fn get_keep_cache(&self) -> bool {
        self.keep_cache
    }

    pub fn get_no_confirm(&self) -> bool {
        self.no_confirm
    }

    pub fn get_verbose(&self) -> &str {
        &self.verbose
    }

    pub fn get_verbose_config(&self) -> (Stdio, Stdio) {
        match self.verbose.as_str() {
            "verbose" => (
                std::process::Stdio::inherit(),
                std::process::Stdio::inherit(),
            ),
            "quiet" => (std::process::Stdio::piped(), std::process::Stdio::piped()),
            _ => (std::process::Stdio::piped(), std::process::Stdio::inherit()),
        }
    }
}

pub fn read() -> Config {
    let path = format!(
        "{}/{}/{}",
        home::home_dir().unwrap().display(),
        DEFAULT_PATH,
        SETTINGS_FILE
    );
    let config_path = std::path::Path::new(&path);

    if let Err(_) = std::fs::metadata(config_path) {
        return create_default(config_path);
    }

    let json = std::fs::read_to_string(config_path).unwrap();
    if let Ok(config) = serde_json::from_str::<Config>(&json) {
        return config;
    }

    println!(
        "{}",
        theme::colorize(
            theme::Type::Warning,
            "Your config file has been updated to the latest version."
        )
    );
    println!(
        "{}",
        theme::colorize(
            theme::Type::Warning,
            "Old config file has been renamed to config.json.old."
        )
    );
    std::fs::rename(config_path, format!("{}.old", path)).unwrap();

    create_default(config_path)
}

fn create_default(path: &Path) -> Config {
    let config_folder = format!("{}/{}", home::home_dir().unwrap().display(), DEFAULT_PATH);
    let config_folder_path = std::path::Path::new(&config_folder);

    let config = Config {
        cache_path: String::from(PACKAGES_CACHE_PATH),
        db_path: String::from(CACHE_PATH),
        keep_cache: true,
        no_confirm: false,
        verbose: String::from("default"),
    };
    let json = serde_json::to_string_pretty(&config).unwrap();
    if let Err(_) = std::fs::metadata(config_folder_path) {
        std::fs::create_dir_all(config_folder_path).unwrap();
    }
    std::fs::write(path, json).unwrap();
    config
}
