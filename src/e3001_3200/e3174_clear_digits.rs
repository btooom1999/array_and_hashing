fn clear_digits(s: String) -> String {
    let mut stack = Vec::new();
    for num in s.as_bytes() {
        if num.is_ascii_digit() {
            stack.pop();
        } else {
            stack.push(*num);
        }
    }

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let s = "cb34".to_string();
    println!("{}", clear_digits(s));
}
