package one.tesseract.polkachat.rust

import android.app.Application
import one.tesseract.interop.rust.RustObject
import java.util.concurrent.CompletableFuture

class Core(handle: Long): RustObject(handle) {
    companion object {
        init {
            System.loadLibrary("polkachat")
        }

        @JvmStatic
        external fun create(/*ui: UI, */application: Application, loader: ClassLoader): Core
    }

    external fun account(): CompletableFuture<String>
    external fun messages(): CompletableFuture<List<String>>
    external fun send(message: String): CompletableFuture<Unit>
}