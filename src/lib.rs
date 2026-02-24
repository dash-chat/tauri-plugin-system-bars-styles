use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod commands;
mod error;
#[cfg(mobile)]
mod mobile;
pub mod models;

pub use error::{Error, Result};

#[cfg(mobile)]
mod ext {
    use tauri::{Manager, Runtime};

    pub trait SystemBarsStylesExt<R: Runtime> {
        fn system_bars_styles(&self) -> &super::mobile::SystemBarsStyles<R>;
    }

    impl<R: Runtime, T: Manager<R>> SystemBarsStylesExt<R> for T {
        fn system_bars_styles(&self) -> &super::mobile::SystemBarsStyles<R> {
            self.state::<super::mobile::SystemBarsStyles<R>>().inner()
        }
    }
}

#[cfg(mobile)]
pub use ext::SystemBarsStylesExt;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-bars-styles")
        .invoke_handler(tauri::generate_handler![commands::set_style])
        .setup(|app, api| {
            #[cfg(mobile)]
            {
                let plugin = mobile::init(app, api)?;
                app.manage(plugin);
            }
            #[cfg(desktop)]
            let _ = (app, api);
            Ok(())
        })
        .build()
}
