use std::collections::VecDeque;

fn count_collisions(directions: String) -> i32 {
    let directions = directions.as_bytes();
    let mut r = directions.len() as i32 - 1;
    let mut stack = VecDeque::<u8>::new();
    let mut res = 0;
    while r > -1 {
        let byte = directions[r as usize];
        if byte == b'L' || byte == b'S' {
            stack.push_front(byte);
            r -= 1;
        } else {
            let mut execute = false;
            // remove 'S'
            if stack.front().is_some_and(|v| *v == b'S') {
                execute = true;
                stack.pop_front();
            }

            // remove 'L'
            while stack.front().is_some_and(|v| *v == b'L') {
                res += 1;
                execute = true;
                stack.pop_front();
            }

            // remove adjent or single 'R'
            while r > -1 && directions[r as usize] == b'R' {
                if execute {
                    res += 1;
                }
                r -= 1;
            }
        }
    }

    while stack.front().is_some_and(|v| *v != b'S') {
        stack.pop_front();
    }

    while stack.pop_front().is_some_and(|v| v == b'S') {
        while stack.front().is_some_and(|v| *v == b'L') {
            res += 1;
            stack.pop_front();
        }
    }

    res
}

pub fn main() {
    let directions = "RRR".to_string();
    println!("{}", count_collisions(directions));
}
