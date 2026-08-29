fn remove_outer_parentheses(s: String) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut i = 0;

    for (j, c) in s.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }

        if count == 0 {
            #[allow(clippy::char_indices_as_byte_indices)]
            result.push_str(&s[i+1..j]);
            i = j+1;
        }
    }

    result
}

pub fn main() {
    let s = "(()())(())".to_string();
    println!("{}", remove_outer_parentheses(s));
}
