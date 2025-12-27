use std::collections::HashMap;

fn can_permute_palindrome(s: String) -> bool {
    let mut hashmap = HashMap::<char, i32>::with_capacity(s.len());
    for c in s.chars() {
        *hashmap.entry(c).or_default() += 1;
    }

    let mut count = 0;
    for v in hashmap.values() {
        if *v % 2 != 0 {
            count += 1;
        }

        if count >= 2 {
            return false;
        }
    }

    true
}

pub fn main() {
    let s = "aab".to_string();
    println!("{}", can_permute_palindrome(s));
}

#[cfg(test)]
mod tests {
    use super::can_permute_palindrome;

    #[test]
    fn test_single_char() {
        let s = "a".to_string();
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_two_same_chars() {
        let s = "aa".to_string();
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_two_different_chars() {
        let s = "ab".to_string();
        assert!(!can_permute_palindrome(s));
    }

    #[test]
    fn test_aab() {
        let s = "aab".to_string();
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_carerac() {
        let s = "carerac".to_string();
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_all_unique_chars() {
        let s = "abc".to_string();
        assert!(!can_permute_palindrome(s));
    }

    #[test]
    fn test_even_length_all_even_counts() {
        let s = "aabbcc".to_string();
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_odd_length_one_odd_count() {
        let s = "aabbccd".to_string();
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_large_string_all_same_char() {
        let s = "z".repeat(5000);
        assert!(can_permute_palindrome(s));
    }

    #[test]
    fn test_large_string_half_even_half_odd() {
        let mut s = "x".repeat(4999);
        s.push('y');
        assert!(!can_permute_palindrome(s));
    }
}
