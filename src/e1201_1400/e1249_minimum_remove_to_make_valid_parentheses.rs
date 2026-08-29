fn min_remove_to_make_valid(s: String) -> String {
    let mut s = s.into_bytes();
    let (mut open_parentheses, mut close_parentheses) = (0,0);
    let n = s.len();
    for i in 0..n {
        if s[i] == b'(' {
            open_parentheses += 1;
        } else if s[i] == b')' {
            if open_parentheses == 0 {
                close_parentheses += 1;
            } else {
                open_parentheses -= 1;
            }
        }
    }

    for i in 0..n {
        if s[i] == b')' && close_parentheses > 0 {
            s[i] = b'*';
            close_parentheses -= 1;
        }
        if s[n-i-1] == b'(' && open_parentheses > 0 {
            s[n-i-1] = b'*';
            open_parentheses -= 1;
        }
    }

    let mut res = String::new();
    for i in 0..n {
        if s[i] != b'*' {
            res.push(s[i] as char);
        }
    }

    res
}

pub fn main() {
    let s = "lee(t(c)o)de)".to_string();
    println!("{}", min_remove_to_make_valid(s));
}
