package one.tesseract.polkachat

import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.future.asDeferred
import kotlinx.coroutines.launch
import one.tesseract.polkachat.rust.Core

class MainViewModel : ViewModel() {
    lateinit var core: Core

    private val _messages = mutableStateListOf<String>()
    val messages: List<String> = _messages

    private val _account = mutableStateOf<String?>(null)
    val account: State<String?> = _account

    private val _failure = MutableSharedFlow<String>(replay = 0)
    val failure: SharedFlow<String> = _failure

    init {
        _messages.add("One")
        _messages.add("Two")
        _messages.add("Three")
        _messages.add("Four")
    }

    private suspend fun error(message: String) {
        _failure.emit(message)
    }

    fun login() {
        viewModelScope.launch {
            try {
                _account.value = core.account().asDeferred().await()
            } catch (e: Exception) {
                val message = e.message ?: ""
                if (!message.contains("Cancelled Tesseract error")) {
                    error(message)
                }
            }
        }
    }

    fun sendMessage(message: String) {
        _messages.add(message)
    }
}