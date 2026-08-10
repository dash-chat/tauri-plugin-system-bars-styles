import Tauri
import UIKit
import WebKit

/// Mirror of the scheme the Rust store holds. Read at launch, before Rust has
/// opened its own store, so the window can be themed before it is first drawn —
/// the counterpart of the Android side's `SharedPreferences` copy.
private let colorSchemeKey = "tauri-plugin-system-theme.color-scheme"

class SetColorSchemePreferenceArgs: Decodable {
    let scheme: String
}

/// The bars override is given up by passing no scheme at all.
class OverrideSystemBarsColorSchemeArgs: Decodable {
    let scheme: String?
}

class SystemThemePlugin: Plugin {
    private var windowObserver: NSObjectProtocol?

    /// Plugins are initialised before Tauri creates any window, so staging the
    /// stored style here is what keeps the app's own scheme — rather than the
    /// system's — on screen from the very first frame.
    override init() {
        super.init()
        applyPersisted()
    }

    deinit {
        if let windowObserver {
            NotificationCenter.default.removeObserver(windowObserver)
        }
    }

    /// The webview is left opaque on purpose. A host page may paint nothing at
    /// the document level, in which case a see-through webview exposes the
    /// window wherever the page's own surface fails to cover — which a
    /// keyboard-driven viewport change reliably causes, since such surfaces are
    /// typically `position: fixed`. Opaque, the webview paints its own base in
    /// the scheme set below, so it is its own backstop.
    override func load(webview: WKWebView) {
        if let window = webview.window {
            Self.apply(Self.storedStyle(), to: window)
        }
    }

    @objc public func setColorSchemePreference(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SetColorSchemePreferenceArgs.self)
        UserDefaults.standard.set(args.scheme, forKey: colorSchemeKey)

        let style = Self.style(for: args.scheme)
        DispatchQueue.main.async {
            Self.applyToAllWindows(style)
        }

        invoke.resolve()
    }

    /// The status bar tracks the window's interface style on iOS, so there is
    /// nothing to force independently of it.
    @objc public func overrideSystemBarsColorScheme(_ invoke: Invoke) throws {
        _ = try invoke.parseArgs(OverrideSystemBarsColorSchemeArgs.self)
        invoke.resolve()
    }

    private func applyPersisted() {
        // Tauri's window does not exist yet; catching it as it appears is what
        // gets the style in before the first draw. With no stored scheme the
        // style is `.unspecified`, which is what following the system means.
        windowObserver = NotificationCenter.default.addObserver(
            forName: UIWindow.didBecomeVisibleNotification,
            object: nil,
            queue: .main
        ) { notification in
            guard let window = notification.object as? UIWindow,
                window.windowLevel == .normal
            else { return }
            Self.apply(Self.storedStyle(), to: window)
        }

        let style = Self.storedStyle()
        DispatchQueue.main.async {
            Self.applyToAllWindows(style)
        }
    }

    private static func storedStyle() -> UIUserInterfaceStyle {
        style(for: UserDefaults.standard.string(forKey: colorSchemeKey) ?? "system")
    }

    private static func style(for scheme: String) -> UIUserInterfaceStyle {
        switch scheme {
        case "light": return .light
        case "dark": return .dark
        default: return .unspecified
        }
    }

    /// Only the app's own windows. The system puts the keyboard in windows of
    /// its own (`UIRemoteKeyboardWindow`, `UITextEffectsWindow`) that show up in
    /// the same scene and post the same notifications; restyling those breaks
    /// their rendering, and they are not ours to theme. They sit above
    /// `.normal`, which is what separates them.
    private static func appWindows() -> [UIWindow] {
        UIApplication.shared.connectedScenes
            .compactMap { $0 as? UIWindowScene }
            .flatMap(\.windows)
            .filter { $0.windowLevel == .normal }
    }

    private static func applyToAllWindows(_ style: UIUserInterfaceStyle) {
        for window in appWindows() {
            apply(style, to: window)
        }
    }

    /// Overriding the *window's* interface style rather than painting views a
    /// colour keeps every system-drawn surface — and the webview's
    /// `prefers-color-scheme` — consistent with the app's own theme.
    private static func apply(_ style: UIUserInterfaceStyle, to window: UIWindow) {
        window.overrideUserInterfaceStyle = style
        // `systemGroupedBackground` is the system colour the app's own surfaces
        // are modelled on, and being dynamic it resolves against the style set
        // just above with nothing to keep in sync by hand.
        window.backgroundColor = .systemGroupedBackground
    }
}

@_cdecl("init_plugin_system_theme")
func initPlugin() -> Plugin {
    return SystemThemePlugin()
}
