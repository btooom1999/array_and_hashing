fn longest_valid_parentheses(s: String) -> i32 {
    let n = s.len();
    let mut stack = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' || stack.last().is_none_or(|(_, is_valid)| !is_valid) {
            stack.push((i+1, c == '('));
        } else {
            stack.pop();
        }
    }

    if stack.is_empty() {
        return n as i32;
    }

    let mut res = stack.first().unwrap_or(&(1,true)).0-1;
    res = res.max(n - stack.last().unwrap_or(&(n,true)).0);
    for i in 1..stack.len() {
        res = res.max(stack[i].0-stack[i-1].0-1);
    }

    res as i32
}

pub fn main() {
    // let s = ")()())".to_string();
    let s = "()()(".to_string();
    println!("{}", longest_valid_parentheses(s));
}
