fn first_uniq_char(s: String) -> i32 {
    let mut hashmap = [0; 26];
    for n in s.as_bytes() {
        hashmap[(n - b'a') as usize] += 1;
    }

    for (i, n) in s.char_indices() {
        if hashmap[(n as u8 - b'a') as usize] == 1 {
            return i as i32;
        }
    }

    -1
}

pub fn main() {
    let s = "leetcode".to_string();
    println!("{}", first_uniq_char(s));
}
