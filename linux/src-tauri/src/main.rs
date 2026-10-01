// Coucou runs without a console window: Mochi is the whole UI.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    choose_windowing_backend();
    coucou_lib::run()
}

/// A Wayland client is not allowed to say where its window goes, and the island
/// is nothing without its place along the top edge. Layer-shell is the proper
/// answer, but it is a compositor extension rather than core Wayland (GNOME does
/// not implement it at all), and the app does not speak it yet — so while that is
/// true, run under XWayland whenever the session is Wayland. X11 does let a
/// client position itself, and `apply_geometry` starts working again.
///
/// Two guards, both deliberate:
///   * `DISPLAY` must already be set — that is XWayland being present. Forcing the
///     x11 backend without it would stop the app from starting at all, which is
///     worse than not being pinned to the top edge;
///   * an explicit `GDK_BACKEND` is never overridden: whoever set it knows better.
fn choose_windowing_backend() {
    if std::env::var_os("GDK_BACKEND").is_some() {
        return;
    }
    let on_wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
    let has_xwayland = std::env::var_os("DISPLAY").is_some();
    if on_wayland && has_xwayland {
        // Safe: this runs before any thread exists (main, single-threaded).
        std::env::set_var("GDK_BACKEND", "x11");
    }
}
