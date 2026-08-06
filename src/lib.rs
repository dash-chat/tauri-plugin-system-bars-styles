use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};

mod commands;
#[cfg(desktop)]
mod desktop;
mod error;
#[cfg(mobile)]
mod mobile;
pub mod models;
mod store;

pub use error::{Error, Result};
use models::ColorSchemePreference;

#[cfg(mobile)]
mod ext {
    use tauri::{Manager, Runtime};

    pub trait SystemThemeExt<R: Runtime> {
        fn system_theme(&self) -> &super::mobile::SystemTheme<R>;
    }

    impl<R: Runtime, T: Manager<R>> SystemThemeExt<R> for T {
        fn system_theme(&self) -> &super::mobile::SystemTheme<R> {
            self.state::<super::mobile::SystemTheme<R>>().inner()
        }
    }
}

#[cfg(mobile)]
pub use ext::SystemThemeExt;

fn apply_color_scheme_preference<R: Runtime>(
    #[allow(unused_variables)] app: &AppHandle<R>,
    #[allow(unused_variables)] scheme: ColorSchemePreference,
) -> Result<()> {
    #[cfg(mobile)]
    app.system_theme()
        .set_color_scheme_preference(models::SetColorSchemePreferencePayload { scheme })?;
    #[cfg(desktop)]
    desktop::set_color_scheme_preference(app, scheme)?;
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-theme")
        .invoke_handler(tauri::generate_handler![
            commands::get_color_scheme_preference,
            commands::set_color_scheme_preference,
            commands::override_system_bars_color_scheme
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            {
                use tauri::Manager;
                let plugin = mobile::init(app, api)?;
                app.manage(plugin);
            }
            #[cfg(desktop)]
            let _ = (app, api);
            Ok(())
        })
        .on_webview_ready(|webview| {
            #[cfg(not(target_os = "android"))]
            {
                use tauri::Manager;
                let app = webview.app_handle();
                #[cfg(desktop)]
                desktop::capture_startup_theme(app);
                if let Err(err) = apply_color_scheme_preference(app, store::load(app)) {
                    log::error!("Failed to apply the stored colour scheme: {err:?}");
                }
            }
            #[cfg(target_os = "android")]
            let _ = webview;
        })
        .build()
}
