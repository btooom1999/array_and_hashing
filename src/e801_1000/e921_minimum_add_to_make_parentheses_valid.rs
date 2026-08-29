fn min_add_to_make_valid(s: String) -> i32 {
    let mut closing_parentheses = 0;
    let mut stack = Vec::new();

    for byte in s.as_bytes() {
        if *byte == b'(' {
            stack.push(*byte);
        } else if stack.is_empty() {
            closing_parentheses += 1;
        } else {
            stack.pop();
        }
    }

    closing_parentheses + stack.len() as i32
}

pub fn main() {
    let s = "())".to_string();
    println!("{}", min_add_to_make_valid(s));
}
