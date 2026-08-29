fn make_palindrome(s: String) -> bool {
    let mut errors = 0;
    let mut l = 0;
    let mut r = s.len();
    let s = s.as_bytes();
    while l < r {
        r -= 1;
        if s[l] != s[r] {
            errors += 1;
        }
        if errors > 2 {
            return false;
        }
        l += 1;
    }

    true
}

pub fn main() {
    let s = "abcdef".to_string();
    println!("{}", make_palindrome(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len40_palindrome() {
        let mut s = String::new();
        for i in 0..20 {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        for i in (0..20).rev() {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        assert!(make_palindrome(s));
    }

    #[test]
    fn test_len60_one_replace() {
        let mut s = String::new();
        for i in 0..30 {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        for i in (0..30).rev() {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars[10] = 'z';
        let s: String = chars.into_iter().collect();
        assert!(make_palindrome(s));
    }

    #[test]
    fn test_len80_two_replace() {
        let mut s = String::new();
        for i in 0..40 {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        for i in (0..40).rev() {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars[20] = 'x';
        chars[60] = 'y';
        let s: String = chars.into_iter().collect();
        assert!(make_palindrome(s));
    }

    #[test]
    fn test_len100_three_replace() {
        let mut s = String::new();
        for i in 0..50 {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        for i in (0..50).rev() {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars[5] = 'm';
        chars[25] = 'n';
        chars[75] = 'o';
        let s: String = chars.into_iter().collect();
        assert!(!make_palindrome(s));
    }

    #[test]
    fn test_len50_random_false() {
        let mut s = String::new();
        for i in 0..50 {
            s.push(('a' as u8 + (i % 26)) as char);
        }
        assert!(!make_palindrome(s));
    }

    #[test]
    fn test_len5_palindrome() {
        assert!(make_palindrome("abcba".to_string()));
    }

    #[test]
    fn test_len5_one_replace() {
        assert!(make_palindrome("abcca".to_string()));
    }

    #[test]
    fn test_len5_two_replace() {
        assert!(make_palindrome("abxda".to_string()));
    }

    #[test]
    fn test_len6_palindrome() {
        assert!(make_palindrome("abccba".to_string()));
    }

    #[test]
    fn test_len6_one_replace() {
        assert!(make_palindrome("abcyba".to_string()));
    }

    #[test]
    fn test_len6_two_replace() {
        assert!(make_palindrome("abxyba".to_string()));
    }

    #[test]
    fn test_len6_three_replace() {
        assert!(!make_palindrome("abcdef".to_string()));
    }

    #[test]
    fn test_len10_palindrome() {
        assert!(make_palindrome("abcdefghhgfedcba".to_string()));
    }

    #[test]
    fn test_len12_false() {
        assert!(!make_palindrome("abcdefghijkl".to_string()));
    }
}

