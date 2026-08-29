fn remove_duplicates(s: String) -> String {
    let mut stack = Vec::new();
    for byte in s.as_bytes() {
        if let Some(last) = stack.last() {
            if last == byte {
                stack.pop();
            } else {
                stack.push(*byte);
            }
        } else {
            stack.push(*byte);
        }
    }

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let s = "abbaca".to_string();
    println!("{}", remove_duplicates(s));
}
