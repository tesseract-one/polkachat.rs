package one.tesseract.polkachat.ui.components

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Card
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.flow.Flow
import one.tesseract.polkachat.Message

@Composable
fun Messages(messages: List<Message>, modifier: Modifier = Modifier, scrollTrigger: Flow<Unit>? = null) {
    val scrollState = rememberLazyListState()

    LaunchedEffect(key1 = messages.size) {
        if (messages.isNotEmpty()) {
            scrollState.animateScrollToItem(messages.size - 1)
        }
    }

    LaunchedEffect(key1 = scrollTrigger) {
        scrollTrigger?.collect {
            scrollState.animateScrollToItem(messages.size - 1)
        }
    }

    LazyColumn(
        state = scrollState,
        verticalArrangement = Arrangement.Bottom,
        modifier = modifier
    ) {
        items(items = messages) {
            Card(
                elevation = 10.dp,
                shape = RoundedCornerShape(16.dp),
                modifier = Modifier
                    .padding(top = 8.dp)
            ) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    val progress =  when(it) {
                        is Message.CommittedMessage -> false
                        is Message.SubmittedMessage -> true
                    }

                    Spacer(modifier = Modifier.requiredSize(8.dp)) //don't use align. CircularProgressIndicator is buggy with it
                    if(progress) {
                        CircularProgressIndicator(
                            strokeWidth = 2.dp,
                            modifier = Modifier.requiredSize(18.dp)
                        )
                    }
                    Text(
                        text = it.text,
                        textAlign = TextAlign.Left,
                        modifier = Modifier
                            .padding(all = 4.dp)
                            .padding(horizontal = 4.dp)
                    )
                    Spacer(modifier = Modifier.requiredSize(8.dp))
                }
            }
        }
    }
}