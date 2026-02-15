use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

fn default_watch_dirs() -> Vec<String> {
    vec![
        "~/Pictures/Screenshots".into(),
        "~/Downloads".into(),
    ]
}
fn default_signal_number() -> u8 {
    8
}
fn default_dismiss_seconds() -> u64 {
    10
}
fn default_ignore_suffixes() -> Vec<String> {
    vec![".part".into(), ".crdownload".into(), ".tmp".into()]
}
fn default_bar_height() -> i32 {
    57
}
fn default_history_size() -> usize {
    5
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_watch_dirs")]
    pub watch_dirs: Vec<String>,
    #[serde(default = "default_signal_number")]
    pub signal_number: u8,
    #[serde(default = "default_dismiss_seconds")]
    pub dismiss_seconds: u64,
    #[serde(default = "default_ignore_suffixes")]
    pub ignore_suffixes: Vec<String>,
    #[serde(default = "default_bar_height")]
    pub bar_height: i32,
    #[serde(default = "default_history_size")]
    pub history_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch_dirs: default_watch_dirs(),
            signal_number: default_signal_number(),
            dismiss_seconds: default_dismiss_seconds(),
            ignore_suffixes: default_ignore_suffixes(),
            bar_height: default_bar_height(),
            history_size: default_history_size(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = config_path();
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&config_path)?;
        let mut cfg: Config = toml::from_str(&content)?;
        cfg.watch_dirs = cfg
            .watch_dirs
            .into_iter()
            .map(|d| shellexpand::tilde(&d).into_owned())
            .collect();
        Ok(cfg)
    }

    pub fn state_file() -> PathBuf {
        runtime_dir().join("glance-latest.json")
    }

    pub fn pid_file() -> PathBuf {
        runtime_dir().join("glance.pid")
    }
}

fn runtime_dir() -> PathBuf {
    PathBuf::from(
        std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into()),
    )
}

fn config_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()))
                .join(".config")
        });
    base.join("glance/config.toml")
}
