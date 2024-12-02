pub fn palindrome_check(input: &str) -> bool {
    let cleaned: String = input.chars().filter(|c| c.is_alphanumeric()).collect();
    let cleaned_lower = cleaned.to_lowercase();
    cleaned_lower == cleaned_lower.chars().rev().collect::<String>()
}

#[cfg(test)]
mod palindrom_test {
    use super::palindrome_check;

    #[test]
    fn test_palindrom() {
        assert!(palindrome_check("racecar"))
    }
}
