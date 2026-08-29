use std::collections::HashSet;

fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut hashset: HashSet<String> = HashSet::with_capacity(words.len());
    for w1 in words.iter() {
        for w2 in words.iter() {
            if w1.len() >= w2.len() {
                continue;
            }

            if w2.contains(w1) {
                hashset.insert(w1.to_owned());
            }
        }
    }

    hashset.into_iter().collect::<Vec<_>>()
}

pub fn main() {
    let words = ["mass", "as", "hero", "superhero"]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let res = string_matching(words);
    println!("{:?}", res);
}
