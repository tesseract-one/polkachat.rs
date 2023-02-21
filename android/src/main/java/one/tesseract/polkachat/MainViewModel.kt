package one.tesseract.polkachat

import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.lifecycle.ViewModel

class MainViewModel: ViewModel() {
    private val _messages = mutableStateListOf<String>()
    val messages: SnapshotStateList<String> = _messages

    private val _account = mutableStateOf<String?>(null)
    val account: MutableState<String?> = _account

    fun sendMessage(message: String) {
        messages.add(message)
    }
}