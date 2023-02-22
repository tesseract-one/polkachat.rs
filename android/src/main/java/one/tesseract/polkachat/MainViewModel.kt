package one.tesseract.polkachat

import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.future.asDeferred
import kotlinx.coroutines.launch
import one.tesseract.polkachat.rust.Core

class MainViewModel: ViewModel() {
    lateinit var core: Core

    private val _messages = mutableStateListOf<String>()
    val messages: List<String> = _messages

    private val _account = mutableStateOf<String?>(null)
    val account: State<String?> = _account

    init {
        _messages.add("One")
        _messages.add("Two")
        _messages.add("Three")
        _messages.add("Four")
    }

    fun login() {
        viewModelScope.launch {
            _account.value = core.account().asDeferred().await()
        }
    }

    fun sendMessage(message: String) {
        _messages.add(message)
    }
}