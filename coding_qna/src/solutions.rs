use std::collections::HashSet;

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
