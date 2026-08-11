fn recursion(s: &[u8]) -> i32 {
    let mut open_parenthesis = 0;
    let mut at = usize::MAX;
    let mut res = 0;
    let mut num = 0;
    let mut operator = b'+';
    for i in 0..s.len() {
        let b = s[i];
        if b == b'(' {
            open_parenthesis += 1;
            if at == usize::MAX { at = i; }
        } else if b == b')' {
            open_parenthesis -= 1;
            if open_parenthesis == 0 {
                num = recursion(&s[at+1..i]);
                at = usize::MAX;
            }
        } else if b.is_ascii_digit() && open_parenthesis == 0 {
            num = num * 10 + (b - b'0') as i32;
        } else if open_parenthesis == 0 {
            if operator == b'+' {
                res += num;
            } else {
                res -= num;
            }
            num = 0;
            operator = b;
        }
    }

    if operator == b'+' {
        res + num
    } else {
        res - num
    }
}

fn calculate(s: String) -> i32 {
    let s = s.replace(" ", "");
    recursion(s.as_bytes())
}

pub fn main() {
    let s = "(1+(4+5+2)-3)+(6+8)".to_string();
    println!("{}", calculate(s));
}
