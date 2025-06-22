package pp.example.codingqna.data

import android.util.Log
import pp.example.codingqna.JniCall
import pp.example.codingqna.model.Question

class QuestionRepository {
    private val questionsList = mutableListOf<Question>()

    fun getQuestions(): List<Question> {
        return questionsList
    }

    private fun addQuestion(question: Question) {
        questionsList.add(question)
    }

    init {
        addQuestion(Question("Reverse a string") {
            val expected = "dlroW olleH"
            val result = JniCall.reverse_a_string("Hello World")
            it.isAnswered = true
            it.isCorrect = result == expected
            Log.d("QuestionRepository", "Reverse a string: $expected and result is $result")
            result == expected
        })

        addQuestion(Question("Check for Palindrome") {
            val expected = true
            val result = JniCall.check_for_palindrome("madam")
            it.isAnswered = true
            it.isCorrect = result == expected
            Log.d("QuestionRepository", "Check for Palindrome: $expected and result is $result")
            result == expected
        })

        addQuestion(Question("Find the maximum element in an array") {
            val expected = 5
            val result = JniCall.find_the_maximum_element_in_an_array(intArrayOf(1, 2, 3, 4, 5))
            it.isAnswered = true
            Log.d("QuestionRepository", "Find the maximum element in an array: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Fizz Buzz problem") {
            val expected = "1\n2\nfizz\n4\nbuzz\nfizz\n7\n8\nfizz\nbuzz\n11\nfizz\n13\n14\nfizzbuzz\n"
            val result = JniCall.fizz_buzz_problem(15)
            it.isAnswered = true
            Log.d("QuestionRepository", "Fizz Buzz problem: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Count vowels in a string") {
            val expected = 3
            val result = JniCall.count_vowels_in_a_string("Hello World")
            it.isAnswered = true
            Log.d("QuestionRepository", "Count vowels in a string: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Remove duplicates from the list") {
            val expected = intArrayOf(1, 2, 3, 4, 5)
            val result = JniCall.remove_duplicates_from_the_list(intArrayOf(1, 2, 2, 3, 4, 4, 5))
            it.isAnswered = true
            Log.d("QuestionRepository", "Remove duplicates from the list: ${expected.toList()} and result is ${result.toList()}")
            it.isCorrect = result.contentEquals(expected)
            result.contentEquals(expected)
        })

        addQuestion(Question("Find The First Non Repeated Char In String") {
            val expected = 'w'
            val result = JniCall.findTheFirstNonRepeatedCharInString("swiss")
            it.isAnswered = true
            Log.d("QuestionRepository", "Find The First Non Repeated Char In String: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Factorial Using Recursion") {
            val expected = 120
            val result = JniCall.factorialUsingRecursion(5)
            it.isAnswered = true
            Log.d("QuestionRepository", "Factorial Using Recursion: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Find the second largest number") {
            val expected = 8
            val result = JniCall.findTheSecondLargestNumber(intArrayOf(1, 5, 3, 6, 8, 12, 5))
            it.isAnswered = true
            Log.d("QuestionRepository", "Find the second largest number: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Sum of digits in number") {
            val expected = 6
            val result = JniCall.sumOfDigitsInNumber(15)
            it.isAnswered = true
            Log.d("QuestionRepository", "Sum of digits in number: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Find The Missing Number In Array") {
            val expected = 3
            val result = JniCall.findTheMissingNumberInArray(intArrayOf(1, 2, 4, 5))
            it.isAnswered = true
            Log.d("QuestionRepository", "Find The Missing Number In Array: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Check If Two Strings Are Anagram") {
            val expected = true
            val result = JniCall.checkIfTwoStringsAreAnagram("silent", "listen")
            it.isAnswered = true
            Log.d("QuestionRepository", "Check If Two Strings Are Anagram: $expected and result is $result")
            it.isCorrect = result == expected
            result == expected
        })

        addQuestion(Question("Flatten A List Of Integers") {
            val expected = intArrayOf(1, 2, 3, 4, 5, 6, 7, 8, 9)
            val result = JniCall.flattenAListOfIntegers(arrayOf(intArrayOf(1, 2, 3), intArrayOf(4, 5, 6), intArrayOf(7, 8, 9)))
            it.isAnswered = true
            Log.d("QuestionRepository", "Flatten A List Of Integers: ${expected.toList()} and result is ${result.toList()}")
            it.isCorrect = result.contentEquals(expected)
            result.contentEquals(expected)
        })
    }

}