fn reverse_parentheses(s: String) -> String {
    let mut stack = Vec::new();
    for byte in s.as_bytes() {
        if *byte != b')' {
            stack.push(*byte);
            continue;
        }

        let mut temp_stack = Vec::new();
        while stack.last().is_some_and(|v| *v != b'(') {
            temp_stack.push(stack.pop().unwrap());
        }

        stack.pop();
        stack.extend(temp_stack);
    }

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let s = "(ed(et(oc))el)".to_string();
    println!("{}", reverse_parentheses(s));
}
