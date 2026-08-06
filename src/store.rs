use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

use crate::models::ColorSchemePreference;

const FILE_NAME: &str = "system-theme.json";

fn path<R: Runtime>(app: &AppHandle<R>) -> crate::Result<PathBuf> {
    Ok(app.path().app_local_data_dir()?.join(FILE_NAME))
}

pub(crate) fn load<R: Runtime>(app: &AppHandle<R>) -> ColorSchemePreference {
    let Ok(path) = path(app) else {
        return ColorSchemePreference::default();
    };
    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => ColorSchemePreference::default(),
        Err(err) => {
            log::error!("Failed to read the colour scheme at {path:?}: {err:?}");
            ColorSchemePreference::default()
        }
    }
}

pub(crate) fn save<R: Runtime>(app: &AppHandle<R>, scheme: ColorSchemePreference) -> crate::Result<()> {
    let path = path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_string(&scheme)?)?;
    Ok(())
}
