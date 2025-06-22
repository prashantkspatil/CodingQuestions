
use crate::solutions::*;

#[test]
fn test_reverse_a_string() {
    let rev_string = String::from("Hello World");
    assert_eq!("dlroW olleH", reverse_a_string(&rev_string))
}

#[test]
fn test_check_for_palindrome() {
    let str = "madam";
    assert!(check_for_palindrome(str));
}

#[test]
fn test_max_element() {
    let mut input = vec!(1, 2, 3, 4, 5);
    let result = find_the_maximum_element_in_an_array(&mut input);
    assert_eq!(result, 5)
}

#[test]
fn test_fizz_buzz() {
    let result = fizz_buzz(15 as u32);
    assert_eq!(result, "1\n2\nfizz\n4\nbuzz\nfizz\n7\n8\nfizz\nbuzz\n11\nfizz\n13\n14\nfizzbuzz\n");
}

#[test]
fn test_vowel_count() {
    let result = count_vowels_in_a_string("Hello World");
    assert_eq!(result, 3);
}

#[test]
fn test_remove_duplicates_from_array() {
    let mut result = remove_duplicates_from_the_list(&[1, 2, 2, 3, 4, 4, 5]);
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_find_first_single_char() {
    let result = find_first_non_repeated_char_in_string("swiss");
    assert_eq!(result, 'w');
}

#[test]
fn test_factorial_using_recursion() {
    let result = factorial_using_recursion(5);
    assert_eq!(result, 120);
}

#[test]
fn test_find_second_largest_number() {
    let result = find_the_second_largest(&[1, 5, 3, 6, 8, 12, 5]);
    assert_eq!(result, 8);
}

#[test]
fn test_sum_of_digits_in_number() {
    let result = sum_of_digits_in_number(25);
    assert_eq!(result, 7);
}

#[test]
fn test_find_the_missing_number_in_array() {
    let result = find_the_missing_number_in_array(&[1, 2, 3, 4, 6]);
    assert_eq!(result, 5);
}

#[test]
fn test_check_if_two_strings_are_anagram() {
    let result = check_if_two_strings_are_anagram("silent", "listen");
    assert!(result);
}

#[test]
fn test_flatten_a_list_of_integers() {
    let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let result = flatten_a_list_of_integers(&arr);
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}