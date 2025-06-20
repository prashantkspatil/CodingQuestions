package pp.example.codingqna

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import pp.example.codingqna.question_list.QuestionsViewModel
import pp.example.codingqna.question_list.UiEvent
import pp.example.codingqna.ui.theme.CodingQuestionsTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            CodingQuestionsTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    val viewModel by viewModels<QuestionsViewModel>()
                    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
                    LazyColumn(
                        modifier = Modifier
                            .fillMaxSize()
                            .padding(innerPadding)
                    ) {
                        items(uiState.questions, key = { it.question }) {
                            var qState by remember(it.question) {
                                mutableStateOf(it.isCorrect)
                            }
                            Column (
                                modifier = Modifier.fillMaxWidth()
                                    .clickable(
                                        onClick = {
                                            viewModel.onEvent(UiEvent.OnQuestionClicked(it))
                                            qState = it.isCorrect
                                        }
                                    )
                                    .background(
                                        color = if (qState == true) {
                                                MaterialTheme.colorScheme.primary
                                            } else if (qState == false) {
                                                MaterialTheme.colorScheme.error
                                            } else {
                                                MaterialTheme.colorScheme.surface
                                            }
                                    )
                            ) {
                                Text(
                                    text = it.question,
                                    color = if (qState == true) {
                                        MaterialTheme.colorScheme.onPrimary
                                    } else if (qState == false) {
                                        MaterialTheme.colorScheme.onError
                                    } else {
                                        MaterialTheme.colorScheme.onSurface
                                    },
                                    style = MaterialTheme.typography.labelLarge,
                                    modifier = Modifier.padding(10.dp).height(40.dp)
                                )
                            }
                            HorizontalDivider()
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    CodingQuestionsTheme {
        Greeting("Android")
    }
}