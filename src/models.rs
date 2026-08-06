use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorSchemePreference {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetColorSchemePreferencePayload {
    pub scheme: ColorSchemePreference,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OverrideSystemBarsColorSchemePayload {
    /// `None` gives up the override so the bars track the app theme again.
    pub scheme: Option<ColorScheme>,
}
