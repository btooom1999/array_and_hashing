fn minimum_deletions(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut prefix_b = vec![0; n+1];
    let mut suffix_a = vec![0; n+2];

    for i in 0..n {
        prefix_b[i+1] = prefix_b[i];
        suffix_a[n-i] = suffix_a[n-i+1];
        if s[i] == b'b' {
            prefix_b[i+1] += 1;
        }
        if s[n-i-1] == b'a' {
            suffix_a[n-i] += 1;
        }
    }

    let mut res = i32::MAX;
    for i in 0..=n {
        res = res.min(prefix_b[i] + suffix_a[i+1]);
    }

    res
}

pub fn main() {
    // let s = "ababaaaabbbbbaaababbbbbbaaabbaababbabbbbaabbbbaabbabbabaabbbababaa".to_string();
    let s = "b".to_string();
    println!("{}", minimum_deletions(s));
}
