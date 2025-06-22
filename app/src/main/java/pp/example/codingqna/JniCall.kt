package pp.example.codingqna

object JniCall {
    init {
        System.loadLibrary("coding_qna")
    }

/* 01 */    external fun reverse_a_string(str: String): String
/* 02 */    external fun check_for_palindrome(str: String): Boolean
/* 03 */    external fun find_the_maximum_element_in_an_array(arr: IntArray): Int
/* 04 */    external fun fizz_buzz_problem(n: Int): String
/* 05 */    external fun count_vowels_in_a_string(str: String): Int
/* 06 */    external fun remove_duplicates_from_the_list(arr: IntArray): IntArray
/* 07 */    external fun findTheFirstNonRepeatedCharInString(str: String): Char
/* 08 */    external fun factorialUsingRecursion(num: Int): Int
/* 09 */    external fun findTheSecondLargestNumber(arr: IntArray): Int
/* 10 */    external fun sumOfDigitsInNumber(num: Int): Int
/* 11 */    external fun findTheMissingNumberInArray(arr: IntArray): Int
/* 12 */    external fun checkIfTwoStringsAreAnagram(str1: String, str2: String): Boolean
/* 13 */    external fun flattenAListOfIntegers(arr: Array<IntArray>): IntArray
}