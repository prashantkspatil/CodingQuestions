package pp.example.codingqna.question_list

import pp.example.codingqna.model.Question

sealed class UiEvent {
    data class OnQuestionClicked(val question: Question): UiEvent()
}
