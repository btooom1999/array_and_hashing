fn dfs(s: &[u8]) -> i32 {
    if s.is_empty() || (s.len() == 1 && !s[0].is_ascii_digit()) {
        return 0;
    }

    if let Some(i) = s.iter().position(|&v| v.is_ascii_alphabetic() || v == b'.') {
        return dfs(s[..i].trim_ascii());
    }

    for i in 1..s.len() {
        if s[i] == b'+' || s[i] == b'-' {
            return dfs(s[..i].trim_ascii());
        }
    }

    if let Some(i) = s.iter().position(|v| v.is_ascii_whitespace()) {
        return dfs(s[..i].trim_ascii());
    }

    if let Ok(num) = String::from_utf8(s.to_vec()).unwrap().parse::<i32>() {
        num
    } else if s[0] == b'-' {
        i32::MIN
    } else {
        i32::MAX
    }
}

fn my_atoi(s: String) -> i32 {
    dfs(s.trim().as_bytes())
}

pub fn main() {
    let s = "0-1".to_string();
    println!("{}", my_atoi(s));
}
