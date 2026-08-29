fn is_valid(s: String) -> bool {
    let open_parentheses = ['[', '(', '{'];
    let mut stack = Vec::new();
    for c in s.chars() {
        if open_parentheses.contains(&c) {
            stack.push(c);
            continue;
        }

        if let Some(val) = stack.pop() {
            match c {
                ')' => {
                    if val != '(' {
                        return false;
                    }
                }
                ']' => {
                    if val != '[' {
                        return false;
                    }
                }
                '}' => {
                    if val != '{' {
                        return false;
                    }
                },
                _ => unreachable!(),
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}

pub fn main() {
    let s = "()[]{}".to_string();
    println!("{}", is_valid(s));
}
