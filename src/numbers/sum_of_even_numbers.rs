fn sum_of_even_number(nums: &[u32]) -> u32 {
    let mut result = 0;
    for num in nums {
        if num % 2 == 0 {
            result += num;
        }
    }
    result
}

#[cfg(test)]
mod sum_of_even_number_test {
    use super::sum_of_even_number;

    #[test]
    fn test_sum_of_even_number() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_of_even_number(&arr), 6)
    }
}
