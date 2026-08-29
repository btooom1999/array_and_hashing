fn min_length(s: String) -> i32 {
    let mut stack = Vec::new();
    for num in s.as_bytes() {
        if stack.last().is_some_and(|v| (*v == b'A' && *num == b'B') || (*v == b'C' && *num == b'D')) {
            stack.pop();
        } else {
            stack.push(*num);
        }
    }

    stack.len() as i32
}

pub fn main() {
    let s = "ABFCACDB".to_string();
    println!("{}", min_length(s));
}
