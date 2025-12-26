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

#[cfg(test)]
mod tests {
    use super::are_sentences_similar;

    #[test]
    fn test_identical_sentences() {
        let w1 = vec![
            "great".to_string(),
            "acting".to_string(),
            "skills".to_string(),
        ];
        let w2 = vec![
            "great".to_string(),
            "acting".to_string(),
            "skills".to_string(),
        ];
        let pairs: Vec<Vec<String>> = vec![];
        assert!(are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_basic_similarity() {
        let w1 = vec![
            "great".to_string(),
            "acting".to_string(),
            "skills".to_string(),
        ];
        let w2 = vec![
            "fine".to_string(),
            "drama".to_string(),
            "talent".to_string(),
        ];
        let pairs = vec![
            vec!["great".to_string(), "fine".to_string()],
            vec!["acting".to_string(), "drama".to_string()],
            vec!["skills".to_string(), "talent".to_string()],
        ];
        assert!(are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_missing_pair() {
        let w1 = vec![
            "great".to_string(),
            "acting".to_string(),
            "skills".to_string(),
        ];
        let w2 = vec![
            "fine".to_string(),
            "drama".to_string(),
            "talent".to_string(),
        ];
        let pairs = vec![
            vec!["great".to_string(), "fine".to_string()],
            vec!["acting".to_string(), "drama".to_string()],
        ];
        assert!(!are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_symmetric_pair() {
        let w1 = vec!["great".to_string()];
        let w2 = vec!["fine".to_string()];
        let pairs = vec![vec!["fine".to_string(), "great".to_string()]];
        assert!(are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_not_transitive() {
        let w1 = vec!["great".to_string()];
        let w2 = vec!["good".to_string()];
        let pairs = vec![
            vec!["great".to_string(), "fine".to_string()],
            vec!["fine".to_string(), "good".to_string()],
        ];
        assert!(!are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_different_length_sentences() {
        let w1 = vec!["great".to_string(), "skills".to_string()];
        let w2 = vec!["great".to_string()];
        let pairs: Vec<Vec<String>> = vec![];
        assert!(!are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_self_similarity_without_pairs() {
        let w1 = vec!["hello".to_string()];
        let w2 = vec!["hello".to_string()];
        let pairs: Vec<Vec<String>> = vec![];
        assert!(are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_case_sensitive_words() {
        let w1 = vec!["Hello".to_string()];
        let w2 = vec!["hello".to_string()];
        let pairs: Vec<Vec<String>> = vec![vec!["Hello".to_string(), "hello".to_string()]];
        assert!(are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_multiple_pairs_same_word() {
        let w1 = vec!["fast".to_string()];
        let w2 = vec!["quick".to_string()];
        let pairs = vec![
            vec!["fast".to_string(), "rapid".to_string()],
            vec!["fast".to_string(), "quick".to_string()],
        ];
        assert!(are_sentences_similar(w1, w2, pairs));
    }

    #[test]
    fn test_no_pairs_and_different_words() {
        let w1 = vec!["dog".to_string()];
        let w2 = vec!["cat".to_string()];
        let pairs: Vec<Vec<String>> = vec![];
        assert!(!are_sentences_similar(w1, w2, pairs));
    }
}
