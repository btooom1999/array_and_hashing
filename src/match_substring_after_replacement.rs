use std::collections::{HashMap, HashSet};

fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
    let mut hashmap = HashMap::<_, HashSet<_>>::new();
    for map in mappings {
        hashmap.entry(map[0] as u8).or_default().insert(map[1] as u8);
    }

    let s = s.as_bytes();
    let sub = sub.as_bytes();
    let (m, n) = (s.len(), sub.len());
    for i in 0..=m-n {
        let mut found = true;
        for j in 0..n {
            if s[i+j] != sub[j] && hashmap.get(&sub[j]).is_none_or(|v| !v.contains(&s[i+j])) {
                found = false;
                break;
            }
        }

        if found { return true; }
    }

    false
}

pub fn main() {
    let s = "fool3e7bar".to_string();
    let sub = "leet".to_string();
    let mappings = [["e","3"],["e","3"],["e","1"],["t","7"],["t","8"]]
        .into_iter()
        .map(|v| v
            .into_iter()
            .map(|v| v
                .chars()
                .next()
                .unwrap())
            .collect())
        .collect();
    println!("{}", match_replacement(s, sub, mappings));
}
