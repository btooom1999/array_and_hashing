use std::collections::HashSet;

fn count_elements(arr: Vec<i32>) -> i32 {
    let mut hashset = HashSet::<i32>::with_capacity(arr.len());
    for n in &arr {
        hashset.insert(*n);
    }

    let mut res = 0;
    for n in &arr {
        if hashset.contains(&(n + 1)) {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let arr = vec![1, 2, 3];
    println!("{}", count_elements(arr));
}

#[cfg(test)]
mod tests {
    use super::count_elements;

    #[test]
    fn test_example1() {
        let arr = vec![1, 2, 3];
        // 1 có 2, 2 có 3 → count = 2
        assert_eq!(count_elements(arr), 2);
    }

    #[test]
    fn test_example2() {
        let arr = vec![1, 1, 3, 3, 5, 5, 7, 7];
        // không có phần tử nào có x+1 trong arr → count = 0
        assert_eq!(count_elements(arr), 0);
    }

    #[test]
    fn test_single_element() {
        let arr = vec![10];
        // không có 11 → count = 0
        assert_eq!(count_elements(arr), 0);
    }

    #[test]
    fn test_all_consecutive() {
        let arr = vec![1, 2, 3, 4, 5];
        // 1→2, 2→3, 3→4, 4→5 → count = 4
        assert_eq!(count_elements(arr), 4);
    }

    #[test]
    fn test_duplicates() {
        let arr = vec![1, 1, 2, 2];
        // mỗi 1 có 2, tổng 2; mỗi 2 không có 3 → count = 2
        assert_eq!(count_elements(arr), 2);
    }

    #[test]
    fn test_large_values() {
        let arr = vec![999, 1000];
        // 999 có 1000 → count = 1
        assert_eq!(count_elements(arr), 1);
    }

    #[test]
    fn test_zero_in_array() {
        let arr = vec![0, 1];
        // 0 có 1 → count = 1
        assert_eq!(count_elements(arr), 1);
    }

    #[test]
    fn test_non_consecutive_numbers() {
        let arr = vec![2, 4, 6, 8];
        // không có x+1 nào → count = 0
        assert_eq!(count_elements(arr), 0);
    }

    #[test]
    fn test_mixed_case() {
        let arr = vec![1, 2, 2, 3];
        // 1→2, 2→3 (2 lần), 3 không có 4 → count = 3
        assert_eq!(count_elements(arr), 3);
    }

    #[test]
    fn test_large_array() {
        let arr: Vec<i32> = (0..1000).collect();
        // tất cả từ 0..998 có x+1 → count = 999
        assert_eq!(count_elements(arr), 999);
    }
}

