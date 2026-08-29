fn largest_even(s: String) -> String {
    let mut s = s.into_bytes();
    while let Some(&last) = s.last() && last == b'1' {
        s.pop();
    }

    String::from_utf8(s).unwrap()
}

pub fn main() {
    let s = "1112".to_string();
    println!("{}", largest_even(s));
}
