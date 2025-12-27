fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut hashmap = [i32::MAX; 26];
    for word in &words {
        let mut temp_hashmap = [0; 26];
        for b in word.as_bytes() {
            temp_hashmap[(b - b'a') as usize] += 1;
        }

        for i in 0..26 {
            hashmap[i] = std::cmp::min(hashmap[i], temp_hashmap[i]);
        }
    }

    let mut res = Vec::new();
    for (i, val) in hashmap.iter().enumerate() {
        if *val > 0 {
            let c = (i as u8 + b'a') as char;
            res.append(&mut vec![c.to_string(); *val as usize]);
        }
    }

    res
}

pub fn main() {
    let words = vec![
        "bella".to_string(),
        "label".to_string(),
        "roller".to_string(),
    ];
    println!("{:?}", common_chars(words));
}
