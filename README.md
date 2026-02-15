# glance

A file clipboard for Wayland — watches directories for new files and shows a transient widget in [Waybar](https://github.com/Alexays/Waybar).
Click to copy the path, drag-and-drop into another app, or scroll through recent files.

![demo](demo.gif)

## Dependencies

**Arch Linux:**

```sh
sudo pacman -S gtk4 gtk4-layer-shell wl-clipboard
```

**Fedora:**

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel wl-clipboard
```

You also need a [Rust toolchain](https://rustup.rs/) and **Hyprland** (uses `hyprctl` for overlay placement).

Optional: install [ripdrag](https://github.com/nik012003/ripdrag) for reliable Wayland drag-and-drop.

## Install

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
- Adds the Waybar module to your Waybar config
- Appends CSS styles to your Waybar `style.css`
- Adds `exec-once = glance watch` and `SUPER+V` keybind to your Hyprland config

Restart Waybar and you're done.

<details>
<summary>Manual setup</summary>

### Autostart

Add to your Hyprland config (`~/.config/hypr/hyprland.conf`):

```
exec-once = glance watch
bind = SUPER, V, exec, glance drag
```

### Waybar module

Add to your Waybar config (`~/.config/waybar/config.jsonc`):

```jsonc
"custom/glance": {
    "exec": "glance status",
    "return-type": "json",
    "interval": 1,
    "signal": 8,
    "on-click": "glance drag",
    "on-click-right": "glance copy",
    "on-scroll-up": "glance scroll up",
    "on-scroll-down": "glance scroll down"
}
```

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

# auto-dismiss after N seconds
dismiss_seconds = 10

# skip partial downloads etc.
ignore_suffixes = [".part", ".crdownload", ".tmp"]

# waybar bar height in px (for popup placement)
bar_height = 57

# number of files to remember in history
history_size = 5
```

## Commands

```
glance init            # set up config, waybar module, CSS, and autostart
glance watch           # run the inotify watcher (long-running)
glance status          # JSON for waybar (called by exec)
glance copy            # wl-copy the latest file path
glance drag            # drag-and-drop overlay at cursor
glance bubble <path>   # floating thumbnail notification for a file
glance scroll up|down  # navigate through file history
```

## License

[MIT](LICENSE)
