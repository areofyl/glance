use crate::config::Config;
use crate::state::read_state;
use anyhow::Result;
use gtk4::gdk;
use gtk4::gio;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::process::Command;
use std::time::Duration;

pub fn run(cfg: &Config) -> Result<()> {
    let state_file = Config::state_file();
    let Some(st) = read_state(&state_file, cfg.dismiss_seconds) else {
        return Ok(());
    };
    if !st.path.exists() {
        return Ok(());
    }

    let (cursor_x, _cursor_y) = cursor_pos().unwrap_or((800, 0));
    let bar_height = cfg.bar_height;
    let filepath = st.path.clone();

    let app = gtk4::Application::builder()
        .application_id("dev.file-preview.drag")
        .build();

    app.connect_activate(move |app| {
        let win = gtk4::ApplicationWindow::new(app);

        win.init_layer_shell();
        win.set_layer(Layer::Overlay);
        win.set_anchor(Edge::Top, true);
        win.set_anchor(Edge::Left, true);

        let overlay_w = 200;
        let margin_left = (cursor_x - overlay_w / 2).max(0);
        win.set_margin(Edge::Top, 0);
        win.set_margin(Edge::Left, margin_left);
        win.set_exclusive_zone(-1);
        win.set_namespace(Some("file-preview-drag"));

        // near-invisible surface so Wayland routes pointer events
        let css = gtk4::CssProvider::new();
        #[allow(deprecated)]
        css.load_from_data(
            "window { background: rgba(0,0,0,0.01); } \
             label  { color: rgba(0,0,0,0.01); }",
        );
        gtk4::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &css,
            gtk4::STYLE_PROVIDER_PRIORITY_USER,
        );

        let label = gtk4::Label::new(Some(" "));
        label.set_size_request(overlay_w, bar_height);

        // drag source
        let ds = gtk4::DragSource::new();
        ds.set_actions(gdk::DragAction::COPY);

        let file = gio::File::for_path(&filepath);
        let uri = format!("{}\r\n", file.uri());

        ds.connect_prepare(move |_src, _x, _y| {
            Some(gdk::ContentProvider::for_bytes(
                "text/uri-list",
                &glib::Bytes::from(uri.as_bytes()),
            ))
        });

        let app_ref = app.clone();
        ds.connect_drag_end(move |_src, _drag, _delete| {
            let a = app_ref.clone();
            glib::timeout_add_local_once(Duration::from_millis(500), move || {
                a.quit();
            });
        });

        label.add_controller(ds);
        win.set_child(Some(&label));
        win.present();

        // escape to dismiss
        let key_ctl = gtk4::EventControllerKey::new();
        let app_ref = app.clone();
        key_ctl.connect_key_pressed(move |_ctl, keyval, _code, _state| {
            if keyval == gdk::Key::Escape {
                app_ref.quit();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        win.add_controller(key_ctl);

        // auto-dismiss 8s
        let app_ref = app.clone();
        glib::timeout_add_local_once(Duration::from_secs(8), move || {
            app_ref.quit();
        });
    });

    app.run_with_args::<&str>(&[]);
    Ok(())
}

fn cursor_pos() -> Option<(i32, i32)> {
    let out = Command::new("hyprctl")
        .arg("cursorpos")
        .output()
        .ok()?;
    let text = String::from_utf8(out.stdout).ok()?;
    let parts: Vec<&str> = text.trim().split(',').collect();
    if parts.len() >= 2 {
        Some((parts[0].trim().parse().ok()?, parts[1].trim().parse().ok()?))
    } else {
        None
    }
}
