package pp.example.codingqna.model

data class Question(
    val question: String,
    var isAnswered: Boolean = false,
    var isCorrect: Boolean? = null,
    val executer: (Question) -> Boolean,
)
