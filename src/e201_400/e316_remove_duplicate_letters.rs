fn dfs(
    s: &[u8],
    idx: usize,
    suffix: &[i32],
    bit: &mut i32,
    chars: &mut Vec<char>,
) -> String {
    if *bit == suffix[0] {
        return chars.iter().collect::<String>();
    }

    if *bit | suffix[idx] != suffix[0] {
        return String::new();
    }

    for i in 0..26 {
        if suffix[idx] & 1 << i != 0 && *bit & 1 << i == 0 {
            *bit ^= 1 << i;
            chars.push((i+b'a') as char);
            let nidx = s.iter().position(|&v| v == i+b'a').unwrap();
            let res = dfs(&s[nidx+1..], idx+nidx+1, suffix, bit, chars);
            if !res.is_empty() {
                return res;
            }
            chars.pop();
            *bit ^= 1 << i;
        }
    }

    String::new()
}

fn remove_duplicate_letters(s: String) -> String {
    let s = s.as_bytes();
    let n = s.len();
    let mut suffix = vec![0; n+1];
    for i in (0..n).rev() {
        suffix[i] = suffix[i+1];
        suffix[i] |= 1 << (s[i] - b'a');
    }

    dfs(s, 0, &suffix, &mut 0, &mut vec![])
}

pub fn main() {
    let s = "cbacdcbc".to_string();
    println!("{}", remove_duplicate_letters(s));
}
