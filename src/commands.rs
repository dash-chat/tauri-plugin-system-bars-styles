use tauri::{command, AppHandle, Emitter, Runtime};

use crate::models::{ColorScheme, ColorSchemePreference};

/// Emitted whenever the colour scheme changes, so a frontend can track it
/// without polling. Mirrors the convention of the official plugins.
const CHANGED_EVENT: &str = "system-theme://changed";

#[command]
pub(crate) async fn get_color_scheme_preference<R: Runtime>(app: AppHandle<R>) -> crate::Result<ColorSchemePreference> {
    Ok(crate::store::load(&app))
}

#[command]
pub(crate) async fn set_color_scheme_preference<R: Runtime>(
    app: AppHandle<R>,
    scheme: ColorSchemePreference,
) -> crate::Result<()> {
    crate::store::save(&app, scheme)?;
    crate::apply_color_scheme_preference(&app, scheme)?;
    app.emit(CHANGED_EVENT, scheme)?;
    Ok(())
}

#[command]
pub(crate) async fn override_system_bars_color_scheme<R: Runtime>(
    #[allow(unused_variables)] app: AppHandle<R>,
    #[allow(unused_variables)] scheme: Option<ColorScheme>,
) -> crate::Result<()> {
    #[cfg(mobile)]
    {
        use crate::SystemThemeExt;
        app.system_theme()
            .override_system_bars_color_scheme(crate::models::OverrideSystemBarsColorSchemePayload { scheme })?;
    }
    Ok(())
}
