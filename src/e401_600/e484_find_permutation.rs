use std::collections::VecDeque;

fn find_permutation(s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let mut max = 1;
    let mut stack = VecDeque::from([max]);

    for (r, byte) in s.iter().enumerate() {
        max += 1;

        if *byte == b'D' {
            let mut temp_stack = VecDeque::new();
            let mut l = r as i32;
            while l >= 0 && s[l as usize] == b'D' {
                l -= 1;
                temp_stack.push_front(stack.pop_back().unwrap());
            }
            stack.push_back(max);
            stack.extend(temp_stack);
        } else {
            stack.push_back(max);
        }
    }

    stack.into()
}

pub fn main() {
    let s = "DIIIID".to_string();
    println!("{:?}", find_permutation(s));
}
