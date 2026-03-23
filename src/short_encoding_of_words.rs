use std::collections::HashSet;

fn minimum_length_encoding(words: Vec<String>) -> i32 {
    let hashset = words.into_iter().collect::<HashSet<_>>();
    let mut temp = Vec::new();

    for str in hashset {
        temp.push(str.chars().rev().collect::<String>());
    }

    temp.sort();

    let mut count = 0;
    for (i, w) in temp.iter().enumerate() {
        if i == temp.len() - 1 || !temp[i+1].starts_with(&temp[i]) {
            count += temp[i].len() + 1;
        }
    }

    count as _
}

pub fn main() {
    let words = ["time", "me", "bell"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", minimum_length_encoding(words));
}
