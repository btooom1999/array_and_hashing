fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut vec = [0; 26];
    for c in allowed.as_bytes() {
        vec[(c - b'a') as usize] = 1;
    }

    let mut count = 0;
    for c in &words {
        let mut valid = true;
        for c in c.as_bytes() {
            if vec[(c - b'a') as usize] == 0 {
                valid = false;
                break;
            }
        }
        if valid {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let allowed = "ab".to_string();
    let words = vec![
        "ad".to_string(),
        "bd".to_string(),
        "aaab".to_string(),
        "baa".to_string(),
        "badab".to_string(),
    ];
    println!("{}", count_consistent_strings(allowed, words));
}
