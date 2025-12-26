use std::collections::HashMap;

fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, Vec<i32>>::with_capacity(b.len());
    for (i, num) in b.into_iter().enumerate() {
        hashmap.entry(num).or_default().push(i as i32);
    }

    let mut res = Vec::new();
    for num in a {
        let vec = hashmap.entry(num).or_default();
        if let Some(val) = vec.first() {
            res.push(*val);
            vec.remove(0);
        } else {
            res.push(-1);
        }
    }

    res
}

pub fn main() {
    let a = vec![12, 28, 46, 32, 50];
    let b = vec![50, 12, 32, 46, 28];
    println!("{:?}", anagram_mappings(a, b));
}

#[cfg(test)]
mod tests {
    use super::anagram_mappings;

    #[test]
    fn test_basic_case() {
        let a = vec![12, 28, 46, 32, 50];
        let b = vec![50, 12, 32, 46, 28];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![1, 4, 3, 2, 0]);
    }

    #[test]
    fn test_with_duplicates() {
        let a = vec![12, 12, 28];
        let b = vec![28, 12, 12];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![1, 2, 0]);
    }

    #[test]
    fn test_not_found_case() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![-1, 0, 1]);
    }

    #[test]
    fn test_single_element() {
        let a = vec![5];
        let b = vec![5];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn test_empty() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_reverse_order() {
        let a = vec![1, 2, 3];
        let b = vec![3, 2, 1];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![2, 1, 0]);
    }

    #[test]
    fn test_all_same_elements() {
        let a = vec![7, 7, 7];
        let b = vec![7, 7, 7];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![0, 1, 2]);
    }

    #[test]
    fn test_large_numbers() {
        let a = vec![100000, 99999];
        let b = vec![99999, 100000];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![1, 0]);
    }

    #[test]
    fn test_not_found_multiple() {
        let a = vec![1, 2, 3];
        let b = vec![2, 4, 5];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![-1, 0, -1]);
    }

    #[test]
    fn test_single_element_not_found() {
        let a = vec![42];
        let b = vec![24];
        let res = anagram_mappings(a, b);
        assert_eq!(res, vec![-1]);
    }
}
