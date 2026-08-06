package org.dashchat.systemtheme

import android.app.Activity
import android.content.res.Configuration
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@InvokeArg
class SchemeArgs {
    lateinit var scheme: String
}

/** The bars override is given up by passing no scheme at all. */
@InvokeArg
class OptionalSchemeArgs {
    var scheme: String? = null
}

@TauriPlugin
class SystemThemePlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun setColorSchemePreference(invoke: Invoke) {
        val args = invoke.parseArgs(SchemeArgs::class.java)

        activity.runOnUiThread {
            SystemTheme.setColorSchemePreference(activity, activity.window, args.scheme)
        }

        invoke.resolve()
    }

    @Command
    fun overrideSystemBarsColorScheme(invoke: Invoke) {
        val args = invoke.parseArgs(OptionalSchemeArgs::class.java)

        activity.runOnUiThread {
            SystemTheme.overrideSystemBarsColorScheme(activity, activity.window, args.scheme)
        }

        invoke.resolve()
    }

    override fun onConfigurationChanged(newConfig: Configuration) {
        activity.runOnUiThread {
            SystemTheme.onConfigurationChanged(activity.window, newConfig)
        }
    }
}
