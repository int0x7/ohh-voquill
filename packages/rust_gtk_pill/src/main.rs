mod constants;
mod draw;
mod custom_anim;
mod input;
mod ipc;
mod pill;
mod state;
mod x11;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    // GNOME Wayland has no layer-shell support, and the PlainWayland fallback
    // renders the loading phase incorrectly. XWayland behaves like plain X11
    // (which works, incl. HiDPI positioning), so re-exec ourselves under the
    // x11 backend. Compositors with layer-shell (KDE, sway) are unaffected,
    // and a set GDK_BACKEND means we already re-executed (no loop).
    let supported = gtk_layer_shell::is_supported();
    // The Voquill host sets GDK_BACKEND=wayland for its children, so treat
    // only our own "x11" as the already-re-executed marker.
    let already_x11 = std::env::var("GDK_BACKEND").map(|v| v == "x11").unwrap_or(false);
    if !already_x11 && !supported {
        use gtk::prelude::*;
        let is_x11 = gtk::gdk::Display::default()
            .map(|d| d.type_().name() == "GdkX11Display")
            .unwrap_or(false);
        if !is_x11 {
            if let Ok(exe) = std::env::current_exe() {
                use std::os::unix::process::CommandExt;
                let err = std::process::Command::new(exe).env("GDK_BACKEND", "x11").exec();
                eprintln!("re-exec with GDK_BACKEND=x11 failed ({err}), staying on wayland");
            }
        }
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    ipc::start_stdin_reader(sender);
    pill::run(receiver);
}
