import {
  overrideSystemBarsColorScheme,
  setColorSchemePreference,
} from "tauri-plugin-system-theme";
import type {
  ColorScheme,
  ColorSchemePreference,
} from "tauri-plugin-system-theme";

const schemeSelect = document.getElementById("scheme") as HTMLSelectElement;
const barsSelect = document.getElementById("bars") as HTMLSelectElement;
const applyBarsButton = document.getElementById("apply-bars")!;
const statusEl = document.getElementById("status")!;

function report(message: string) {
  statusEl.textContent = message;
}

schemeSelect.addEventListener("change", async () => {
  const scheme = schemeSelect.value as ColorSchemePreference;
  try {
    await setColorSchemePreference(scheme);
    report(`Colour scheme: ${scheme} (persisted)`);
  } catch (e) {
    report(`Error: ${e}`);
  }
});

applyBarsButton.addEventListener("click", async () => {
  const value = barsSelect.value;
  const scheme = value === "auto" ? null : (value as ColorScheme);
  try {
    await overrideSystemBarsColorScheme(scheme);
    report(`Bars: ${scheme ?? "tracking the theme"} (transient)`);
  } catch (e) {
    report(`Error: ${e}`);
  }
});
