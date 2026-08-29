fn check_valid_string(s: String) -> bool {
    let mut stars = Vec::new();
    let mut parenthesis = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            parenthesis.push(i);
        } else if c == '*' {
            stars.push(i);
        } else if !parenthesis.is_empty() {
            parenthesis.pop();
        } else if !stars.is_empty() {
            stars.pop();
        } else {
            return false;
        }
    }

    while let Some(i) = parenthesis.pop() {
        if stars.last().is_none_or(|v| *v < i) {
            return false;
        }

        stars.pop();
    }

    true
}

pub fn main() {
    let s = "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".to_string();
    println!("{}", check_valid_string(s));
}
