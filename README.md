# tauri-plugin-system-bars-styles

A Tauri plugin to independently control the appearance of Android status bar and navigation bar icons.

Unlike other plugins that set both bars to the same style, this plugin lets you control the status bar and navigation bar independently — useful when you want light navigation bar icons (for a transparent nav bar) but dark status bar icons (for a light-themed app).

## Platform Support

| Platform | Status |
|----------|--------|
| Android  | Supported |
| iOS      | No-op |
| Desktop  | No-op |

## Installation

### Rust

Add the dependency to your `src-tauri/Cargo.toml`:

```toml
[target.'cfg(any(target_os = "android", target_os = "ios"))'.dependencies]
tauri-plugin-system-bars-styles = { git = "https://github.com/dash-chat/tauri-plugin-system-bars-styles", branch = "main" }
```

### Capabilities

Add the permission to your capabilities file:

```json
{
  "permissions": [
    "system-bars-styles:default"
  ]
}
```

## Usage

### Rust setup

Register the plugin in your Tauri app:

```rust
#[cfg(mobile)]
{
    builder = builder.plugin(tauri_plugin_system_bars_styles::init());
}
```

### Frontend

Invoke the command directly from JavaScript/TypeScript:

```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('plugin:system-bars-styles|set_style', {
  statusBarStyle: 'dark',       // dark icons (for light backgrounds)
  navigationBarStyle: 'light',  // light icons (for dark backgrounds)
});
```

### Style values

| Value    | Icons          | Use with               |
|----------|----------------|------------------------|
| `"dark"` | Black icons    | Light backgrounds      |
| `"light"`| White icons    | Dark backgrounds       |

## How it works

On Android, the plugin uses `WindowCompat.getInsetsController()` to set `isAppearanceLightStatusBars` and `isAppearanceLightNavigationBars` independently. Only depends on `androidx.core` — no Material3 or Compose dependencies.

## License

AGPL-3.0
