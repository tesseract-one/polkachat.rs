package one.tesseract.polkachat

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.ViewModelProvider
import one.tesseract.polkachat.ui.components.Messages
import one.tesseract.polkachat.ui.components.SignIn
import one.tesseract.polkachat.ui.components.UserControls
import one.tesseract.polkachat.ui.theme.PolkaChatTheme

class MainActivity : ComponentActivity() {
    companion object {
        init {
            System.loadLibrary("polkachat")
        }
    }

    external fun test()

    @OptIn(ExperimentalAnimationApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val vm = ViewModelProvider(this).get(MainViewModel::class.java)

        setContent {
            PolkaChatTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colors.background
                ) {
                    Column(
                        modifier = Modifier
                            .fillMaxSize()
                            .padding(all = 24.dp)
                    ) {
                        Text(
                            text = "Polkadot Demo dApp",
                            fontSize = 24.sp,
                            modifier = Modifier.padding(bottom = 24.dp)
                        )

                        Text(
                            text = "This dApp is a simple chat room made with smart contracts on the Polkadot network.",
                            modifier = Modifier.padding(bottom = 24.dp)
                        )

                        Messages(
                            messages = vm.messages, modifier = Modifier.weight(1f)
                        )

                        Box(modifier = Modifier.padding(vertical = 8.dp)) {
                            AnimatedContent(targetState = vm.account.value) { acid ->
                                if (acid != null) {
                                    UserControls(accountId = acid, send = vm::sendMessage)
                                } else {
                                    SignIn {
                                        vm.account.value = "myacc"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        test()
    }
}