use std::process::Command;

pub fn cursor_pos() -> Option<(i32, i32)> {
    let out = Command::new("hyprctl").arg("cursorpos").output().ok()?;
    let text = String::from_utf8(out.stdout).ok()?;
    let parts: Vec<&str> = text.trim().split(',').collect();
    if parts.len() >= 2 {
        Some((parts[0].trim().parse().ok()?, parts[1].trim().parse().ok()?))
    } else {
        None
    }
}

pub fn find_monitor_at(gx: i32, gy: i32) -> Option<(String, i32, i32)> {
    let out = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .ok()?;
    let text = String::from_utf8(out.stdout).ok()?;
    let monitors: Vec<serde_json::Value> = serde_json::from_str(&text).ok()?;
    for m in &monitors {
        let name = m["name"].as_str()?;
        let x = m["x"].as_i64()? as i32;
        let y = m["y"].as_i64()? as i32;
        let w = m["width"].as_i64()? as i32;
        let h = m["height"].as_i64()? as i32;
        if gx >= x && gx < x + w && gy >= y && gy < y + h {
            return Some((name.to_string(), x, y));
        }
    }
    None
}

pub fn human_size(bytes: u64) -> String {
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
