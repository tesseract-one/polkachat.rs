package one.tesseract.polkachat.rust

import android.app.Application
import one.tesseract.crabdroid.RustObject
import one.tesseract.polkachat.UI
import java.util.concurrent.CompletableFuture

class Core(handle: Long): RustObject(handle) {
    companion object {
        init {
            System.loadLibrary("polkachat")
        }

        @JvmStatic
        external fun create(application: Application, ui: UI, loader: ClassLoader): Core
    }

    external fun account(): CompletableFuture<String>
    external fun messages(from: Int): CompletableFuture<List<String>>
    external fun send(message: String): CompletableFuture<Unit>
}