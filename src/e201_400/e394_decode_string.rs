use std::collections::VecDeque;

fn decode_string(s: String) -> String {
    let mut stack = VecDeque::new();
    for byte in s.as_bytes() {
        if *byte != b']' {
            stack.push_back(*byte);
            continue;
        }

        let mut n = VecDeque::new();
        let mut chars = VecDeque::new();
        while stack.back().is_some_and(|v| *v != b'[') {
            chars.push_front(stack.pop_back().unwrap());
        }

        stack.pop_back();

        while stack.back().is_some_and(|v| v.is_ascii_digit()) {
            n.push_front(stack.pop_back().unwrap());
        }

        let n = String::from_utf8(n.into()).unwrap().parse::<usize>().unwrap();
        let chars = std::iter::repeat_n(chars, n).flatten().collect::<VecDeque<_>>();
        stack.extend(chars);
    }

    String::from_utf8(stack.into()).unwrap()
}

pub fn main() {
    let s = "3[a]2[bc]".to_string();
    println!("{}", decode_string(s));
}
