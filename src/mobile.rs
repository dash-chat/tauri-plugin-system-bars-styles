use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::{OverrideSystemBarsColorSchemePayload, SetColorSchemePreferencePayload};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "org.dashchat.systemtheme";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_system_theme);

pub struct SystemTheme<R: Runtime>(PluginHandle<R>);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<SystemTheme<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "SystemThemePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_system_theme)?;
    Ok(SystemTheme(handle))
}

impl<R: Runtime> SystemTheme<R> {
    pub fn set_color_scheme_preference(
        &self,
        payload: SetColorSchemePreferencePayload,
    ) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setColorSchemePreference", payload)
            .map_err(Into::into)
    }

    pub fn override_system_bars_color_scheme(
        &self,
        payload: OverrideSystemBarsColorSchemePayload,
    ) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("overrideSystemBarsColorScheme", payload)
            .map_err(Into::into)
    }
}
