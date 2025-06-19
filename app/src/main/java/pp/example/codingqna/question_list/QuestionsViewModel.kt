package pp.example.codingqna.question_list

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import pp.example.codingqna.data.QuestionRepository

class QuestionsViewModel(
    private val repository: QuestionRepository = QuestionRepository()
): ViewModel() {
    private val _uiState = MutableStateFlow(QuestionsUiState())
    val uiState: StateFlow<QuestionsUiState> = _uiState.asStateFlow()

    init {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true) }
            _uiState.update { it.copy(
                questions = repository.getQuestions(),
            ) }
            _uiState.update { it.copy(isLoading = false) }
        }
    }

    fun onEvent(event: UiEvent) {
        when(event) {
            is UiEvent.OnQuestionClicked -> {
                event.question.isCorrect = event.question.executer(event.question)
            }
        }
    }
}