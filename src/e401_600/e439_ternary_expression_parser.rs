use std::collections::VecDeque;

fn parse_ternary(expression: String) -> String {
    let expression = expression.as_bytes();
    let mut stack = VecDeque::new();

    let mut i = expression.len() as i32 - 1;
    while i >= 0 {
        let byte = expression[i as usize];
        if byte != b'?' {
            stack.push_front(byte);
            i -= 1;
            continue;
        }

        let condition = expression[i as usize - 1];
        let mut former = VecDeque::<u8>::new();
        while stack.front().is_some_and(|v| *v != b':') {
            former.push_front(stack.pop_front().unwrap());
        }

        // remove ':'
        stack.pop_front();

        let mut latter = VecDeque::<u8>::new();
        while stack.front().is_some_and(|v| *v != b':') {
            latter.push_front(stack.pop_front().unwrap());
        }

        if condition == b'T' {
            stack = former.into_iter().chain(stack.into_iter()).collect::<VecDeque<_>>();
        } else {
            stack = latter.into_iter().chain(stack.into_iter()).collect::<VecDeque<_>>();
        }

        i -= 2;
    }

    String::from_utf8(stack.into()).unwrap()
}

pub fn main() {
    // let expression = "T?2:3".to_string();
    // let expression = "T?T?F:5:3".to_string();
    let expression = "F?1:T?4:5".to_string();
    println!("{}", parse_ternary(expression));
}
