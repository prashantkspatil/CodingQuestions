package pp.example.codingqna

object JniCall {
    init {
        System.loadLibrary("coding_qna")
    }

    external fun reverse_a_string(str: String): String
    external fun check_for_palindrome(str: String): Boolean
    external fun find_the_maximum_element_in_an_array(arr: IntArray): Int
    external fun fizz_buzz_problem(n: Int): String
    external fun count_vowels_in_a_string(str: String): Int
    external fun remove_duplicates_from_the_list(arr: IntArray): IntArray
}