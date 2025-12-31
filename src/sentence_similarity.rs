use std::collections::HashSet;

fn are_sentences_similar(
    words1: Vec<String>,
    words2: Vec<String>,
    pairs: Vec<Vec<String>>,
) -> bool {
    if words1.len() != words2.len() {
        return false;
    }

    let mut hashset = HashSet::<Vec<String>>::with_capacity(pairs.len());

    for pair in pairs {
        hashset.insert(pair);
    }

    for (a, b) in words1.into_iter().zip(words2.into_iter()) {
        if a != b
            && !hashset.contains(&vec![b.to_string(), a.to_string()])
            && !hashset.contains(&vec![a.to_string(), b.to_string()])
        {
            return false;
        }
    }

    true
}

pub fn main() {
    let words1 = vec![
        "great".to_string(),
        "acting".to_string(),
        "skills".to_string(),
    ];
    let words2 = vec![
        "fine".to_string(),
        "drama".to_string(),
        "talent".to_string(),
    ];
    let pairs = vec![
        vec!["great".to_string(), "fine".to_string()],
        vec!["acting".to_string(), "drama".to_string()],
        vec!["skills".to_string(), "talent".to_string()],
    ];
    println!("{}", are_sentences_similar(words1, words2, pairs));
}
