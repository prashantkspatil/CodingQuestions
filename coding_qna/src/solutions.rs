use std::collections::{HashMap, HashSet};

pub fn reverse_a_string(str: &str) -> String {
    let rev_str:String = str.chars().rev().collect();
    return format!("{}", rev_str);
}

pub fn check_for_palindrome(str: &str) -> bool {
    let rev_str: String = str.chars().rev().collect();
    return str == &rev_str;
}

pub fn find_the_maximum_element_in_an_array(input: &[i32]) -> i32 {
    return input.iter().copied().max().unwrap();
}

pub fn fizz_buzz(input: u32) -> String {
    let mut result = String::new();
    for n in 1..input+1 {
        if n % 3 == 0 && n % 5 == 0 {
            result = result + "fizzbuzz\n";
        } else if n % 3 == 0 {
            result += "fizz\n";
        } else if n % 5 == 0 {
            result += "buzz\n";
        } else {
            result += format!("{}\n", n).as_str();
        }
    }
    return result;
}

pub fn count_vowels_in_a_string(input: &str) -> i32 {
    let mut result = 0;
    let vowels = "aeiouAEIOU";
    for c in input.chars() {
        if vowels.contains(c) {
            result += 1;
        }
    }
    return result;
}

pub fn remove_duplicates_from_the_list(input: &[i32]) -> Vec<i32> {
    let mut result: HashSet<i32> = HashSet::new();
    for i in input {
        result.insert(*i);
    }
    
    return result.into_iter().collect();
}

pub fn find_first_non_repeated_char_in_string(input: &str) -> char {
    let mut char_couts:HashMap<char, i32> = HashMap::new();
    input.chars().for_each(|x| {
        let count = char_couts.entry(x).or_insert(0);
        *count += 1;
    });
    let ret = input.chars().find(|c| char_couts[c] == 1).unwrap();
    ret
}

pub fn factorial_using_recursion(num: i32) -> i32 {
    if num <= 1 {
        return 1;
    }
    return factorial_using_recursion(num - 1) * num;
}

pub fn find_the_second_largest(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    let mut second_largest = arr[0];
    for i in arr.iter() {
        if *i > max {
            second_largest = max;
            max = *i;
        }
    }
    return second_largest;
}

pub fn sum_of_digits_in_number(num:i32) -> i32 {
    let mut sum = 0;
    let mut num = num;
    while num > 0 {
        sum += num % 10;
        num = num / 10;
    }
    return sum;
}