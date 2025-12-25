fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut hashes = [0; 26];
    for b in magazine.as_bytes() {
        hashes[(b - b'a') as usize] += 1;
    }

    for b in ransom_note.as_bytes() {
        let val = hashes.get_mut((b - b'a') as usize).unwrap();
        *val -= 1;

        if *val < 0 {
            return false;
        }
    }

    true
}

pub fn main() {
    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    println!("{}", can_construct(ransom_note, magazine));
}
