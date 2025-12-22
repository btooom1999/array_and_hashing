use std::collections::HashMap;

fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    if strings.is_empty() {
        return vec![];
    };

    let mut res: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    for str in &strings {
        if str.len() == 1 {
            res.entry(vec![0]).or_default().push(str.to_owned());
            continue;
        }
        let nums = str.as_bytes();
        let mut distances = vec![];
        for i in 0..(nums.len() - 1) {
            let mut d = (nums[i + 1] as i32 - nums[i] as i32);
            if d < 0 {
                d = d.abs() + (b'z' - nums[i]) as i32;
            }

            distances.push(d as u8);
        }

        res.entry(distances).or_default().push(str.to_owned());
    }

    res.into_values().collect::<Vec<Vec<_>>>()
}

pub fn main() {
    let strings = ["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let res = group_strings(strings);
    println!("{:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input: Vec<String> = vec![];
        let output = group_strings(input);
        assert!(output.is_empty());
    }

    #[test]
    fn test_single_element() {
        let input = vec!["a".to_string()];
        let output = group_strings(input);
        assert_eq!(output, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_multiple_single_char() {
        let input = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut output = group_strings(input);
        output.sort();
        assert_eq!(
            output,
            vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]]
        );
    }

    #[test]
    fn test_different_patterns() {
        let input = vec!["ab".to_string(), "ac".to_string()];
        let mut output = group_strings(input);
        output.sort();
        // "ab" has [1] distance,
        // "ac" has [2] distance
        assert_eq!(output, vec![vec!["ab".to_string()], vec!["ac".to_string()]]);
    }

    #[test]
    fn test_same_pattern() {
        let input = vec!["abc".to_string(), "bcd".to_string()];
        let mut output = group_strings(input);
        output.sort();
        // all of them have [1,1] distance
        assert_eq!(output, vec![vec!["abc".to_string(), "bcd".to_string()]]);
    }

    #[test]
    fn test_mixed() {
        let input = vec![
            "a".to_string(),
            "b".to_string(),
            "abc".to_string(),
            "bcd".to_string(),
            "ace".to_string(),
        ];
        let mut output = group_strings(input);
        output.sort();
        // group A: [1]: ["a","b"]
        // group B: [1,1]: ["abc","bcd"]
        // group C: [2,2]: ["ace"]
        assert_eq!(
            output,
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["abc".to_string(), "bcd".to_string()],
                vec!["ace".to_string()],
            ]
        );
    }
}
