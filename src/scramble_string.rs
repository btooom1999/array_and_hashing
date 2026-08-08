use std::collections::HashMap;

fn dfs<'a>(
    s1: &'a [u8],
    s2: &'a [u8],
    memo: &mut HashMap<(&'a [u8], &'a [u8]), bool>,
) -> bool {
    if let Some(&value) = memo.get(&(s1, s2)) {
        return value;
    }

    let mut res = s1 == s2;
    if s1.len() != 1 {
        for i in 0..s1.len()-1 {
            if res { break; }
            let (left, right) = (&s1[..=i], &s1[i+1..]);
            let a = dfs(left, &s2[..left.len()], memo);
            let b = dfs(right, &s2[left.len()..], memo);
            res = a && b;
            if !res {
                let a = dfs(right, &s2[..right.len()], memo);
                let b = dfs(left, &s2[right.len()..], memo);
                res = a && b;
            }
        }
    }

    memo.insert((s1, s2), res);
    res
}

fn is_scramble(s1: String, s2: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let n = s1.len();
    let mut memo = HashMap::new();
    for i in (0..n).rev() {
        for j in i..n {
            dfs(&s1[i..=j], &s2[i..=j], &mut memo);
        }
    }

    *memo.get(&(s1, s2)).unwrap_or(&false)
}

pub fn main() {
    let s1 = "agg".to_string();
    let s2 = "gag".to_string();
    println!("{}", is_scramble(s1, s2));
}
