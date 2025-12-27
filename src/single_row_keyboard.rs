use std::collections::HashMap;

fn calculate_time(keyboard: String, word: String) -> i32 {
    let mut hashmap = HashMap::<char, i32>::with_capacity(keyboard.len());
    for (i, v) in keyboard.chars().enumerate() {
        hashmap.insert(v, i as i32);
    }

    let mut sum = 0;
    let mut time = None as Option<char>;
    for c in word.chars() {
        if let Some(val) = time {
            let (val1, val2) = (hashmap.get(&c).unwrap(), hashmap.get(&val).unwrap());
            sum += (val1 - val2).abs();
        } else {
            sum += hashmap.get(&c).unwrap();
        }

        time = Some(c);
    }

    sum
}

pub fn main() {
    let keyboard = "pqrstuvwxyzabcdefghijklmno".to_string();
    let word = "leetcode".to_string();
    println!("{}", calculate_time(keyboard, word));
}

#[cfg(test)]
mod tests {
    use super::calculate_time;

    #[test]
    fn test_example1() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "cba".to_string();
        assert_eq!(calculate_time(keyboard, word), 4);
    }

    #[test]
    fn test_example2() {
        let keyboard = "pqrstuvwxyzabcdefghijklmno".to_string();
        let word = "leetcode".to_string();
        assert_eq!(calculate_time(keyboard, word), 73);
    }

    #[test]
    fn test_single_char_word() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "z".to_string();
        assert_eq!(calculate_time(keyboard, word), 25);
    }

    #[test]
    fn test_word_all_same_char() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "aaaaa".to_string();
        assert_eq!(calculate_time(keyboard, word), 0);
    }

    #[test]
    fn test_word_full_alphabet() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(calculate_time(keyboard, word), 25);
    }

    #[test]
    fn test_reverse_alphabet_word() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "zyxwvutsrqponmlkjihgfedcba".to_string();
        assert_eq!(calculate_time(keyboard, word), 50);
    }

    #[test]
    fn test_keyboard_shuffled() {
        let keyboard = "zabcdefghijklmnopqrstuvwxy".to_string();
        let word = "az".to_string();
        assert_eq!(calculate_time(keyboard, word), 2);
    }

    #[test]
    fn test_long_word_repeated_pattern() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "abababab".to_string();
        assert_eq!(calculate_time(keyboard, word), 7);
    }

    #[test]
    fn test_word_with_far_jumps() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "azaz".to_string();
        assert_eq!(calculate_time(keyboard, word), 75);
    }

    #[test]
    fn test_large_word_length() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "a".repeat(10000);
        assert_eq!(calculate_time(keyboard, word), 0);
    }
}

