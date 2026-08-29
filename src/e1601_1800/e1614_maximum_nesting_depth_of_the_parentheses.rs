fn max_depth(s: String) -> i32 {
    let mut res = 0;
    let mut stack = 0;
    for c in s.chars() {
        if c == '(' {
            stack += 1;
            res = res.max(stack);
        } else if c == ')' {
            stack -= 1;
        }
    }

    res
}

pub fn main() {
    let s = "(1+(2*3)+((8)/4))+1".to_string();
    println!("{}", max_depth(s));
}
