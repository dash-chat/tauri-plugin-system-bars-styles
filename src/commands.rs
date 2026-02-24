use tauri::{command, AppHandle, Runtime};

use crate::models::BarStyle;

#[command]
pub(crate) async fn set_style<R: Runtime>(
    #[allow(unused_variables)] app: AppHandle<R>,
    #[allow(unused_variables)] status_bar_style: BarStyle,
    #[allow(unused_variables)] navigation_bar_style: BarStyle,
) -> crate::Result<()> {
    #[cfg(mobile)]
    {
        use crate::SystemBarsStylesExt;
        app.system_bars_styles().set_style(
            crate::models::SetStylePayload {
                status_bar_style,
                navigation_bar_style,
            },
        )?;
    }
    Ok(())
}
