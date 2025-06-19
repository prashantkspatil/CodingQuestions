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

#[cfg(test)]
mod test {
    use super::*;

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
}