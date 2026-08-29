fn letter_case_permutation(s: String) -> Vec<String> {
    let alphas = s
        .chars()
        .enumerate()
        .filter(|v| v.1.is_ascii_alphabetic())
        .collect::<Vec<_>>();
    let n = alphas.len();

    let mut res = Vec::new();
    for mask in 0..1 << n {
        let mut s = s.clone().into_bytes();

        for j in 0..n {
            if mask >> j & 1 == 1 {
                s[alphas[j].0] = s[alphas[j].0].to_ascii_uppercase();
            } else {
                s[alphas[j].0] = s[alphas[j].0].to_ascii_lowercase();
            }
        }

        res.push(String::from_utf8(s).unwrap());
    }

    res
}

pub fn main() {
    let s = "a1b2".to_string();
    println!("{:?}", letter_case_permutation(s));
}
