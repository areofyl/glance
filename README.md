# glance

A file clipboard for Wayland — watches directories for new files and shows a transient widget in [Waybar](https://github.com/Alexays/Waybar).
Click to open a dropdown menu with actions: drag-and-drop, open, edit, or copy the path.

## What's new in 0.3.5

- **Configurable drag command** — set `drag_command` in your config to use [ripdrag](https://github.com/nik012003/ripdrag), [dragon](https://github.com/mwh/dragon), or any drag tool of your choice. Useful for dragging files into XWayland browsers where native Wayland DnD doesn't work. Defaults to the builtin GTK4 drag overlay.
- **Fix: waybar freeze** — `pkill -RTMIN+8 waybar` was accidentally killing waybar's child processes (like a running `glance status`), leaving waybar with an orphaned pipe that it busy-loops on. Now targets only the main waybar process with `-x -o` flags.
- **Fix: waybar polling interval** — changed from `"interval": 1` (spawned 3600 processes/hr) to `"interval": 5`, massively reducing process spawning overhead.
- **Fix: full binary paths** — `glance init` now writes absolute paths in Waybar and Hyprland configs, so glance works even when `~/.local/bin` or `~/.cargo/bin` aren't in waybar's PATH.
- **Fix: menu scroll** — scroll inside the dropdown menu now uses the full binary path, fixing scroll not working when launched from waybar.

![demo](demo.gif)

## Dependencies

**Arch Linux:**

```sh
sudo pacman -S gtk4 gtk4-layer-shell wl-clipboard swappy
```

**Fedora:**

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel wl-clipboard swappy
```

You also need a [Rust toolchain](https://rustup.rs/) and **Hyprland** (uses `hyprctl` for overlay placement).

[swappy](https://github.com/jtheoof/swappy) is a lightweight Wayland screenshot annotation tool used by the Edit button. If not installed, glance will fall back to opening files with your default app. You can also set a different editor in the config.

## Install

**From crates.io:**

```sh
cargo install wayglance
```

> If `cargo install` fails with "gtk4.pc not found", try: `PKG_CONFIG_PATH=/usr/lib64/pkgconfig cargo install wayglance`

**From source:**

```sh
git clone https://github.com/areofyl/glance
cd glance
cargo build --release
cp target/release/glance ~/.local/bin/
```

Then run the setup wizard:

```sh
glance init
```

This automatically:
- Creates the default config at `~/.config/glance/config.toml`
- Adds the Waybar module to your Waybar config (with full binary paths)
- Appends CSS styles to your Waybar `style.css`
- Adds `exec-once` and `SUPER+V` keybind to your Hyprland config

Restart Waybar and you're done.

<details>
<summary>Manual setup</summary>

### Autostart

Add to your Hyprland config (`~/.config/hypr/hyprland.conf`):

```
exec-once = /path/to/glance watch
bind = SUPER, V, exec, /path/to/glance drag
```

### Waybar module

Add to your Waybar config (`~/.config/waybar/config.jsonc`):

```jsonc
"custom/glance": {
    "exec": "/path/to/glance status",
    "return-type": "json",
    "interval": 5,
    "signal": 8,
    "on-click": "/path/to/glance menu",
    "on-click-right": "/path/to/glance copy",
    "on-scroll-up": "/path/to/glance scroll up",
    "on-scroll-down": "/path/to/glance scroll down"
}
```

> Use the full path to the glance binary (e.g. `/home/you/.local/bin/glance`) since waybar may not have `~/.local/bin` in its PATH.

Then add `"custom/glance"` to your bar layout (e.g. `modules-right`).
A complete snippet is in [`waybar-module.jsonc`](waybar-module.jsonc).

### Waybar styling

Add to your Waybar CSS (`~/.config/waybar/style.css`):

```css
#custom-glance {
    padding: 0 8px;
    color: #cdd6f4;
}

#custom-glance.active {
    color: #a6e3a1;
}

#custom-glance.empty {
    padding: 0;
}
```

</details>

## Config

Optional. Copy `config.example.toml` to `~/.config/glance/config.toml`
and edit to taste. Everything has sane defaults.

```toml
# directories to watch for new files
watch_dirs = ["~/Pictures/Screenshots", "~/Downloads"]

# RTMIN+N signal to poke waybar
signal_number = 8

# auto-dismiss the waybar widget after N seconds
dismiss_seconds = 10

# skip partial downloads etc.
ignore_suffixes = [".part", ".crdownload", ".tmp"]

# waybar bar height in px (for menu placement)
bar_height = 57

# number of files to remember in history
history_size = 5

# editor command for the Edit button (default: "swappy -f")
# supports full commands with arguments, e.g. "gimp -n" or "swappy -f"
editor = "swappy -f"

# which buttons to show in the dropdown
actions = ["drag", "open", "edit", "copy"]

# auto-dismiss the dropdown after N seconds (0 = never)
menu_dismiss_seconds = 8

# drag command: "builtin" for GTK4 native drag, or an external tool
# use "ripdrag --and-exit" for better browser compatibility (XWayland)
drag_command = "builtin"

# customize menu appearance
[menu_style]
background = "rgba(30,30,46,0.95)"
text_color = "#cdd6f4"
secondary_color = "#a6adc8"
button_background = "rgba(255,255,255,0.08)"
button_hover = "rgba(255,255,255,0.15)"
border_radius = 12
```

## Commands

```
glance init            # set up config, waybar module, CSS, and autostart
glance watch           # run the inotify watcher (long-running)
glance status          # JSON for waybar (called by exec)
glance menu            # dropdown menu below waybar with actions
glance copy            # wl-copy the selected file path
glance drag            # drag-and-drop overlay at cursor
glance scroll up|down  # navigate through file history
```

## License

[MIT](LICENSE)
