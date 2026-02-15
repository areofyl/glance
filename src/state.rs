use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileState {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub time: f64,
}

impl FileState {
    pub fn new(path: PathBuf) -> Result<Self> {
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs_f64();
        Ok(Self { path, name, size, time })
    }

    pub fn is_expired(&self, dismiss_secs: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        now - self.time > (dismiss_secs + 2) as f64
    }
}

pub fn read_state(state_file: &Path, dismiss_secs: u64) -> Option<FileState> {
    let content = std::fs::read_to_string(state_file).ok()?;
    let state: FileState = serde_json::from_str(&content).ok()?;
    if state.is_expired(dismiss_secs) {
        None
    } else {
        Some(state)
    }
}

pub fn write_state(state_file: &Path, state: &FileState) -> Result<()> {
    let json = serde_json::to_string(state)?;
    std::fs::write(state_file, json)?;
    Ok(())
}
