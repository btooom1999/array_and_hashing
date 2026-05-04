fn largest_odd_number(s: String) -> String {
    let mut s = s.into_bytes();
    while let Some(&last) = s.last() && (last-b'0').is_multiple_of(2) {
        s.pop();
    }

    String::from_utf8(s).unwrap()
}

pub fn main() {
    let s = "1112".to_string();
    println!("{}", largest_odd_number(s));
}
