fn repeated_character(s: String) -> char {
    let mut mask = 0;
    for &byte in s.as_bytes() {
        let pos = (byte - b'a') as u32;
        if mask & (1 << pos) != 0 {
            return byte as char;
        }

        mask |= 1 << pos;
    }

    unreachable!()
}

pub fn main() {
    let s = "abccbaacz".to_string();
    println!("{}", repeated_character(s));
}
