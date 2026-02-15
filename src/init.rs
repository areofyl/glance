use anyhow::Result;
use std::fs;
use std::path::PathBuf;

const CONFIG_TOML: &str = r#"# file-preview daemon configuration

# directories to watch for new files
watch_dirs = ["~/Pictures/Screenshots", "~/Downloads"]

# RTMIN+N signal to poke waybar on new file
signal_number = 8

# auto-dismiss the widget after N seconds
dismiss_seconds = 10

# ignore files with these suffixes (partial downloads, etc.)
ignore_suffixes = [".part", ".crdownload", ".tmp"]

# pixels from top of screen to below waybar (popup appears here)
bar_height = 57
"#;

const WAYBAR_MODULE: &str = r#"
"custom/file-preview": {
    "exec": "file-preview status",
    "return-type": "json",
    "interval": 1,
    "signal": 8,
    "on-click": "file-preview drag",
    "on-click-right": "file-preview copy"
}
"#;

const WAYBAR_CSS: &str = r#"
/* file-preview widget */
#custom-file-preview {
    padding: 0 8px;
    color: #cdd6f4;
}

#custom-file-preview.active {
    color: #a6e3a1;
}

#custom-file-preview.empty {
    padding: 0;
}
"#;

fn config_base() -> PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()))
                .join(".config")
        })
}

fn ok(msg: &str) {
    eprintln!("\x1b[32m  ✓\x1b[0m {msg}");
}

fn skip(msg: &str) {
    eprintln!("\x1b[90m  ·\x1b[0m {msg}");
}

fn contains(path: &PathBuf, needle: &str) -> bool {
    fs::read_to_string(path)
        .map(|s| s.contains(needle))
        .unwrap_or(false)
}

fn setup_config() -> Result<()> {
    let path = config_base().join("file-preview/config.toml");
    if path.exists() {
        skip(&format!("config already exists: {}", path.display()));
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, CONFIG_TOML)?;
    ok(&format!("created {}", path.display()));
    Ok(())
}

fn setup_waybar_module() -> Result<()> {
    let base = config_base().join("waybar");
    if !base.exists() {
        skip("waybar config dir not found, skipping module setup");
        return Ok(());
    }

    // find the main config file
    let config_file = ["config.jsonc", "config"]
        .iter()
        .map(|f| base.join(f))
        .find(|p| p.exists());

    let Some(config_file) = config_file else {
        skip("no waybar config file found, skipping module setup");
        return Ok(());
    };

    if contains(&config_file, "file-preview") {
        skip("waybar module already configured");
        return Ok(());
    }

    // try to find a modules include file to append to, otherwise use the main config
    let modules_file = ["UserModules", "ModulesCustom"]
        .iter()
        .map(|f| base.join(f))
        .find(|p| p.exists());

    if let Some(mf) = modules_file {
        if contains(&mf, "file-preview") {
            skip("waybar module already in modules file");
            return Ok(());
        }
        // insert before the last closing brace
        let content = fs::read_to_string(&mf)?;
        if let Some(pos) = content.rfind('}') {
            let mut new = String::with_capacity(content.len() + WAYBAR_MODULE.len() + 10);
            let before = content[..pos].trim_end();
            new.push_str(before);
            // add comma if the last non-whitespace char before } isn't { or ,
            let last_char = before.chars().rev().find(|c| !c.is_whitespace());
            if last_char != Some('{') && last_char != Some(',') {
                new.push(',');
            }
            new.push_str(WAYBAR_MODULE);
            new.push_str("}\n");
            fs::write(&mf, new)?;
            ok(&format!("added waybar module to {}", mf.display()));
        }
    } else {
        // append as comment with instructions
        let mut content = fs::read_to_string(&config_file)?;
        content.push_str(&format!(
            "\n// Add this to your modules config:\n// {}\n",
            WAYBAR_MODULE.trim().replace('\n', "\n// ")
        ));
        fs::write(&config_file, content)?;
        ok(&format!(
            "added waybar module snippet to {}",
            config_file.display()
        ));
    }

    // also add to modules-right if not already there
    if let Ok(content) = fs::read_to_string(&config_file) {
        if content.contains("modules-right")
            && !content.contains("custom/file-preview")
        {
            let new = content.replacen(
                "\"modules-right\": [",
                "\"modules-right\": [\n\t\"custom/file-preview\",",
                1,
            );
            if new != content {
                fs::write(&config_file, new)?;
                ok("added custom/file-preview to modules-right");
            }
        }
    }

    Ok(())
}

fn setup_waybar_css() -> Result<()> {
    let path = config_base().join("waybar/style.css");
    if !path.exists() {
        skip("waybar style.css not found, skipping CSS setup");
        return Ok(());
    }
    if contains(&path, "custom-file-preview") {
        skip("waybar CSS already has file-preview styles");
        return Ok(());
    }
    let mut content = fs::read_to_string(&path)?;
    content.push_str(WAYBAR_CSS);
    fs::write(&path, content)?;
    ok(&format!("appended styles to {}", path.display()));
    Ok(())
}

fn setup_hyprland() -> Result<()> {
    let path = config_base().join("hypr/hyprland.conf");
    if !path.exists() {
        skip("hyprland.conf not found, skipping autostart setup");
        return Ok(());
    }
    if contains(&path, "file-preview") {
        skip("hyprland autostart already configured");
        return Ok(());
    }
    let mut content = fs::read_to_string(&path)?;
    content.push_str("\nexec-once = file-preview watch\n");
    fs::write(&path, content)?;
    ok(&format!("added exec-once to {}", path.display()));
    Ok(())
}

pub fn run() -> Result<()> {
    eprintln!("\n  file-preview init\n");
    setup_config()?;
    setup_waybar_module()?;
    setup_waybar_css()?;
    setup_hyprland()?;
    eprintln!("\n  Done! Restart Waybar to activate: pkill waybar && waybar &\n");
    Ok(())
}
