fn make_equal(words: Vec<String>) -> bool {
    let mut hashmap = [0i32; 26];
    for word in &words {
        for num in word.as_bytes() {
            hashmap[(num - b'a') as usize] += 1;
        }
    }

    let hashmap = hashmap.into_iter().filter(|v| *v > 0).collect::<Vec<_>>();

    hashmap.iter().all(|v| *v % words.len() as i32 == 0)
}

pub fn main() {
    let words = vec!["abc".to_string(), "cba".to_string()];
    // let words = vec![
    //     "caaaaa".to_string(),
    //     "aaaaaaaaa".to_string(),
    //     "a".to_string(),
    //     "bbb".to_string(),
    //     "bbbbbbbbb".to_string(),
    //     "bbb".to_string(),
    //     "cc".to_string(),
    //     "cccccccccccc".to_string(),
    //     "ccccccc".to_string(),
    //     "ccccccc".to_string(),
    //     "cc".to_string(),
    //     "cccc".to_string(),
    //     "c".to_string(),
    //     "cccccccc".to_string(),
    //     "c".to_string(),
    // ];
    println!("{}", make_equal(words));
}
