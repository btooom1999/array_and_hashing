use std::collections::HashMap;

fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let order = order.chars().enumerate().map(|v| (v.1, v.0)).collect::<HashMap<char, usize>>();
    let words = words
        .into_iter()
        .map(|w| w
            .chars()
            .map(|c| *order.get(&c).unwrap())
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let mut sorted_words = words.clone();
    sorted_words.sort();

    words == sorted_words
}

pub fn main() {
    let words = ["hello","leetcode"].into_iter().map(String::from).collect();
    let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
    println!("{}", is_alien_sorted(words, order))
}
