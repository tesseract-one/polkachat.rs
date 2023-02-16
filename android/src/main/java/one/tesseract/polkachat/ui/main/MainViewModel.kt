package one.tesseract.polkachat.ui.main

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class MainViewModel : ViewModel() {
    private val _hello = MutableLiveData<String>().apply {
        value = "hello world"
    }
    val hello: MutableLiveData<String> = _hello
}