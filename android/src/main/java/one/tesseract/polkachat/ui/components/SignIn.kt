package one.tesseract.polkachat.ui.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun SignIn(signIn: () -> Unit) {
    Column {
        Text(
            text = "To start sending messages, please, sign in",
            modifier = Modifier.padding(vertical = 8.dp))

        Button(onClick = signIn) {
            Text(text = "Sign-in with Tesseract")
        }
    }
}