fn remove_kdigits(num: String,mut k: i32) -> String {
    let mut stack = Vec::<u8>::new();
    for byte in num.as_bytes() {
        while k > 0 && stack.last().is_some_and(|v| v > byte) {
            stack.pop();
            k -= 1;
        }
        if *byte != b'0' && !stack.is_empty() {
            continue;
        }
        stack.push(*byte);
    }

    while k > 0 {
        stack.pop();
        k -= 1;
    }

    if stack.is_empty() {
        stack.push(b'0');
    }

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let num = "112".to_string();
    let k = 1;
    println!("{}", remove_kdigits(num, k));
}
