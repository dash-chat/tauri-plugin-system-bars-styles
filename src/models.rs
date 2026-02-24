use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BarStyle {
    Light,
    Dark,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStylePayload {
    pub status_bar_style: BarStyle,
    pub navigation_bar_style: BarStyle,
}
