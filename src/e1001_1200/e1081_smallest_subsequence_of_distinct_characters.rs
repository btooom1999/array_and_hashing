fn smallest_subsequence(s: String) -> String {
    let mut lasts = [0; 256];
    let s = s.as_bytes();
    let n = s.len();
    for i in 0..n {
        lasts[s[i] as usize] = i;
    }

    let mut stack = Vec::new();
    let mut hashset = [false; 256];
    for i in 0..n {
        if hashset[s[i] as usize] {
            continue;
        }

        while let Some(&last) = stack.last() {
            if last > s[i] && lasts[last as usize] > i {
                stack.pop();
                hashset[last as usize] = false;
            } else {
                break;
            }
        }

        stack.push(s[i]);
        hashset[s[i] as usize] = true;
    }

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let s = "cbacdcbc".to_string();
    println!("{}", smallest_subsequence(s));
}
