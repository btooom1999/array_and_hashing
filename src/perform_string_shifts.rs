fn string_shift(mut s: String, shifts: Vec<Vec<i32>>) -> String {
    for shift in &shifts {
        let (direction, amount) = (shift[0], shift[1] % s.len() as i32);
        let (left, right) = if direction == 0 {
            s.split_at(amount as usize)
        } else {
            s.split_at(s.len() - amount as usize)
        };

        s = format!("{}{}", right, left);
    }

    s.clone()
}

pub fn main() {
    let s = "abcdefg".to_string();
    let shift = vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]];
    println!("{}", string_shift(s, shift));
}

#[cfg(test)]
mod tests {
    use super::string_shift;

    #[test]
    fn test_example1() {
        let s = "abc".to_string();
        let shift = vec![vec![0, 1], vec![1, 2]];
        // Trái 1: "bca" → Phải 2: "cab"
        assert_eq!(string_shift(s, shift), "cab");
    }

    #[test]
    fn test_example2() {
        let s = "abcdefg".to_string();
        let shift = vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]];
        // Phải 1: "gabcdef" → Phải 1: "fgabcde" → Trái 2: "abcdefg" → Phải 3: "efgabcd"
        assert_eq!(string_shift(s, shift), "efgabcd");
    }

    #[test]
    fn test_single_left_shift() {
        let s = "abcd".to_string();
        let shift = vec![vec![0, 1]];
        // Trái 1 → "bcda"
        assert_eq!(string_shift(s, shift), "bcda");
    }

    #[test]
    fn test_single_right_shift() {
        let s = "abcd".to_string();
        let shift = vec![vec![1, 1]];
        // Phải 1 → "dabc"
        assert_eq!(string_shift(s, shift), "dabc");
    }

    #[test]
    fn test_multiple_left_shifts() {
        let s = "abcd".to_string();
        let shift = vec![vec![0, 1], vec![0, 2]];
        // Tổng trái 3 → "dabc"
        assert_eq!(string_shift(s, shift), "dabc");
    }

    #[test]
    fn test_multiple_right_shifts() {
        let s = "abcd".to_string();
        let shift = vec![vec![1, 1], vec![1, 2]];
        // Tổng phải 3 → "bcda"
        assert_eq!(string_shift(s, shift), "bcda");
    }

    #[test]
    fn test_equal_left_right_cancel() {
        let s = "abcd".to_string();
        let shift = vec![vec![0, 2], vec![1, 2]];
        // Trái 2 + Phải 2 = 0 → "abcd"
        assert_eq!(string_shift(s, shift), "abcd");
    }

    #[test]
    fn test_large_shift_amount_modulo() {
        let s = "abcd".to_string();
        let shift = vec![vec![0, 10]];
        // 10 % 4 = 2 → Trái 2 → "cdab"
        assert_eq!(string_shift(s, shift), "cdab");
    }

    #[test]
    fn test_full_rotation_no_change() {
        let s = "abcd".to_string();
        let shift = vec![vec![1, 4]];
        // Phải 4 = độ dài chuỗi → "abcd"
        assert_eq!(string_shift(s, shift), "abcd");
    }

    #[test]
    fn test_long_string_combined_shifts() {
        let s = "abcdefghijklmnopqrstuvwxyz".to_string();
        let shift = vec![vec![0, 5], vec![1, 3]];
        // Net = Trái (5 - 3) = Trái 2 → "cdefghijklmnopqrstuvwxyzab"
        assert_eq!(string_shift(s, shift), "cdefghijklmnopqrstuvwxyzab");
    }
}
