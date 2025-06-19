package pp.example.codingqna.question_list

import pp.example.codingqna.model.Question

data class QuestionsUiState(
    val questions: List<Question> = emptyList(),
    val isLoading: Boolean = false,
    val errorMessage: String? = null
)
