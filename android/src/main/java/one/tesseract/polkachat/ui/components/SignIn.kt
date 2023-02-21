package one.tesseract.polkachat.ui.components

import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable

@Composable
fun SignIn(signIn: () -> Unit) {
    Button(onClick = signIn) {
        Text(text = "Sign-in with Tesseract")
    }
}