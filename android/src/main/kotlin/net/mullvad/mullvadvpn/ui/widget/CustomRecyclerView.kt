package net.mullvad.mullvadvpn.ui.widget

import android.content.Context
import android.support.v7.widget.RecyclerView
import android.util.AttributeSet
import android.view.View
import kotlin.properties.Delegates.observable
import net.mullvad.mullvadvpn.util.ListenableScrollableView

class CustomRecyclerView : RecyclerView, ListenableScrollableView {
    private val headerReadyCallback = object : OnLayoutChangeListener {
        override fun onLayoutChange(
            view: View,
            left: Int,
            top: Int,
            right: Int,
            bottom: Int,
            oldLeft: Int,
            oldTop: Int,
            oldRight: Int,
            oldBottom: Int
        ) {
            synchronized(this@CustomRecyclerView) {
                headerAvailable = true
                onHeaderAvailable?.invoke()
                removeOnLayoutChangeListener(this)
            }
        }
    }

    private var headerAvailable = false

    override var horizontalScrollOffset = 0
    override var verticalScrollOffset = 0

    override var onScrollListener: ((Int, Int, Int, Int) -> Unit)? = null

    var onHeaderAvailable by observable<(() -> Unit)?>(null) { _, _, listener ->
        synchronized(this) {
            if (headerAvailable) {
                listener?.invoke()
            }
        }
    }

    constructor(context: Context) : super(context) {}

    constructor(context: Context, attributes: AttributeSet) : super(context, attributes) {}

    constructor(context: Context, attributes: AttributeSet, defaultStyleAttribute: Int) :
        super(context, attributes, defaultStyleAttribute) {
    }

    init {
        addOnLayoutChangeListener(headerReadyCallback)
    }

    override fun onScrolled(horizontalDelta: Int, verticalDelta: Int) {
        super.onScrolled(horizontalDelta, verticalDelta)

        val oldHorizontalScrollOffset = horizontalScrollOffset
        val oldVerticalScrollOffset = verticalScrollOffset

        horizontalScrollOffset += horizontalDelta
        verticalScrollOffset += verticalDelta

        onScrollListener?.invoke(
            horizontalScrollOffset,
            verticalScrollOffset,
            oldHorizontalScrollOffset,
            oldVerticalScrollOffset
        )
    }
}
