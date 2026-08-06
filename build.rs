const COMMANDS: &[&str] = &["get_color_scheme_preference", "set_color_scheme_preference", "override_system_bars_color_scheme"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
