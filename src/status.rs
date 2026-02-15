use crate::config::Config;
use crate::state::read_history;
use anyhow::Result;
use serde_json::json;

pub fn run(cfg: &Config, index_override: Option<usize>) -> Result<()> {
    let state_file = Config::state_file();
    let history = read_history(&state_file);

    // use override if provided, otherwise use persisted selection
    let selected = index_override.unwrap_or(history.selected);

    // get the entry at the selected position (if not expired)
    let current = history
        .entries
        .get(selected)
        .filter(|e| !e.is_expired(cfg.dismiss_seconds));

    // count all non-expired entries
    let active_count = history
        .entries
        .iter()
        .filter(|e| !e.is_expired(cfg.dismiss_seconds))
        .count();

    let output = match current {
        Some(st) => {
            let name = if st.name.len() > 18 {
                format!("{}\u{2026}", &st.name[..15])
            } else {
                st.name.clone()
            };
            let count_suffix = if active_count > 1 {
                format!(" ({}/{})", selected + 1, active_count)
            } else {
                String::new()
            };
            let tooltip_lines: Vec<String> = history
                .entries
                .iter()
                .enumerate()
                .filter(|(_, e)| !e.is_expired(cfg.dismiss_seconds))
                .map(|(i, e)| {
                    let marker = if i == selected { "▸" } else { " " };
                    format!("{marker} {} ({})", e.name, human_size(e.size))
                })
                .collect();
            json!({
                "text": format!(" {name}{count_suffix}"),
                "tooltip": tooltip_lines.join("\n"),
                "class": "active",
                "alt": "active",
            })
        }
        None => json!({
            "text": "",
            "tooltip": "",
            "class": "empty",
            "alt": "empty",
        }),
    };
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

fn human_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    for unit in &["B", "KB", "MB", "GB"] {
        if size < 1024.0 {
            return if *unit == "B" {
                format!("{size} B")
            } else {
                format!("{size:.1} {unit}")
            };
        }
        size /= 1024.0;
    }
    format!("{size:.1} TB")
}
