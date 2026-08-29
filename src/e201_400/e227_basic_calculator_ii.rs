use std::collections::VecDeque;

fn calculate(s: String) -> i32 {
    let s = s.as_bytes();
    let mut stack = VecDeque::<u8>::new();
    let mut j = 0;
    while j < s.len() {
        if s[j].is_ascii_whitespace() || ![b'*', b'/'].contains(&s[j]) {
            if !s[j].is_ascii_whitespace() {
                stack.push_back(s[j]);
            }
            j += 1;
            continue;
        }

        let mut prev_num = VecDeque::new();
        while stack.back().is_some_and(|v | v.is_ascii_digit()) {
            prev_num.push_front(stack.pop_back().unwrap());
        }

        let operation = s[j];
        j += 1;
        let mut next_num = VecDeque::new();
        while let Some(&byte) = s.get(j) {
            if byte.is_ascii_digit() || byte.is_ascii_whitespace() {
                if byte.is_ascii_digit() {
                    next_num.push_back(byte);
                }
                j += 1;
            } else {
                break;
            }
        }

        let prev_num = prev_num.iter().fold(0, |acc, v| acc * 10 + (v - b'0') as i32);
        let next_num = next_num.iter().fold(0, |acc, v| acc * 10 + (v - b'0') as i32);
        if operation == b'*' {
            stack.extend((prev_num * next_num).to_string().as_bytes());
        } else {
            stack.extend((prev_num / next_num).to_string().as_bytes());
        }
    }

    let mut res = 0;
    let mut bytes = VecDeque::new();
    let mut operation = b'+';
    stack.push_back(b'+');
    while let Some(byte) = stack.pop_front() {
        if byte.is_ascii_digit() {
            bytes.push_back(byte);
            continue;
        }

        let num = bytes.iter().fold(0, |acc, v| acc * 10 + (v - b'0') as i32);
        if operation == b'+' {
            res += num;
            bytes.clear();
        } else {
            res -= num;
            bytes.clear();
        }

        operation = byte;
    }

    res
}

pub fn main() {
    let s = "42".to_string();
    println!("{}", calculate(s));
}
