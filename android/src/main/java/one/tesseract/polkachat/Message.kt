package one.tesseract.polkachat

sealed interface Message {
    val text: String

    data class CommittedMessage(override val text: String): Message
    data class SubmittedMessage(override val text: String): Message {
        fun intoCommitted() = CommittedMessage(this.text)
    }
}