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
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.ViewModelProvider
import one.tesseract.polkachat.rust.Core
import one.tesseract.polkachat.ui.components.Messages
import one.tesseract.polkachat.ui.components.SignIn
import one.tesseract.polkachat.ui.components.UserControls
import one.tesseract.polkachat.ui.theme.PolkaChatTheme

class MainActivity : ComponentActivity() {
    @OptIn(ExperimentalAnimationApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val core = Core.create(application, this.javaClass.classLoader!!)
        val vm = ViewModelProvider(this, MainViewModelFactory(core)).get(MainViewModel::class.java)

        setContent {
            PolkaChatTheme {
                val scaffoldState: ScaffoldState = rememberScaffoldState()

                LaunchedEffect(key1 = true) {
                    vm.failure.collect {
                        scaffoldState.snackbarHostState.showSnackbar(
                            message = it
                        )
                    }
                }

                Scaffold(scaffoldState = scaffoldState) { padding ->
                    // A surface container using the 'background' color from the theme
                    Surface(
                        color = MaterialTheme.colors.background,
                        modifier = Modifier
                            .padding(padding)
                            .fillMaxSize(),
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
                                messages = vm.messages,
                                modifier = Modifier
                                    .weight(1f)
                                    .fillMaxSize()
                            )

                            Box(modifier = Modifier.padding(vertical = 8.dp)) {
                                AnimatedContent(targetState = vm.account.value) { account ->
                                    if (account != null) {
                                        UserControls(account = account, send = vm::sendMessage)
                                    } else {
                                        SignIn(vm::login)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}