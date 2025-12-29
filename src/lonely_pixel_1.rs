use std::collections::HashMap;

fn count_lonely_pixels(picture: Vec<Vec<char>>) -> i32 {
    let mut blacks = Vec::new();
    let mut hashmap = HashMap::<String, i32>::new();
    for (i, vec) in picture.iter().enumerate() {
        for (j, p) in vec.iter().enumerate() {
            if *p == 'B' {
                blacks.push((i, j));
                *hashmap.entry(format!("r{}", j)).or_default() += 1;
                *hashmap.entry(format!("c{}", i)).or_default() += 1;
            }
        }
    }

    println!("{:#?}", hashmap);

    blacks.into_iter().fold(0, |count, (i, j)| {
        if let (Some(val1), Some(val2)) = (
            hashmap.get(&format!("r{}", j)),
            hashmap.get(&format!("c{}", i)),
        ) && *val1 == 1
            && *val2 == 1
        {
            count + 1
        } else {
            count
        }
    })
}

pub fn main() {
    let picture: Vec<Vec<char>> = vec![
        vec!['W', 'W', 'B'],
        vec!['W', 'B', 'W'],
        vec!['B', 'W', 'W'],
    ];
    // let picture = vec![vec!['B', 'W'], vec!['B', 'W'], vec!['B', 'W']];

    println!("{}", count_lonely_pixels(picture))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let picture = vec![
            vec!['W', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'W'],
        ];
        assert_eq!(count_lonely_pixels(picture), 3);
    }

    #[test]
    fn test_case2() {
        let picture = vec![
            vec!['B', 'B', 'W'],
            vec!['W', 'B', 'W'],
            vec!['W', 'W', 'B'],
        ];
        assert_eq!(count_lonely_pixels(picture), 1);
    }

    #[test]
    fn test_case3() {
        let picture = vec![vec!['B', 'B'], vec!['B', 'B']];
        assert_eq!(count_lonely_pixels(picture), 0);
    }

    #[test]
    fn test_case4() {
        let picture = vec![vec!['W', 'W'], vec!['W', 'W']];
        assert_eq!(count_lonely_pixels(picture), 0);
    }

    #[test]
    fn test_case5() {
        let picture = vec![
            vec!['B', 'B', 'B'],
            vec!['W', 'W', 'W'],
            vec!['W', 'W', 'W'],
        ];
        assert_eq!(count_lonely_pixels(picture), 0);
    }

    #[test]
    fn test_case6() {
        let picture = vec![vec!['B', 'W'], vec!['B', 'W'], vec!['B', 'W']];
        assert_eq!(count_lonely_pixels(picture), 0);
    }

    #[test]
    fn test_case7() {
        let picture = vec![
            vec!['B', 'W', 'W'],
            vec!['W', 'W', 'W'],
            vec!['W', 'W', 'B'],
        ];
        assert_eq!(count_lonely_pixels(picture), 2);
    }

    #[test]
    fn test_case8() {
        let picture = vec![
            vec!['W', 'W', 'W'],
            vec!['W', 'B', 'W'],
            vec!['W', 'W', 'W'],
        ];
        assert_eq!(count_lonely_pixels(picture), 1);
    }

    #[test]
    fn test_case9() {
        let picture = vec![
            vec!['B', 'W', 'W', 'W'],
            vec!['W', 'B', 'W', 'W'],
            vec!['W', 'W', 'B', 'W'],
            vec!['W', 'W', 'W', 'B'],
        ];
        assert_eq!(count_lonely_pixels(picture), 4);
    }

    #[test]
    fn test_case10() {
        let picture = vec![
            vec!['B', 'W', 'W', 'W', 'W'],
            vec!['W', 'B', 'W', 'W', 'W'],
            vec!['W', 'W', 'B', 'W', 'W'],
            vec!['W', 'W', 'W', 'B', 'W'],
            vec!['W', 'W', 'W', 'W', 'B'],
        ];
        assert_eq!(count_lonely_pixels(picture), 5);
    }
}
