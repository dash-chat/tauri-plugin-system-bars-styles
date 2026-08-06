use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::{OverrideSystemBarsColorSchemePayload, SetColorSchemePreferencePayload};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "org.dashchat.systemtheme";

#[cfg(target_os = "android")]
pub struct SystemTheme<R: Runtime>(PluginHandle<R>);

#[cfg(target_os = "ios")]
pub struct SystemTheme<R: Runtime>(AppHandle<R>);

pub fn init<R: Runtime, C: DeserializeOwned>(
    #[cfg(target_os = "android")] _app: &AppHandle<R>,
    #[cfg(target_os = "ios")] app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<SystemTheme<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "SystemThemePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = app.clone();
    #[cfg(target_os = "ios")]
    let _ = api;
    Ok(SystemTheme(handle))
}

#[cfg(target_os = "android")]
impl<R: Runtime> SystemTheme<R> {
    pub fn set_color_scheme_preference(&self, payload: SetColorSchemePreferencePayload) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setColorSchemePreference", payload)
            .map_err(Into::into)
    }

    pub fn override_system_bars_color_scheme(&self, payload: OverrideSystemBarsColorSchemePayload) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("overrideSystemBarsColorScheme", payload)
            .map_err(Into::into)
    }
}

#[cfg(target_os = "ios")]
impl<R: Runtime> SystemTheme<R> {
    /// Overriding the *window's* interface style rather than painting views a
    /// colour keeps every system-drawn surface — and the webview's
    /// `prefers-color-scheme` — consistent with the app's own theme.
    pub fn set_color_scheme_preference(&self, payload: SetColorSchemePreferencePayload) -> crate::Result<()> {
        use objc2::msg_send;
        use objc2::rc::Retained;
        use objc2_ui_kit::{UIColor, UIScrollView, UIUserInterfaceStyle, UIView};
        use tauri::Manager;

        use crate::models::ColorSchemePreference;

        let style = match payload.scheme {
            ColorSchemePreference::Light => UIUserInterfaceStyle::Light,
            ColorSchemePreference::Dark => UIUserInterfaceStyle::Dark,
            ColorSchemePreference::System => UIUserInterfaceStyle::Unspecified,
        };

        let Some(window) = self.0.get_webview_window("main") else {
            log::error!("No main webview window to apply the colour scheme to");
            return Ok(());
        };

        window.with_webview(move |webview| unsafe {
            let view = &*(webview.inner() as *mut UIView);
            let Some(window) = view.window() else {
                log::error!("The webview is not in a window yet");
                return;
            };
            window.setOverrideUserInterfaceStyle(style);

            // What shows until the UI has rendered is the window, so the webview
            // has to stay see-through. `systemGroupedBackground` is the system
            // colour the app's own surfaces are modelled on, and being dynamic it
            // resolves against the style set just above with nothing to keep in
            // sync by hand.
            window.setBackgroundColor(Some(&UIColor::systemGroupedBackgroundColor()));

            let clear = UIColor::clearColor();
            view.setOpaque(false);
            view.setBackgroundColor(Some(&clear));
            let scroll_view: Retained<UIScrollView> = msg_send![view, scrollView];
            scroll_view.setBackgroundColor(Some(&clear));
        })?;

        Ok(())
    }

    /// The status bar tracks the window's interface style on iOS, so there is
    /// nothing to force independently of it.
    pub fn override_system_bars_color_scheme(&self, _payload: OverrideSystemBarsColorSchemePayload) -> crate::Result<()> {
        Ok(())
    }
}
