package one.tesseract.polkachat

import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.receiveAsFlow
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

    private val _failure = Channel<String>()
    val failure: Flow<String> = _failure.receiveAsFlow()

    init {
        val messagesState = _messages
        this.viewModelScope.launch {
            while (true) {
                try {
                    val messages = core.messages().await()
                    messagesState.clear()
                    messagesState.addAll(messages)
                } catch (e: Exception) {
                    val message = e.message ?: ""
                    if (!message.contains("Cancelled Tesseract error")) {
                        error(message)
                    }
                }
                delay(5000L)
            }
        }
    }

    private suspend fun error(message: String) {
        _failure.send(message)
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