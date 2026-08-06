use std::sync::OnceLock;

use tauri::{AppHandle, Manager, Runtime, Theme};

use crate::models::ColorSchemePreference;

static STARTUP_THEME: OnceLock<Theme> = OnceLock::new();

/// Remember the theme the app launched with, before any override is applied.
pub(crate) fn capture_startup_theme<R: Runtime>(app: &AppHandle<R>) {
    let Some(theme) = app
        .webview_windows()
        .values()
        .next()
        .and_then(|window| window.theme().ok())
    else {
        return;
    };
    let _ = STARTUP_THEME.set(theme);
}

pub(crate) fn set_color_scheme_preference<R: Runtime>(
    app: &AppHandle<R>,
    scheme: ColorSchemePreference,
) -> crate::Result<()> {
    let theme = match scheme {
        ColorSchemePreference::Light => Some(Theme::Light),
        ColorSchemePreference::Dark => Some(Theme::Dark),
        // Linux maps `None` onto Light rather than dropping the override
        // (tao's `WindowRequest::SetTheme`), so the theme the app launched with
        // stands in for the system's. It cannot track later changes to the
        // system theme the way `None` does elsewhere.
        ColorSchemePreference::System if cfg!(target_os = "linux") => STARTUP_THEME.get().copied(),
        ColorSchemePreference::System => None,
    };
    // App-level rather than per-window: it also re-themes native menus on Windows.
    app.set_theme(theme);
    Ok(())
}
