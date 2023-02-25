package one.tesseract.polkachat

import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.future.asDeferred
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import one.tesseract.polkachat.rust.Core

//TODO: double check that errors are caught everywhere
class MainViewModel(private val core: Core) : ViewModel() {
    private val _messages = mutableStateListOf<String>()
    val messages: List<String> = _messages

    private val _account = mutableStateOf<String?>(null)
    val account: State<String?> = _account

    private val _failure = MutableSharedFlow<String>(replay = 0)
    val failure: SharedFlow<String> = _failure

    init {
        val messagesState = _messages
        this.viewModelScope.launch {
            val messages = core.messages().await()
            messagesState.addAll(messages)
        }
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
        viewModelScope.launch {
            try {
                core.send(message).await()
            } catch (e: Exception) {
                @Suppress("NAME_SHADOWING") val message = e.message ?: ""
                if (!message.contains("Cancelled Tesseract error")) {
                    error(message)
                }
            }
        }
    }
}

class MainViewModelFactory(private val core: Core) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if(modelClass.isAssignableFrom(MainViewModel::class.java)){
            @Suppress("UNCHECKED_CAST")
            return MainViewModel(core) as T
        }
        throw TypeCastException("Can't create view model ${MainViewModel::class.java.name} and cast to ${modelClass.name}")
    }
}