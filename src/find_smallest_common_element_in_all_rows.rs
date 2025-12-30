use std::collections::HashMap;

fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    let mut min = i32::MAX;
    for vec in &mat {
        for num in vec {
            let val = hashmap.entry(*num).or_default();
            *val += 1;

            if *val == mat.len() as i32 {
                min = std::cmp::min(min, *num);
            }
        }
    }

    if min == i32::MAX {
        return -1;
    }

    min
}

pub fn main() {
    let mat = vec![vec![1, 2, 3, 4, 5], vec![2, 4, 5, 8, 10], vec![3, 5, 7, 9, 11], vec![1, 3, 5, 7, 9]];
    println!("{}", smallest_common_element(mat));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mat = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 4, 5, 8, 10],
            vec![3, 5, 7, 9, 11],
            vec![1, 3, 5, 7, 9],
        ];
        assert_eq!(smallest_common_element(mat), 5);
    }

    #[test]
    fn test_case2() {
        let mat = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![3, 4, 5],
        ];
        assert_eq!(smallest_common_element(mat), 3);
    }

    #[test]
    fn test_case3() {
        let mat = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(smallest_common_element(mat), -1);
    }

    #[test]
    fn test_case4() {
        let mat = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 5],
            vec![1, 4, 6, 7],
        ];
        assert_eq!(smallest_common_element(mat), 1);
    }

    #[test]
    fn test_case5() {
        let mat = vec![
            vec![2, 3, 5],
            vec![1, 2, 5],
            vec![2, 4, 5],
        ];
        assert_eq!(smallest_common_element(mat), 2);
    }

    #[test]
    fn test_case6() {
        let mat = vec![
            vec![10, 20, 30],
            vec![20, 30, 40],
            vec![30, 40, 50],
        ];
        assert_eq!(smallest_common_element(mat), 30);
    }

    #[test]
    fn test_case7() {
        let mat = vec![
            vec![1, 5, 9],
            vec![2, 5, 10],
            vec![3, 5, 11],
        ];
        assert_eq!(smallest_common_element(mat), 5);
    }

    #[test]
    fn test_case8() {
        let mat = vec![
            vec![1, 2, 3, 4],
            vec![2, 3, 4, 5],
            vec![3, 4, 5, 6],
            vec![4, 5, 6, 7],
        ];
        assert_eq!(smallest_common_element(mat), 4);
    }

    #[test]
    fn test_case9() {
        let mat = vec![
            vec![1, 2],
            vec![1, 2],
            vec![1, 2],
        ];
        assert_eq!(smallest_common_element(mat), 1);
    }

    #[test]
    fn test_case10() {
        let mat = vec![
            vec![100, 200, 300],
            vec![200, 300, 400],
            vec![300, 400, 500],
            vec![300, 600, 700],
        ];
        assert_eq!(smallest_common_element(mat), 300);
    }
}

