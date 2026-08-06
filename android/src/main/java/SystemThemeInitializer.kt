package org.dashchat.systemtheme

import android.content.ContentProvider
import android.content.ContentValues
import android.database.Cursor
import android.net.Uri

/**
 * Applies the stored colour scheme before the first activity exists.
 *
 * Content providers are instantiated ahead of `Application.onCreate`, which is
 * the only hook early enough to theme the window on the API levels where the
 * system does not persist the app's night mode itself. Declaring it in this
 * library's manifest means the host app needs no wiring at all.
 */
class SystemThemeInitializer : ContentProvider() {
    override fun onCreate(): Boolean {
        context?.let { SystemTheme.applyPersisted(it) }
        return true
    }

    override fun query(
        uri: Uri,
        projection: Array<out String>?,
        selection: String?,
        selectionArgs: Array<out String>?,
        sortOrder: String?,
    ): Cursor? = null

    override fun getType(uri: Uri): String? = null

    override fun insert(uri: Uri, values: ContentValues?): Uri? = null

    override fun delete(uri: Uri, selection: String?, selectionArgs: Array<out String>?): Int = 0

    override fun update(
        uri: Uri,
        values: ContentValues?,
        selection: String?,
        selectionArgs: Array<out String>?,
    ): Int = 0
}
