use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::SetStylePayload;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "org.dashchat.systembarsstyles";

#[cfg(target_os = "android")]
pub struct SystemBarsStyles<R: Runtime>(PluginHandle<R>);

#[cfg(target_os = "ios")]
pub struct SystemBarsStyles<R: Runtime>(AppHandle<R>);

pub fn init<R: Runtime, C: DeserializeOwned>(
    #[cfg(target_os = "android")] _app: &AppHandle<R>,
    #[cfg(target_os = "ios")] app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<SystemBarsStyles<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "SystemBarsStylesPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = app.clone();
    #[cfg(target_os = "ios")]
    let _ = api;
    Ok(SystemBarsStyles(handle))
}

impl<R: Runtime> SystemBarsStyles<R> {
    #[cfg(target_os = "android")]
    pub fn set_style(&self, payload: SetStylePayload) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("setStyle", payload)
            .map_err(Into::into)
    }

    #[cfg(target_os = "ios")]
    pub fn set_style(&self, _payload: SetStylePayload) -> crate::Result<()> {
        // No-op on iOS
        Ok(())
    }
}
