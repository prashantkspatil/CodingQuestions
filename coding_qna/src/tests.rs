
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