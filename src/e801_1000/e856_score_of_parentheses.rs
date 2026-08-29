fn dfs(s: &[u8]) -> i32 {
    if s == [b'(', b')'] {
        return 1;
    }

    let mut i = 0;
    let mut sum = 0;
    let mut count = 0;
    for (j, &b) in s.iter().enumerate() {
        if b == b'(' {
            count += 1;
        } else {
            count -= 1;
        }

        if count == 0 {
            if i+1 == j {
                sum += 1;
            } else {
                sum += dfs(&s[i+1..j]) * 2;
            }
            i = j+1;
        }
    }

    sum
}

fn score_of_parentheses(s: String) -> i32 {
    dfs(s.as_bytes())
}

pub fn main() {
    let s = "(())".to_string();
    println!("{}", score_of_parentheses(s));
}
