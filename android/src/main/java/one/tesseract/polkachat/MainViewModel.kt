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
    private val _messages = mutableStateListOf<Message>()
    val messages: List<Message> = _messages

    private val _account = mutableStateOf<String?>(null)
    val account: State<String?> = _account

    private val _failure = Channel<String>()
    val failure: Flow<String> = _failure.receiveAsFlow()

    init {
        val messagesState = _messages
        this.viewModelScope.launch {
            try {
                val committedSize = messagesState.filterIsInstance<Message.CommittedMessage>().size
                val messages = core.messages(committedSize).await().map { Message.CommittedMessage(it) }
                messagesState.addAll(messages)
            } catch (e: Exception) {
                val message = e.message ?: ""
                if (!message.contains("Cancelled Tesseract error")) {
                    error(message)
                }
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
        val message = Message.SubmittedMessage(message)

        viewModelScope.launch {
            try {
                _messages.add(message)
                core.send(message.text).await()
                val index = _messages.lastIndexOf(message)
                _messages[index] = message.intoCommitted()
            } catch (e: Exception) {
                @Suppress("NAME_SHADOWING") val error = e.message ?: ""
                if (!error.contains("panicked")) /*we can't know more due to subxt::Signer limitations */ {
                    error(error)
                }

                _messages.remove(message)
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