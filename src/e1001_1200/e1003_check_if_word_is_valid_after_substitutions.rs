fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == 'a' || c == 'b' {
            stack.push(c);
        } else if stack.len() < 2 {
            return false;
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            if b != 'b' || a != 'a' {
                return false;
            }
        }
    }

    stack.is_empty()
}

pub fn main() {
    let s = "abcabcababcc".to_string();
    println!("{}", is_valid(s));
}
