fn reverse_string_with_iterators(input: &str) -> String {
    input.chars().rev().collect()
}

fn reverse_string_with_loop(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars().rev() {
        result.push(c);
    }
    result
}

fn reverse_string_with_recursion(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let (first, rest) = input.split_at(1);
    reverse_string_with_recursion(rest) + first
}

#[cfg(test)]
mod strings_tests {
    use super::*;

    #[test]
    fn test_reverse_string_with_iterators() {
        assert_eq!(reverse_string_with_iterators("hello"), "olleh");
    }

    #[test]
    fn test_reverse_string_with_loop() {
        assert_eq!(reverse_string_with_loop("hello"), "olleh");
    }

    #[test]
    fn test_reverse_string_with_recursion() {
        assert_eq!(reverse_string_with_recursion("hello"), "olleh");
    }
}
