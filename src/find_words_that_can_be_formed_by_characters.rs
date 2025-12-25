use std::collections::HashMap;

fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut hashmap = HashMap::<char, i32>::with_capacity(chars.len());
    for c in chars.chars() {
        *hashmap.entry(c).or_default() += 1;
    }

    let mut count = 0;
    for word in &words {
        let mut temp_hashmap = HashMap::<char, i32>::with_capacity(word.len());
        let mut flag = true;
        for c in word.chars() {
            let val2 = temp_hashmap.get(&c).unwrap_or(&0) + 1;
            let val1 = hashmap.get(&c).unwrap_or(&0);

            temp_hashmap.insert(c, val2);

            if val2 > *val1 {
                flag = false;
                break;
            }
        }

        if flag {
            count += word.len();
        }
    }

    count as i32
}

pub fn main() {
    let words = vec![
        "cat".to_string(),
        "bt".to_string(),
        "hat".to_string(),
        "tree".to_string(),
    ];
    let chars = "atach".to_string();
    println!("{}", count_characters(words, chars));
}
