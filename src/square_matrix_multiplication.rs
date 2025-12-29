fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = b[0].len();
    let mut res = vec![vec![0; m]; n];
    for (i, vec) in a.iter().enumerate() {
        for (j, num) in vec.iter().enumerate() {
            for x in 0..m {
                res[i][x] += num * b[j][x];
            }
        }
    }

    res
}

pub fn main() {
    let a = vec![
        vec![1], 
        vec![2]
    ];
    let b = vec![
        vec![3, 4, 5], 
    ];

    println!("{:?}", multiply(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        let a = vec![vec![1, 0, 0], vec![-1, 0, 3]];
        let b = vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        let expected = vec![vec![7, 0, 0], vec![-7, 0, 3]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case2() {
        let a = vec![vec![1, 2], vec![0, 3]];
        let b = vec![vec![4, 0], vec![0, 5]];
        let expected = vec![vec![4, 10], vec![0, 15]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case3() {
        let a = vec![vec![0, 0], vec![0, 0]];
        let b = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case4() {
        let a = vec![vec![1], vec![2]];
        let b = vec![vec![3, 4, 5]];
        let expected = vec![vec![3, 4, 5], vec![6, 8, 10]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case5() {
        let a = vec![vec![2, 0], vec![0, 2]];
        let b = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![vec![2, 2], vec![2, 2]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case6() {
        let a = vec![vec![1, 2, 3]];
        let b = vec![vec![4], vec![5], vec![6]];
        let expected = vec![vec![32]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case7() {
        let a = vec![vec![1, 0], vec![0, 1]];
        let b = vec![vec![7, 8], vec![9, 10]];
        let expected = vec![vec![7, 8], vec![9, 10]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case8() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![0, 1], vec![1, 0]];
        let expected = vec![vec![2, 1], vec![4, 3]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case9() {
        let a = vec![vec![1, -1], vec![2, -2]];
        let b = vec![vec![3, 4], vec![5, 6]];
        let expected = vec![vec![-2, -2], vec![-4, -4]];
        assert_eq!(multiply(a, b), expected);
    }
    #[test]
    fn test_case10() {
        let a = vec![vec![2, 3], vec![4, 5]];
        let b = vec![vec![6, 7], vec![8, 9]];
        let expected = vec![vec![36, 41], vec![64, 73]];
        assert_eq!(multiply(a, b), expected);
    }
}
