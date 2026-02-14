#!/usr/bin/env python3
"""Quick test: red overlay on waybar with drag source."""
import gi, os, signal
gi.require_version("Gtk", "4.0")
gi.require_version("Gdk", "4.0")
gi.require_version("Gtk4LayerShell", "1.0")
from gi.repository import Gtk, Gdk, Gio, GLib, Gtk4LayerShell

TEST_FILE = os.path.expanduser("~/Pictures/Screenshots/20260213_193932.png")

class App(Gtk.Application):
    def __init__(self):
        super().__init__(application_id="dev.test.drag")
    def do_activate(self):
        css = Gtk.CssProvider()
        css.load_from_string(".test-drag { background: rgba(255,0,0,0.3); color: white; }")
        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(), css, Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION)

        win = Gtk.Window(application=self)
        Gtk4LayerShell.init_for_window(win)
        Gtk4LayerShell.set_layer(win, Gtk4LayerShell.Layer.OVERLAY)
        Gtk4LayerShell.set_anchor(win, Gtk4LayerShell.Edge.TOP, True)
        Gtk4LayerShell.set_anchor(win, Gtk4LayerShell.Edge.RIGHT, True)
        Gtk4LayerShell.set_margin(win, Gtk4LayerShell.Edge.TOP, 6)
        Gtk4LayerShell.set_margin(win, Gtk4LayerShell.Edge.RIGHT, 280)
        Gtk4LayerShell.set_exclusive_zone(win, -1)
        Gtk4LayerShell.set_namespace(win, "test-drag")
        win.set_decorated(False)
        win.add_css_class("test-drag")

        area = Gtk.Label(label="DRAG ME")
        area.set_size_request(230, 51)

        ds = Gtk.DragSource()
        ds.set_actions(Gdk.DragAction.COPY)
        ds.connect("prepare", self._prep)
        ds.connect("drag-begin", self._begin)
        area.add_controller(ds)
        win.set_child(area)
        win.present()

        GLib.timeout_add(20000, lambda: self.quit())
        GLib.unix_signal_add(GLib.PRIORITY_DEFAULT, signal.SIGINT, lambda: self.quit())
        self.hold()

    def _prep(self, src, x, y):
        uri = Gio.File.new_for_path(TEST_FILE).get_uri() + "\r\n"
        print(f"Drag prepare! uri={uri.strip()}")
        return Gdk.ContentProvider.new_for_bytes("text/uri-list", GLib.Bytes.new(uri.encode()))

    def _begin(self, src, drag):
        print("Drag begin!")

App().run(None)
