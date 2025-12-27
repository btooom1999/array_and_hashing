use std::collections::HashMap;

fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(nums.len());
    for num in nums {
        *hashmap.entry(num).or_insert(0) += 1;
    }

    let mut max = -1;
    for (k, v) in hashmap.into_iter() {
        if v == 1 {
            max = std::cmp::max(max, k);
        }
    }

    max
}

pub fn main() {
    let nums = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
    println!("{}", largest_unique_number(nums));
}

#[cfg(test)]
mod tests {
    use super::largest_unique_number;

    #[test]
    fn test_basic_case() {
        let nums = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
        assert_eq!(largest_unique_number(nums), 8);
    }

    #[test]
    fn test_all_unique() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(largest_unique_number(nums), 5);
    }

    #[test]
    fn test_all_duplicates() {
        let nums = vec![2, 2, 3, 3, 4, 4];
        assert_eq!(largest_unique_number(nums), -1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        assert_eq!(largest_unique_number(nums), 42);
    }

    #[test]
    fn test_multiple_uniques() {
        let nums = vec![10, 20, 20, 30, 40, 40];
        assert_eq!(largest_unique_number(nums), 30);
    }

    #[test]
    fn test_zero_in_array() {
        assert_eq!(largest_unique_number(nums), 0);
    }

    #[test]
    fn test_large_values() {
        let nums = vec![1000, 999, 1000];
        assert_eq!(largest_unique_number(nums), 999);
    }

    #[test]
    fn test_unique_at_end() {
        let nums = vec![7, 7, 8, 8, 9];
        assert_eq!(largest_unique_number(nums), 9);
    }

    #[test]
    fn test_unique_at_start() {
        let nums = vec![11, 12, 12, 13, 13];
        assert_eq!(largest_unique_number(nums), 11);
    }

    #[test]
    fn test_large_array() {
        let mut nums = vec![500; 999];
        nums.push(600);
        assert_eq!(largest_unique_number(nums), 600);
    }
}

