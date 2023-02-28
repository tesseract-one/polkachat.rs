package one.tesseract.polkachat

class UI {
    lateinit var model: MainViewModel

    @Suppress("unused") //used in native code
    fun presentError(message: String) {
        model.presentError(message)
    }
}