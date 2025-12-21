fn append_characters(s: String, t: String) -> i32 {
    let t = t.chars().collect::<Vec<_>>();
    let mut i = 0;

    for char in s.chars() {
        if i >= t.len() { break; }

        if char == t[i] {
            i += 1;
        }
    }

    if i == t.len() { return 0 };
    t[i..].len() as i32
}

pub fn main() {
    let s = "z".to_string();
    let t = "abcde".to_string();

    println!("{}", append_characters(s, t));
}
