fn can_be_valid(s: String, locked: String) -> bool {
    let s = s.as_bytes();
    let locked = locked.as_bytes();
    let mut free = Vec::new();
    let mut parentheses = Vec::new();

    for i in 0..s.len() {
        if locked[i] == b'0' {
            free.push(i);
        } else if s[i] == b'(' {
            parentheses.push(i);
        } else if !parentheses.is_empty() {
            parentheses.pop();
        } else if !free.is_empty() {
            free.pop();
        } else {
            return false;
        }
    }

    while let Some(parenthesis) = parentheses.pop()  {
        if free.last().is_none_or(|v| *v < parenthesis) {
            return false;
        }

        free.pop();
    }

    free.len() % 2 == 0
}

pub fn main() {
    let s = "))()))".to_string();
    let locked = "010100".to_string();
    println!("{}", can_be_valid(s, locked));
}
