fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut hashmap = std::collections::HashMap::<_, i32>::new();
    for word in &words {
        *hashmap.entry(word.as_bytes()).or_default() += 1;
    }

    let mut counts = [0;26];
    let mut res = 0;
    for word in &words {
        let word = word.as_bytes();
        let val = hashmap.get_mut(&word).unwrap();
        if *val > 0 {
            *val -= 1;
            if word[0] == word[1] {
                counts[(word[0] - b'a') as usize] += 2;
            } else if let Some(value) = hashmap.get_mut(&[word[1], word[0]] as &[u8]) && *value > 0 {
                *value -= 1;
                res += 4;
            }
        }
    }

    let mut extra = 0;
    for count in counts {
        if count % 4 == 0 {
            res += count;
        } else if count > 0 {
            res += count - 2;
            extra = 2;
        }
    }

    res + extra
}

pub fn main() {
    let words = ["lc","cl","gg","lc"].into_iter().map(String::from).collect();
    println!("{}", longest_palindrome(words));
}
