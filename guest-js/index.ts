import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  type ReactiveFn,
  type ReactivePromise,
  reactive,
  relay,
  signal,
} from "signalium";

export type ColorScheme = "light" | "dark";
export type ColorSchemePreference = ColorScheme | "system";

const darkQuery =
  typeof window === "undefined"
    ? undefined
    : window.matchMedia("(prefers-color-scheme: dark)");

const appliedSignal = signal<ColorScheme>(
  darkQuery?.matches ? "dark" : "light",
);

darkQuery?.addEventListener("change", (event) => {
  appliedSignal.value = event.matches ? "dark" : "light";
});

/**
 * The theme actually in effect.
 *
 * The colour scheme is applied to the platform itself, so the webview reports
 * it: synchronous, and correct on the very first frame with no round trip to
 * wait on. Tauri's own `theme()`/`onThemeChanged()` cannot serve this — they are
 * async, and unsupported on Android and iOS.
 */
export const colorScheme = reactive((): ColorScheme => appliedSignal.value);

/**
 * The stored preference, for a UI that offers the choice.
 */
export const colorSchemePreference: ReactiveFn<
  ReactivePromise<ColorSchemePreference>,
  []
> = reactive(() =>
  relay<ColorSchemePreference>((state) => {
    invoke<ColorSchemePreference>(
      "plugin:system-theme|get_color_scheme_preference",
    )
      .then((scheme) => {
        state.value = scheme;
      })
      .catch((e) => {
        state.setError(e);
      });

    const unlisten = listen<ColorSchemePreference>(
      "system-theme://changed",
      (event) => {
        state.value = event.payload;
      },
    );

    return () => {
      unlisten.then((u) => u()).catch(() => {});
    };
  }),
);

/**
 * Apply and persist the app's colour scheme at the system level, so every
 * native surface follows it — including the window the system draws before the
 * webview exists, and the status and navigation bar icons.
 */
export async function setColorSchemePreference(
  scheme: ColorSchemePreference,
): Promise<void> {
  await invoke("plugin:system-theme|set_color_scheme_preference", { scheme });
}

/**
 * Force the status and navigation bar icon colour, for overlays whose
 * background ignores the app theme. Transient — never persisted. Pass `null`
 * (or call `setColorSchemePreference`) to track the app theme again.
 */
export async function overrideSystemBarsColorScheme(
  scheme: ColorScheme | null,
): Promise<void> {
  await invoke("plugin:system-theme|override_system_bars_color_scheme", { scheme });
}
