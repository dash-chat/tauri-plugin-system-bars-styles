package org.dashchat.systembarsstyles

import android.app.Activity
import android.graphics.Color
import android.os.Build
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
    var navigationBarTransparent: Boolean = false
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

            if (args.navigationBarTransparent) {
                window.navigationBarColor = Color.TRANSPARENT
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                    window.isNavigationBarContrastEnforced = false
                }
            }
        }

        invoke.resolve()
    }
}
