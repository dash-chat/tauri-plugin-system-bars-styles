package org.dashchat.systembarsstyles

import android.app.Activity
import androidx.core.view.WindowCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@InvokeArg
class SetStyleArgs {
    lateinit var statusBarStyle: String
    lateinit var navigationBarStyle: String
}

@TauriPlugin
class SystemBarsStylesPlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun setStyle(invoke: Invoke) {
        val args = invoke.parseArgs(SetStyleArgs::class.java)

        activity.runOnUiThread {
            val window = activity.window
            val controller = WindowCompat.getInsetsController(window, window.decorView)

            // "dark" style = dark icons = isAppearanceLightBars(true)
            // "light" style = light icons = isAppearanceLightBars(false)
            controller.isAppearanceLightStatusBars = (args.statusBarStyle == "dark")
            controller.isAppearanceLightNavigationBars = (args.navigationBarStyle == "dark")
        }

        invoke.resolve()
    }
}
